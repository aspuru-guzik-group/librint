"""Paper figure: thread scaling of the basis-parameter gradient.

Reads bench_scaling_results.json (bench_par_scaling.py) and
bench_breakdown_results.json (bench_grad_breakdown.py) and draws three panels:

  1. speedup vs threads, against the ideal diagonal, with the Amdahl ceiling
     that applied when only dS and dR were threaded drawn as a dashed line per
     system. The point of the figure is that the curves cross those lines.
  2. parallel efficiency vs threads.
  3. where the serial time goes, as a stacked bar -- which is *why* panel 1
     needed the Fock build threaded and not just the 2e reverse.

Same rule as plot_alkanes.py: every plotted point comes from a results JSON,
both files must come from the same job, and the machine is read from their
_meta blocks rather than assumed.

Usage:  python plot_scaling.py [--scaling bench_scaling_results.json]
                              [--breakdown bench_breakdown_results.json]
                              [--out gradient_scaling]
Outputs <out>.pdf + <out>.png and a text summary table.
"""
import argparse
import json

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import FixedLocator, FixedFormatter, NullLocator
import numpy as np

# term -> (legend label, colour). Order is the stacking order in panel 3.
TERMS = [
    ("dR", "dR  2e reverse", "#1a7f37"),
    ("getF", "getF  primal Fock build", "#e16f24"),
    ("dHcore", "dHcore  dT+dV", "#8250df"),
    ("dS", "dS  overlap", "#0969da"),
]
# the two that had no parallel version before this work; panel 3 hatches them
WAS_SERIAL = {"getF", "dHcore"}
STYLES = [
    dict(color="#1a7f37", marker="o", ls="-"),
    dict(color="#0969da", marker="s", ls="--"),
    dict(color="#e16f24", marker="^", ls="-."),
    dict(color="#cf222e", marker="D", ls=":"),
]


def load(path):
    with open(path) as f:
        return json.load(f)


def systems(d):
    return [k for k in d if k != "_meta"]


def thread_curve(row):
    """-> (threads, speedup vs serial, speedup vs par@1) sorted by thread count."""
    ts = sorted(int(t) for t in row["threads"])
    t1 = row["threads"][str(ts[0])]["median"]
    med = np.array([row["threads"][str(t)]["median"] for t in ts])
    return np.array(ts), row["serial"] / med, t1 / med


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--scaling", default="bench_scaling_results.json")
    ap.add_argument("--breakdown", default="bench_breakdown_results.json")
    ap.add_argument("--out", default="gradient_scaling")
    args = ap.parse_args()

    sc = load(args.scaling)
    bd = load(args.breakdown)
    meta = sc.get("_meta") or {}
    syss = systems(sc)

    # extra width on the right for the breakdown legend, which lives outside
    fig, axes = plt.subplots(1, 3, figsize=(13.2, 3.9),
                             gridspec_kw={"width_ratios": [1, 1, 1.05]})
    ax_s, ax_e, ax_b = axes

    # ── panel 1: speedup, with the pre-fix Amdahl ceilings ──────────────────
    tmax = 1
    for i, tag in enumerate(syss):
        th, sp, _ = thread_curve(sc[tag])
        tmax = max(tmax, int(th.max()))
        ax_s.plot(th, sp, ms=4, label=tag, **STYLES[i % len(STYLES)])
        ceil = (bd.get(tag) or {}).get("old_ceiling")
        if ceil:
            ax_s.axhline(ceil, color=STYLES[i % len(STYLES)]["color"],
                         lw=0.7, ls=(0, (1, 2)), alpha=0.8)
    ideal = np.array([1, tmax])
    ax_s.plot(ideal, ideal, color="0.6", lw=1, zorder=1)
    ax_s.annotate("ideal", (tmax, tmax), fontsize=8, color="0.4",
                  ha="right", va="bottom")
    ax_s.annotate("dotted: ceiling with only dS+dR threaded",
                  (0.03, 0.97), xycoords="axes fraction", fontsize=7,
                  color="0.35", va="top")
    ax_s.set_xscale("log", base=2)
    ax_s.set_yscale("log", base=2)
    ax_s.set_xlabel("threads")
    ax_s.set_ylabel(r"speedup vs serial $\mathtt{danalyticalf}$")
    ax_s.set_title("end-to-end gradient speedup", fontsize=10)
    ax_s.legend(fontsize=7, loc="lower right", framealpha=0.9)

    # ── panel 2: efficiency ─────────────────────────────────────────────────
    for i, tag in enumerate(syss):
        th, _, sp1 = thread_curve(sc[tag])
        ax_e.plot(th, sp1 / th, ms=4, label=tag, **STYLES[i % len(STYLES)])
    ax_e.axhline(1.0, color="0.6", lw=1, zorder=1)
    ax_e.set_xscale("log", base=2)
    ax_e.set_ylim(0, 1.15)
    ax_e.set_xlabel("threads")
    ax_e.set_ylabel(r"efficiency  (speedup vs par@1) / threads")
    ax_e.set_title("parallel efficiency", fontsize=10)

    # ── panel 3: where the serial time goes ─────────────────────────────────
    bsys = [t for t in syss if t in bd]
    x = np.arange(len(bsys))
    bottom = np.zeros(len(bsys))
    for key, label, colour in TERMS:
        frac = np.array([100.0 * bd[t][key] / bd[t]["accounted"] for t in bsys])
        ax_b.bar(x, frac, 0.6, bottom=bottom, color=colour, label=label,
                 hatch="//" if key in WAS_SERIAL else None,
                 edgecolor="white", lw=0.4)
        for xi, (f, b) in enumerate(zip(frac, bottom)):
            if f >= 4.0:
                ax_b.text(xi, b + f / 2, f"{f:.0f}%", ha="center", va="center",
                          fontsize=7, color="white", fontweight="bold")
        bottom += frac
    ax_b.set_xticks(x)
    ax_b.set_xticklabels([t.replace("/", "\n") for t in bsys], fontsize=7)
    ax_b.set_ylabel("share of 1-core gradient time (%)")
    ax_b.set_title("cost breakdown (hatched = was serial)", fontsize=10)
    # outside the axes: the bars fill 0-100% by construction, so any in-axes
    # legend covers data -- it was hiding the dR labels on the first two bars
    ax_b.legend(fontsize=7, loc="upper left", bbox_to_anchor=(1.01, 1.0),
                framealpha=0.9, borderaxespad=0)
    ax_b.set_ylim(0, 100)

    for ax in (ax_s, ax_e):
        ax.grid(True, which="both", lw=0.3, alpha=0.4)
        ax.xaxis.set_major_locator(FixedLocator([1, 2, 4, 8, 16, 32, 64]))
        ax.xaxis.set_major_formatter(FixedFormatter(
            ["1", "2", "4", "8", "16", "32", "64"]))
        ax.xaxis.set_minor_locator(NullLocator())

    sub = (f"{meta.get('ncores', '?')} cores, {meta.get('cpu_model', '')}"
           f"  |  median of {meta.get('repeats', '?')}")
    fig.suptitle("Basis-parameter gradient of frozen-P HF energy: thread "
                 f"scaling\n{sub}", fontsize=10)
    fig.tight_layout(rect=(0, 0, 1, 0.90))
    for ext in ("pdf", "png"):
        fig.savefig(f"{args.out}.{ext}", dpi=300)
    print(f"wrote {args.out}.pdf/.png")

    # ── text summary ────────────────────────────────────────────────────────
    if meta:
        print(f"\nrun: {meta.get('node', '?')}  {meta.get('ncores', '?')} cores"
              f"  {meta.get('cpu_model', '')}")
    print(f"\n{'system':18s} {'nao':>4s} {'serial':>9s} {'best':>9s} "
          f"{'threads':>7s} {'speedup':>8s} {'eff':>5s} {'old cap':>8s}")
    for tag in syss:
        th, sp, sp1 = thread_curve(sc[tag])
        k = int(np.argmax(sp))
        cap = (bd.get(tag) or {}).get("old_ceiling")
        print(f"{tag:18s} {sc[tag]['nao']:4d} {sc[tag]['serial']:9.3f} "
              f"{sc[tag]['threads'][str(th[k])]['median']:9.3f} {th[k]:7d} "
              f"{sp[k]:7.1f}x {sp1[k] / th[k]:5.2f} "
              f"{(f'{cap:6.1f}x' if cap else '-'):>8s}")


if __name__ == "__main__":
    main()
