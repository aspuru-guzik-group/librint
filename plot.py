import os
import matplotlib.pyplot as plt
import numpy as np
from collections import defaultdict

# Where the output files are
EXP_DIR = "exp"

# Data structure: {(mol, bas): {variant: time}}
times = defaultdict(dict)

# Time unit conversion functions
def convert_to_seconds(time_str):
    """Converts time string to seconds (handles ms, µs, and ns)."""
    time_str = time_str.strip().lower()
    
    if "ms" in time_str:
        return float(time_str.replace("ms", "").strip()) / 1000  # milliseconds to seconds
    elif "µs" in time_str:
        return float(time_str.replace("µs", "").strip()) / 1_000_000  # microseconds to seconds
    elif "ns" in time_str:
        return float(time_str.replace("ns", "").strip()) / 1_000_000_000  # nanoseconds to seconds
    else:
        raise ValueError(f"Unknown time unit in {time_str}")

# Read all .txt files in exp/
for filename in os.listdir(EXP_DIR):
    if not filename.endswith(".txt"):
        continue

    try:
        base = filename.removesuffix(".txt")
        mol, bas, variant = base.split("_")
        filepath = os.path.join(EXP_DIR, filename)

        # Read the last line containing the time
        with open(filepath) as f:
            lines = f.readlines()
        for line in reversed(lines):
            if any(unit in line for unit in ["ms", "µs", "ns"]):
                time_str = line.strip()
                time_seconds = convert_to_seconds(time_str)
                times[(mol, bas)][variant] = time_seconds
                break
    except Exception as e:
        print(f"Skipping {filename}: {e}")

# Create sorted list of (mol, bas) pairs
mol_bas_pairs = sorted(times.keys(), key=lambda x: (x[0], x[1]))

# Extract data for plotting
variants = ["none", "cache", "opt", "all"]
labels = [f"{mol.upper()} {bas.replace('sto3g', 'sto-3g').replace('631g', '6-31g')}" for mol, bas in mol_bas_pairs]
x = np.arange(len(labels))
width = 0.2

# Plot setup
fig, ax = plt.subplots(figsize=(16, 6))
colors = ["red", "orange", "blue", "green"]

for i, variant in enumerate(variants):
    bar_values = [times.get((mol, bas), {}).get(variant, 0) for mol, bas in mol_bas_pairs]
    ax.bar(x + (i - 1.5) * width, bar_values, width, label=variant, color=colors[i])

# Labeling and formatting
ax.set_xlabel("Molecule and Basis Set")
ax.set_ylabel("Time (seconds)")
ax.set_title("Integrator Runtime Comparison")
ax.set_xticks(x)
ax.set_xticklabels(labels, rotation=45, ha="right")
ax.legend()

ax.set_yscale('log')

plt.tight_layout()
plt.savefig("runtime_comparison.png")
plt.show()
