import json
from typing import Dict, List

import pandas as pd


def to_excel(repos: List[Dict], filename:str="repo.xlsx"):
    data = []
    for repo in repos:
        prs = repo.pop("prs")
        for pr in prs:
            pr.update(repo)
            pr["href"] = f"=HYPERLINK(\"{pr['href']}\")"
        data.extend(prs)
    df = pd.DataFrame(data, columns=["name", "href", "title"])
    with pd.ExcelWriter(filename) as writer:
        df.to_excel(writer, sheet_name='Sheet1', index=False)

def to_json(repos: List[Dict], filename: str="repo.json"):
    with open(filename, "w") as f:
        json.dump(repos, f, indent=4)

def sum_up(repos: List[Dict]) -> int:
    return sum(map(lambda repo: repo["num"], repos))

def filter_by_files(repos: List[Dict], expr: str) -> List[Dict]: #expr是一个形如 1<x<2 的表达式字符串
    for repo in repos:
        repo["prs"] = list(filter(lambda _: eval(expr.replace("x", "(_['files'])")) , repo["prs"]))
        repo["num"] = len(repo["prs"])
    return repos

def filter_by_keyword(repos: List[Dict], keyword: str) -> List[Dict]: #筛选出含有keywords中`所有内容`的条目
    for repo in repos:
        repo["prs"] = list(filter(lambda _: keyword in _["title"], repo["prs"]))
        repo["num"] = len(repo["prs"])
    return repos

if __name__ == "__main__":
    filename = "key_optimize_or_optimization_1.json"
    with open(filename, "r") as f:
        repos = json.load(f)
    repos = filter_by_files(repos=repos, expr="x<=1")
    