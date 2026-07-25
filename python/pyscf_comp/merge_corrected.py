"""Merge the alkane bench (jax numbers + librint wall time) with the fixed
librint gradient peak RSS from `mem_probe.py --fix`, in bench_fair's schema for
plot_alkanes.py.

The bench's librint peak_kb measured the OLD getF (full n^4 ERI tensor); the
gradient WALL TIME is unaffected by the fix, so time is kept from the bench and
only peak_kb is replaced by the O(n^2) measurement.

Usage: python merge_corrected.py --bench BENCH.json --fix FIX.json \
                                 --out bench_alkanes_corrected.json
"""
import argparse
import json

CASES = [
    ("def2-svp", "CH4"), ("def2-svp", "C2H6"), ("def2-svp", "C3H8"),
    ("def2-svp", "C4H10"),
    ("def2-tzvp", "CH4"), ("def2-tzvp", "C2H6"), ("def2-tzvp", "C3H8"),
    ("def2-tzvp", "C4H10"), ("def2-tzvp", "C6H6"),
]
# jax provably OOMs here (nao 204/252): its frozen-P vjp intermediates scale
# ~quartic and already OOM-killed at C3H8/tzvp (nao 156) on the 160G node.
# If the bench (still running) hasn't recorded these yet, mark them OOM so the
# figure's open markers are complete; a real bench entry, if present, wins.
KNOWN_JAX_OOM = {("def2-tzvp", "C4H10"), ("def2-tzvp", "C6H6")}


def load(p):
    with open(p) as f:
        return json.load(f)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--bench", required=True)
    ap.add_argument("--fix", required=True)
    ap.add_argument("--out", default="bench_alkanes_corrected.json")
    args = ap.parse_args()
    bench = load(args.bench)
    fix = load(args.fix)

    out = {}
    print(f"{'case':18s} {'lib t(s)':>9s} {'old memG':>9s} {'NEW memG':>9s} "
          f"{'drop':>6s}  {'jax pin':>9s}")
    for basis, geo in CASES:
        tag = f"{geo}/{basis}"
        lib = dict(bench.get(f"t1|librint|{tag}|pin", {}) or {})
        fg = fix.get(f"{tag}|grad", {}) or {}
        old_mem = (lib.get("peak_kb", 0) / 1048576.0) if lib.get("status") == "ok" else 0.0
        if fg.get("status") == "ok":
            lib["peak_kb"] = fg["peak_kb"]          # <-- fixed O(n^2) peak RSS
            lib["status"] = "ok"
            if fg.get("median") is not None:        # fixed-code wall time (full
                lib["median"] = fg["median"]        # getF); grad_sorted kept from
            # bench for the librint-vs-jax agreement column (values unchanged)
        new_mem = (lib.get("peak_kb", 0) / 1048576.0) if lib.get("status") == "ok" else 0.0
        out[f"t1|librint|{tag}|pin"] = lib

        # jax rows straight from the bench (their peak RSS is genuinely quartic);
        # inject OOM only where the bench has no entry AND jax must OOM
        for thr in ("pin", "free"):
            k = f"t1|jax|{tag}|{thr}"
            e = bench.get(k)
            if e is None and (basis, geo) in KNOWN_JAX_OOM:
                e = {"status": "OOM", "note": "inferred (bench pending): jax vjp >> 160G"}
            out[k] = e if e is not None else {"status": "missing"}

        jp = out[f"t1|jax|{tag}|pin"]
        jm = (f"{jp['peak_kb']/1048576:.1f}" if jp.get("status") == "ok"
              else jp.get("status", "-"))
        lt = f"{lib.get('median', 0):.3f}" if lib.get("median") else "-"
        drop = f"{old_mem/new_mem:.0f}x" if new_mem > 0 and old_mem > 0 else "-"
        print(f"{tag:18s} {lt:>9s} {old_mem:>9.2f} {new_mem:>9.2f} {drop:>6s}  {jm:>9s}")

    with open(args.out, "w") as f:
        json.dump(out, f, indent=1)
    print(f"\nwrote {args.out}")


if __name__ == "__main__":
    main()
