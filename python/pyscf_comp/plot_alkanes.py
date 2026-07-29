"""Paper figure: alkane-ladder scaling of the T1 basis-parameter gradient.

Reads bench_alkanes_results.json (bench_fair.py --suite alkanes) and draws a
2x2 log-log grid: rows = wall time / peak memory, columns = def2-SVP /
def2-TZVP, x = number of cartesian basis functions. Runs that died of memory
appear as open markers pinned at the job memory limit.

Every plotted point comes from the results JSON, and both engines' points must
come from the SAME run of that suite -- do not splice timings from one job with
peak RSS from another. Core counts and the memory ceiling come from the run's
own `_meta` block for the same reason: a legend that says "32 cores" about a
96-core run is the same defect as a spliced point.

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

SYSTEMS = ["CH4", "C2H6", "C3H8", "C4H10"]
BASES = ["def2-svp", "def2-tzvp"]
# statuses that mean "ran out of memory" -- kernel kill or an allocation the
# process refused up front. Anything else that failed (timeout, crash) is NOT
# drawn as an OOM; it gets its own marker so a bug can't hide as a memory wall.
MEM_FAIL = ("OOM", "OOM_ALLOC")
MEM_MSG = ("ArrayMemoryError", "Unable to allocate", "std::bad_alloc")


def classify(r):
    """-> 'ok' | 'mem' | 'other' | None. Older results predate the OOM_ALLOC
    status, so a FAIL is re-read against its own captured stderr."""
    s = r.get("status")
    if s is None:
        return None
    if s == "ok":
        return "ok"
    if s in MEM_FAIL:
        return "mem"
    tail = r.get("stderr_tail") or ""
    return "mem" if any(m in tail for m in MEM_MSG) else "other"

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

SERIES = [  # key-parts, style; legend labels are built from the run's own data
    (("librint", "pin"), dict(color="#1a7f37", marker="o", ls="-")),
    (("jax", "pin"), dict(color="#cf222e", marker="s", ls="--")),
    (("jax", "free"), dict(color="#e16f24", marker="^", ls=":")),
]


def load(path):
    with open(path) as f:
        return json.load(f)


def entry(results, eng, geo, basis, threads):
    return results.get(f"t1|{eng}|{geo}/{basis}|{threads}") or {}


def cores_of(results, eng, threads):
    """Cores this series actually ran on, from the workers' own measurement."""
    n = {r["ncores"] for k, r in results.items()
         if k.startswith(f"t1|{eng}|") and k.endswith(f"|{threads}")
         and isinstance(r, dict) and r.get("ncores")}
    if len(n) == 1:
        return n.pop()
    if n:
        return None                                    # inconsistent -> say so
    meta = results.get("_meta") or {}                   # pre-`ncores` results
    return 1 if threads != "free" else meta.get("ncores")


def label_of(results, eng, threads):
    c = cores_of(results, eng, threads)
    return f"{eng} ({c} core{'s' if c != 1 else ''})" if c else f"{eng} (? cores)"


def mem_limit_g(results):
    kb = (results.get("_meta") or {}).get("mem_limit_kb")
    return kb / 1048576.0 if kb and kb > 0 else None


def collect(results, basis, eng, threads, with_benzene):
    """-> (nbf, time, mem) for ok points, (nbf,) out-of-memory, (nbf,) other."""
    xs, ts, ms, oom, bad = [], [], [], [], []
    geos = SYSTEMS + (["C6H6"] if basis == "def2-tzvp" and with_benzene else [])
    for geo in geos:
        # Every point on this figure is a measurement from the results JSON.
        # A missing entry is plotted as nothing, not as an assumed OOM: an
        # earlier revision injected inferred OOM markers for C4H10 and C6H6
        # that no run had produced.
        r = entry(results, eng, geo, basis, threads)
        n = NBF[(geo, basis)]
        c = classify(r)
        if c == "ok":
            xs.append(n)
            ts.append(r["median"])
            ms.append(r["peak_kb"] / 1048576.0)
        elif c == "mem":
            oom.append(n)
        elif c == "other":
            bad.append(n)
    return (np.array(xs), np.array(ts), np.array(ms),
            np.array(oom), np.array(bad))


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

    limit_g = mem_limit_g(results)
    labels = {(eng, thr): label_of(results, eng, thr)
              for (eng, thr), _ in SERIES}

    fig, axes = plt.subplots(2, 2, figsize=(7.0, 5.4), sharex="col")
    for j, basis in enumerate(BASES):
        ax_t, ax_m = axes[0][j], axes[1][j]
        any_oom = False
        for k, ((eng, thr), st) in enumerate(SERIES):
            xs, ts, ms, oom, bad = collect(results, basis, eng, thr, with_bz)
            label = labels[(eng, thr)]
            if xs.size:
                ax_t.plot(xs, ts, label=label, ms=5, **st)
                ax_m.plot(xs, ms, label=label, ms=5, **st)
            dodge = 1.0 + 0.03 * k     # keep overlapping engines readable
            if oom.size and limit_g:   # open marker pinned at the ceiling
                any_oom = True
                ax_m.plot(oom * dodge, [limit_g] * oom.size,
                          marker=st["marker"], color=st["color"], ls="none",
                          mfc="none", ms=7)
            if bad.size:               # crashed for some OTHER reason
                ax_m.plot(bad * dodge, [limit_g or 1.0] * bad.size, marker="x",
                          color=st["color"], ls="none", ms=7, mew=1.4)
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
            ax_m.axhline(limit_g, color="0.5", lw=0.8, ls="--")
            # low-left: the only corner both engines' curves stay out of
            ax_m.annotate(f"job limit {limit_g:.0f}G (open = out of memory)",
                          (0.02, 0.22), xycoords="axes fraction",
                          fontsize=7, color="0.35", va="top")
        ax_m.set_xlabel("cartesian basis functions")
    axes[0][0].set_ylabel("wall time per gradient (s)")
    axes[1][0].set_ylabel("peak RSS (GiB)")
    # time N^4 guide under the librint TZVP curve (its gradient IS ~quartic);
    # memory N^4 guide on the jax curve (librint memory is ~flat by design)
    xs, ts, ms, _, _ = collect(results, "def2-tzvp", "librint", "pin", with_bz)
    if xs.size >= 3:
        guide(axes[0][1], xs[1], ts[1] * 0.25, 4)
    xj, tj, mj, _, _ = collect(results, "def2-tzvp", "jax", "pin", with_bz)
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
    meta = results.get("_meta") or {}
    if meta:
        print(f"\nrun: {meta.get('node', '?')}  {meta.get('ncores', '?')} cores"
              f"  {(meta.get('mem_limit_kb') or 0) / 1048576:.0f}G"
              f"  {meta.get('cpu_model', '')}")
    hdr = [labels[("librint", "pin")], labels[("jax", "pin")],
           labels[("jax", "free")]]
    print(f"\n{'system':18s} {hdr[0]:>16s} {hdr[1]:>16s} "
          f"{hdr[2]:>17s} {'lib mem':>8s} {'jax mem':>8s} {'max|dg|':>9s}")
    for basis in BASES:
        for geo in SYSTEMS + (["C6H6"] if basis == "def2-tzvp" and with_bz
                              else []):
            rl = entry(results, "librint", geo, basis, "pin")
            rj = entry(results, "jax", geo, basis, "pin")
            rf = entry(results, "jax", geo, basis, "free")

            def t(r):
                if r.get("status") == "ok":
                    return f"{r['median']:.3f}"
                c = classify(r)
                if c is None:
                    return "-"
                # spell out WHY it has no number, using the same rule the
                # figure uses to place its markers
                return "out-of-mem" if c == "mem" else r.get("status", "-")

            def m(r):
                return (f"{r['peak_kb'] / 1048576:.1f}G"
                        if r.get("status") == "ok" else "-")

            dg = "-"
            if rl.get("status") == "ok" and rj.get("status") == "ok":
                a = np.array(rl["grad_sorted"])
                b = np.array(rj["grad_sorted"])
                dg = (f"{np.abs(a - b).max():.1e}" if a.shape == b.shape
                      else "len!")
            print(f"{geo + '/' + basis:18s} {t(rl):>16s} {t(rj):>16s} "
                  f"{t(rf):>17s} {m(rl):>8s} {m(rj):>8s} {dg:>9s}")


if __name__ == "__main__":
    main()
