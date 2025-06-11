import os
import matplotlib.pyplot as plt
import numpy as np


def parse_time_line(line):
    key, value = line.strip().split(':')
    return float(value)

def build_dict():
    base_dir = "./grad"
    data = {}

    if not os.path.isdir(base_dir):
        print("Base directory {} not found.".format(base_dir))
        return data

    for filename in os.listdir(base_dir):
        file_path = os.path.join(base_dir, filename)

        if not os.path.isfile(file_path):
            continue

        # Split on underscore: e.g., "LIH_sto-3g"  ["LIH", "sto-3g"]
        parts = filename.split('_')
        if len(parts) != 2:
            print("Skipping malformed filename {}".format(filename))
            continue

        mol, bas = parts
        data.setdefault(mol, {})
        data[mol].setdefault(bas, {})

        with open(file_path, 'r') as f:
            lines = f.readlines()

        for line in lines:
            if ':' not in line:
                continue
            key, value = line.strip().split(':')
            data[mol][bas][key.strip()] = float(value.strip())

    return data

def plot_bar(data, metrics):
    """
    Plots timing bars for selected metrics per (molecule, basis) pair.

    Parameters:
    - data: dict of the form data[mol][basis][metric] = time
    - metrics: list of metric names to plot, e.g., ['jax', 'analytical']
    """

    # Collect all (mol, basis) pairs
    pairs = []
    for mol, basis_dict in data.items():
        for bas in basis_dict:
            pairs.append((mol, bas))
    pairs = sorted(pairs)

    x = np.arange(len(pairs))  # label positions
    width = 0.8 / len(metrics)  # divide total bar width among metrics

    fig, ax = plt.subplots(figsize=(14, 6))

    for i, metric in enumerate(metrics):
        heights = []
        for mol, bas in pairs:
            val = data.get(mol, {}).get(bas, {}).get(metric, 0)
            heights.append(val)

        # Shift each group slightly to avoid overlap
        offset = -0.4 + i * width + width / 2
        rects = ax.bar(x + offset, heights, width, label=metric)

        # Optionally annotate bars
        for rect in rects:
            height = rect.get_height()
            if height > 0:
                ax.annotate(f'{height:.2f}',
                            xy=(rect.get_x() + rect.get_width() / 2, height),
                            xytext=(0, 3),
                            textcoords="offset points",
                            ha='center', va='bottom', fontsize=8)

    labels = [f"{mol}_{bas}" for mol, bas in pairs]
    ax.set_ylabel('Time (s)')
    ax.set_title('Timing comparison by molecule and basis set')
    ax.set_xticks(x)
    ax.set_xticklabels(labels, rotation=45, ha='right')
    ax.set_yscale('log')
    ax.legend()
    plt.tight_layout()
    plt.savefig("timing_comparison.png")

def plot_speedup_ratio(data, metric1, metric2):
    """
    Plots the ratio (metric2 / metric1) for each molecule/basis pair in the data.

    Example: plot_speedup_ratio(data, 'analytical', 'jax') will plot jax / analytical.
    """
    pairs = []

    # Collect all (mol, bas) pairs
    for mol, basis_dict in data.items():
        for bas in basis_dict:
            pairs.append((mol, bas))
    pairs = sorted(pairs)

    ratios = []
    labels = []

    for mol, bas in pairs:
        val1 = data.get(mol, {}).get(bas, {}).get(metric1)
        val2 = data.get(mol, {}).get(bas, {}).get(metric2)

        if val1 is None or val2 is None or val1 == 0:
            print(f"Skipping {mol}_{bas} due to missing or zero '{metric1}' or '{metric2}' value")
            continue

        ratio = val2 / val1
        ratios.append(ratio)
        labels.append(f"{mol}_{bas}")

    x = np.arange(len(ratios))
    width = 0.6

    fig, ax = plt.subplots(figsize=(12, 6))
    bars = ax.bar(x, ratios, width, color='salmon')

    # Reference line at 1.0
    ax.axhline(1.0, color='gray', linestyle='--')

    ax.set_ylabel(f'{metric2} / {metric1} (Speedup Ratio)')
    ax.set_title(f'Speedup: {metric2} vs {metric1}')
    ax.set_xticks(x)
    ax.set_xticklabels(labels, rotation=45, ha='right')
    ax.set_yscale('log')

    # Annotate ratios on top
    for bar in bars:
        height = bar.get_height()
        ax.annotate(f"{height:.2f}", xy=(bar.get_x() + bar.get_width() / 2, height),
                    xytext=(0, 3), textcoords="offset points", ha='center', va='bottom', fontsize=8)

    plt.tight_layout()
    plt.savefig(f"plots/speedup_{metric2}_vs_{metric1}.png")


if __name__ == "__main__":
    d = build_dict()
    import pprint
    pprint.pprint(d)

    metrics = ["jax", "analytical", "denergy", "grad"]
    plot_bar(d, metrics)
    
    plot_speedup_ratio(d, "jax", "analytical")
    plot_speedup_ratio(d, "jax", "denergy")
    plot_speedup_ratio(d, "jax", "grad")

    plot_speedup_ratio(d, "analytical", "denergy")
    plot_speedup_ratio(d, "denergy", "grad")
    plot_speedup_ratio(d, "grad", "analytical")
    
    # plot_speedup_ratio(d, 'nuc')
    # plot_speedup_ratio(d, 'rep')
