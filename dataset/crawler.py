import json
import math
import re
from typing import Dict, List, Tuple
from urllib.parse import urljoin

import lxml
import requests
from lxml.etree import _Element, tostring


class GithubSession:
    def __init__(self, use_proxy: bool=True):
        self.proxies = {
            "http": "http://127.0.0.1:7890",
            "https": "http://127.0.0.1:7890",
        } if use_proxy else None
        self.session = requests.session()
    
    def get(self, url: str, params: Dict) -> requests.Response:
        resp = self.session.get(url=url, params=params, proxies=self.proxies)
        return resp
    
    def get_and_parse(self, url: str, params: Dict) -> _Element:
        html = self.get(url=url, params=params).text
        html = lxml.etree.HTML(html)
        return html

    def close(self):
        self.session.close()

LINKS_PER_PAGE = 25

def inner_text(ele: _Element):
    return tostring(ele, method="text", encoding="unicode").strip()

def urlappend(url: str, sub: str) -> str:
    sub = sub.strip("/")
    if url.endswith("/"):
        return url + sub
    else:
        return url + "/" + sub

def parse_repos(filename: str, encoding="utf-8") -> List[Tuple[str, str]]:
    with open(filename, "r", encoding=encoding) as f:
        rust = f.read()
    results = re.findall(r"\[(.+?)\]\((.+?)\)", rust, re.S)[1:]
    return results

def parse_PR_numbers(html: _Element) -> Tuple[int, int]:
    result = html.xpath("//div[@class='table-list-header-toggle states flex-auto pl-0']")[0]
    num, _ = inner_text(result).split()
    assert(_ == "Total")
    num = int(num.replace(",", ""))
    return num

def parse_PR_links(html: _Element) -> List[str]:
    result = html.xpath("//a[@class='Link--primary v-align-middle no-underline h4 js-navigation-open markdown-title']")
    hrefs = map(lambda ele: ele.xpath("./@href")[0], result)
    return hrefs

def parse_PR_titles(html: _Element) -> List[str]:
    result = html.xpath("//a[@class='Link--primary v-align-middle no-underline h4 js-navigation-open markdown-title']")
    titles = map(inner_text, result)
    return titles

def parse_PR_changed_files_number(html: _Element) -> int:
    result = html.xpath("//a[@class='ActionList-content hx_ActionList-content']")
    num = len(result)
    if num == 0:
        num = 1
    return num

def crawl_one_repo(name: str, repo_url: str, query:str, session: GithubSession) -> dict:
    print(f"Crawling {name}...", end="", flush=True)
    global proxies
    url = urlappend(repo_url, "pulls")
    params = {"q": query, "page": 1}
    html = session.get_and_parse(url=url, params=params)
    num = parse_PR_numbers(html)
    prs = []
    for page in range(1, math.ceil(num / LINKS_PER_PAGE) + 1):
        params["page"] = page
        page_html = session.get_and_parse(url=url, params=params)
        hrefs = parse_PR_links(html=page_html)
        hrefs = list(map(lambda href: urljoin(repo_url, href), hrefs))
        titles = parse_PR_titles(html=page_html)
        changed_files_numbers = [parse_PR_changed_files_number(html=session.get_and_parse(url=urlappend(pr_href, "files"), params=None)) for pr_href in hrefs]
        parse_results = zip(hrefs, titles, changed_files_numbers)
        prs.extend(map(lambda parse_result: {"href": parse_result[0], "title": parse_result[1], "files": parse_result[2]}, parse_results))
    assert(num == len(prs))
    print("Done.")
    repo_info = {
        "name": name,
        "url": repo_url,
        "num": num,
        "prs": prs
    }
    return repo_info

if __name__ == "__main__":
    repos = parse_repos(filename="Rust.md")
    session = GithubSession(use_proxy=True)
    info = []
    query = "is:pr is:merged parallel in:title,comment"
    for name, repo_url in repos:
        repo_info = crawl_one_repo(name=name, repo_url=repo_url, query=query, session=session)
        info.append(repo_info)
    session.close()
    with open("key_parallel.json", "w") as f:
        json.dump(info, f)
