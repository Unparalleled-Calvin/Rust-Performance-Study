import matplotlib.pyplot as plt
import numpy as np
import json
import re
from typing import List, Tuple, Dict

def parse_time(time: str) -> Tuple[int, int]:
    ns, e = re.findall(r"(.+?) ns/iter \(\+/- (.+?)\)", time)[0]
    return int(ns.replace(",", "_")), int(e.replace(",", "_"))

def extract_data(results: List[Dict], key: str, err=True):
    if err:
        data, err = zip(*map(lambda result: parse_time(result[key]), results))
        data = np.array(list(data))
        err = np.array([list(err), (err)])
        return data, err
    else:
        data = map(lambda result: result[key], results)
        data = np.array(list(data))
        return data

def plot_error_bars(x: np.ndarray, ys: List[np.ndarray], yerrs: List[np.ndarray], labels: List[str], xlabel: str, ylabel: str, loc: str="upper left", legendsize: int=15, ticksize: int=15, labelsize: int=15, colors: List[str]=None, linestyles: List[str]=None, from0: bool=True, yreverse: bool=False, appendix:str=None, filename=None):
    if colors is None:
        colors = ["b", "r", "g", "k"]
    if linestyles is None:
        linestyle_dict = {"b": "-", "r": "--", "g": "-.", "k": ":"}
        linestyles = [linestyle_dict[color] for color in colors]
    fig, ax = plt.subplots(figsize=(6, 5))
    for y, yerr, label, linestyle, color in zip(ys, yerrs, labels, linestyles, colors):
        ax.errorbar(x, y, yerr=yerr, label=label, linestyle=linestyle, color=color, capsize=3)
    lines, labels = ax.get_legend_handles_labels()
    ax.legend(lines, labels, loc=loc, fontsize=legendsize)
    ax.set_xlabel(xlabel, fontsize=labelsize)
    ax.set_ylabel(ylabel, fontsize=labelsize)
    ax.tick_params(axis='both', labelsize=ticksize)

    if appendix is not None: #额外的要求
        exec(appendix)

    if from0:
        ax.set_ylim(0)
    if yreverse:
        ax.set_ylim(ax.get_ylim()[::-1])
    fig.tight_layout()
    if filename:
        fig.savefig(filename)

def plot_bounds_checking():
    with open("results/data/bounds_checking.json") as f:
        results = json.load(f)
    x = extract_data(results=results, key="num", err=False)
    f, ferr = extract_data(results=results, key="f")
    o1, o1err = extract_data(results=results, key="o1")
    o2, o2err = extract_data(results=results, key="o2")

    ys = [f, o1, o2]
    yerrs = [ferr, o1err, o2err]
    labels = ["Vec::push", "Vec::extend", "Vec::ptr"]
    xlabel = "number of i32s"
    ylabel = "time consumption (ns)"
    filename = "results/fig/bounds_checking.jpg"
    plot_error_bars(x=x, ys=ys, yerrs=yerrs, labels=labels, xlabel=xlabel, ylabel=ylabel, filename=filename)

def plot_encoding_checking():
    with open("results/data/encoding_checking.json") as f:
        results = json.load(f)
    x = extract_data(results=results, key="num", err=False)
    f1, f1err = extract_data(results=results, key="f1")
    f2, f2err = extract_data(results=results, key="f2")
    o1, o1err = extract_data(results=results, key="o1")

    ys = [f1, f2, o1]
    yerrs = [f1err, f2err, o1err]
    labels = ["str::from_utf8(one-byte)", "str::from_utf8(three-byte)", "str::from_utf8_unchecked"]
    colors = ["g", "b", "r"]
    xlabel = "number of bytes"
    ylabel = "time consumption (ns)"
    filename = "results/fig/encoding_checking.jpg"
    plot_error_bars(x=x, ys=ys, yerrs=yerrs, labels=labels, xlabel=xlabel, ylabel=ylabel, legendsize=15, colors=colors, filename=filename)

def plot_suboptimal_data_collection():
    with open("results/data/suboptimal_data_collection.json") as f:
        results = json.load(f)
    x = extract_data(results=results, key="num", err=False)
    f1, f1err = extract_data(results=results, key="f1")
    f2, f2err = extract_data(results=results, key="f2")
    f3, f3err = extract_data(results=results, key="f3")
    f4, f4err = extract_data(results=results, key="f4")

    ys = [f1, f2, f3, f4]
    yerrs = [f1err, f2err, f3err, f4err]
    labels = ["HashMap", "BTreeMap", "FxHashMap", "AHashMap"]
    xlabel = "number of keys"
    ylabel = "time consumption (ns)"
    colors = ["k", "b", "g", "r"]
    filename = "results/fig/suboptimal_data_collection.jpg"
    plot_error_bars(x=x, ys=ys, yerrs=yerrs, labels=labels, xlabel=xlabel, ylabel=ylabel, legendsize=15, colors=colors, filename=filename)

def plot_data_collection_reallocation():
    with open("results/data/data_collection_reallocation.json") as f:
        results = json.load(f)
    x = extract_data(results=results, key="num", err=False)
    f1, f1err = extract_data(results=results, key="f1")
    f2, f2err = extract_data(results=results, key="f2")
    o1, o1err = extract_data(results=results, key="o1")
    o2, o2err = extract_data(results=results, key="o2")

    # ys = [f1, o1]
    # yerrs = [f1err, o1err]
    # labels = ["Vec::push", "Vec::push with reservation"]
    # xlabel = "number of elements"
    # ylabel = "time consumption (ns)"
    # filename = "results/fig/data_collection_reallocation_1.jpg"
    # plot_error_bars(x=x, ys=ys, yerrs=yerrs, labels=labels, xlabel=xlabel, ylabel=ylabel, legendsize=15, filename=filename)

    ys = [f2, o2]
    yerrs = [f2err, o2err]
    labels = ["HashSet::insert", "HashSet::insert with reservation"]
    xlabel = "number of i32s"
    ylabel = "time consumption (ns)"
    filename = "results/fig/data_collection_reallocation.jpg"
    plot_error_bars(x=x, ys=ys, yerrs=yerrs, labels=labels, xlabel=xlabel, ylabel=ylabel, legendsize=14, filename=filename)

def plot_allocating_then_flatten():
    with open("results/data/allocating_then_flatten.json") as f:
        results = json.load(f)
    x = extract_data(results=results, key="num", err=False)
    f, ferr = extract_data(results=results, key="f")
    o, oerr = extract_data(results=results, key="o")

    ys = [f, o]
    yerrs = [ferr, oerr]
    labels = ["flatten", "Vec::extend"]
    xlabel = "number of inner Vecs"
    ylabel = "time consumption (ns)"
    filename = "results/fig/allocating_then_flatten.jpg"
    plot_error_bars(x=x, ys=ys, yerrs=yerrs, labels=labels, xlabel=xlabel, ylabel=ylabel, appendix="ax.set_xscale('log')",  filename=filename)

def plot_incorrect_iter_type():
    with open("results/data/incorrect_iterator_type.json") as f:
        results = json.load(f)
    x = extract_data(results=results, key="num", err=False)
    f, ferr = extract_data(results=results, key="f")
    o, oerr = extract_data(results=results, key="o")

    ys = [f, o]
    yerrs = [ferr, oerr]
    labels = ["Iter", "IntoIter"]
    xlabel = "number of Strings"
    ylabel = "time consumption (ns)"
    filename = "results/fig/incorrect_iterator_type.jpg"
    plot_error_bars(x=x, ys=ys, yerrs=yerrs, labels=labels, xlabel=xlabel, ylabel=ylabel, filename=filename)
