"""Paper figure: alkane-ladder scaling of the T1 basis-parameter gradient.

Reads bench_alkanes_results.json (bench_fair.py --suite alkanes) and draws a
2x2 log-log grid: rows = wall time / peak memory, columns = def2-SVP /
def2-TZVP, x = number of cartesian basis functions. jax OOM entries appear as
open markers pinned at the job memory limit. The C6H6/def2-TZVP point from the
dedicated one-off (job 30056475) is folded into the TZVP column.

Usage:  python plot_alkanes.py [--json bench_alkanes_results.json]
                               [--out alkane_scaling] [--no-benzene]
Outputs <out>.pdf + <out>.png and a text summary table.
"""
import argparse
import json

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import FixedLocator, FixedFormatter, NullLocator
import numpy as np

MEM_LIMIT_G = 160          # SLURM --mem of the alkane job (OOM ceiling)

SYSTEMS = ["CH4", "C2H6", "C3H8", "C4H10"]
BASES = ["def2-svp", "def2-tzvp"]

# cartesian basis-function counts (C svp [3s2p1d]=15, H svp [2s1p]=5;
# C tzvp [5s3p2d1f]=36, H tzvp [3s1p]=6)
NBF = {
    ("CH4", "def2-svp"): 35,   ("CH4", "def2-tzvp"): 60,
    ("C2H6", "def2-svp"): 60,  ("C2H6", "def2-tzvp"): 108,
    ("C3H8", "def2-svp"): 85,  ("C3H8", "def2-tzvp"): 156,
    ("C4H10", "def2-svp"): 110, ("C4H10", "def2-tzvp"): 204,
    ("C6H6", "def2-tzvp"): 252,
}
CLABEL = {"CH4": "C1", "C2H6": "C2", "C3H8": "C3", "C4H10": "C4", "C6H6": "C6"}

# benzene fallback (used only if the results JSON lacks a C6H6 entry): the
# fixed getF's flat peak (0.095G), not the old ~30G n^4-ERI-tensor peak. jax
# OOMs here (its frozen-P vjp needs ~1TB).
BENZENE = {
    "geo": "C6H6", "basis": "def2-tzvp",
    "librint": {"median": 746.366, "peak_kb": 99576},
    "jax_oom": True,
}

SERIES = [  # key-parts, legend label, style
    (("librint", "pin"), "librint (1 core)",
     dict(color="#1a7f37", marker="o", ls="-")),
    (("jax", "pin"), "jax (1 core)",
     dict(color="#cf222e", marker="s", ls="--")),
    (("jax", "free"), "jax (32 cores)",
     dict(color="#e16f24", marker="^", ls=":")),
]


def load(path):
    with open(path) as f:
        return json.load(f)


def entry(results, eng, geo, basis, threads):
    return results.get(f"t1|{eng}|{geo}/{basis}|{threads}") or {}


def collect(results, basis, eng, threads, with_benzene):
    """-> (nbf, time, mem) arrays for ok points + (nbf_oom,) for OOM/limit."""
    xs, ts, ms, oom = [], [], [], []
    geos = SYSTEMS + (["C6H6"] if basis == "def2-tzvp" and with_benzene else [])
    for geo in geos:
        r = entry(results, eng, geo, basis, threads)
        if not r and geo == "C6H6":  # fallback: pre-isolation one-off numbers
            if eng == "librint":
                r = dict(BENZENE["librint"], status="ok")
            elif BENZENE["jax_oom"]:
                r = {"status": "OOM"}
        n = NBF[(geo, basis)]
        if r.get("status") == "ok":
            xs.append(n)
            ts.append(r["median"])
            ms.append(r["peak_kb"] / 1048576.0)
        elif r.get("status") in ("OOM", "TIMEOUT") or str(
                r.get("status", "")).startswith("FAIL"):
            oom.append(n)
    return np.array(xs), np.array(ts), np.array(ms), np.array(oom)


def guide(ax, x0, y0, power, span=1.6, **kw):
    x = np.array([x0, x0 * span])
    ax.plot(x, y0 * (x / x0) ** power, color="0.6", lw=1, zorder=1, **kw)
    ax.annotate(rf"$\propto N^{power}$", (x[-1], y0 * span ** power),
                fontsize=8, color="0.4", ha="left", va="center")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--json", default="bench_alkanes_results.json")
    ap.add_argument("--out", default="alkane_scaling")
    ap.add_argument("--no-benzene", action="store_true")
    args = ap.parse_args()
    results = load(args.json)
    with_bz = not args.no_benzene

    fig, axes = plt.subplots(2, 2, figsize=(7.0, 5.4), sharex="col")
    for j, basis in enumerate(BASES):
        ax_t, ax_m = axes[0][j], axes[1][j]
        any_oom = False
        for k, ((eng, thr), label, st) in enumerate(SERIES):
            xs, ts, ms, oom = collect(results, basis, eng, thr, with_bz)
            if xs.size:
                ax_t.plot(xs, ts, label=label, ms=5, **st)
                ax_m.plot(xs, ms, label=label, ms=5, **st)
            if oom.size:  # open marker pinned at the memory ceiling
                any_oom = True
                dodge = 1.0 + 0.03 * k  # keep overlapping engines readable
                ax_m.plot(oom * dodge, [MEM_LIMIT_G] * oom.size,
                          marker=st["marker"], color=st["color"], ls="none",
                          mfc="none", ms=7)
        ax_t.set_title(basis.replace("svp", "SVP").replace("tzvp", "TZVP"),
                       fontsize=10)
        for ax in (ax_t, ax_m):
            ax.set_xscale("log")
            ax.set_yscale("log")
            ax.grid(True, which="both", lw=0.3, alpha=0.4)
        # carbon-count labels along the top of each column
        geos = SYSTEMS + (["C6H6"] if basis == "def2-tzvp" and with_bz else [])
        top = ax_t.secondary_xaxis("top")
        top.xaxis.set_major_locator(FixedLocator([NBF[(g, basis)]
                                                  for g in geos]))
        top.xaxis.set_major_formatter(FixedFormatter([CLABEL[g]
                                                      for g in geos]))
        top.xaxis.set_minor_locator(NullLocator())
        top.tick_params(length=0, labelsize=8)
        if any_oom:
            ax_m.axhline(MEM_LIMIT_G, color="0.5", lw=0.8, ls="--")
            ax_m.annotate(f"job limit {MEM_LIMIT_G}G (open = OOM/fail)",
                          (0.02, 0.95), xycoords="axes fraction",
                          fontsize=7, color="0.35", va="top")
        ax_m.set_xlabel("cartesian basis functions")
    axes[0][0].set_ylabel("wall time per gradient (s)")
    axes[1][0].set_ylabel("peak RSS (GiB)")
    # time N^4 guide under the librint TZVP curve (its gradient IS ~quartic);
    # memory N^4 guide on the jax curve (librint memory is ~flat by design)
    xs, ts, ms, _ = collect(results, "def2-tzvp", "librint", "pin", with_bz)
    if xs.size >= 3:
        guide(axes[0][1], xs[1], ts[1] * 0.25, 4)
    xj, tj, mj, _ = collect(results, "def2-tzvp", "jax", "pin", with_bz)
    if mj.size >= 2:
        guide(axes[1][1], xj[0], mj[0] * 0.5, 4)
    axes[0][0].legend(fontsize=8, loc="upper left", framealpha=0.9)
    fig.suptitle("Basis-parameter gradient of frozen-P HF energy "
                 r"(T1): $\mathrm{C}_n\mathrm{H}_{2n+2}$ ladder", fontsize=11)
    fig.tight_layout(rect=(0, 0, 1, 0.97))
    for ext in ("pdf", "png"):
        fig.savefig(f"{args.out}.{ext}", dpi=300)
    print(f"wrote {args.out}.pdf/.png")

    # text summary + gradient agreement
    print(f"\n{'system':18s} {'librint@1c':>12s} {'jax@1c':>12s} "
          f"{'jax@32c':>12s} {'lib mem':>8s} {'jax mem':>8s} {'max|dg|':>9s}")
    for basis in BASES:
        for geo in SYSTEMS:
            rl = entry(results, "librint", geo, basis, "pin")
            rj = entry(results, "jax", geo, basis, "pin")
            rf = entry(results, "jax", geo, basis, "free")

            def t(r):
                return (f"{r['median']:.3f}" if r.get("status") == "ok"
                        else r.get("status", "-"))

            def m(r):
                return (f"{r['peak_kb'] / 1048576:.1f}G"
                        if r.get("status") == "ok" else "-")

            dg = "-"
            if rl.get("status") == "ok" and rj.get("status") == "ok":
                a = np.array(rl["grad_sorted"])
                b = np.array(rj["grad_sorted"])
                dg = (f"{np.abs(a - b).max():.1e}" if a.shape == b.shape
                      else "len!")
            print(f"{geo + '/' + basis:18s} {t(rl):>12s} {t(rj):>12s} "
                  f"{t(rf):>12s} {m(rl):>8s} {m(rj):>8s} {dg:>9s}")


if __name__ == "__main__":
    main()
