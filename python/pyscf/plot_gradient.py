import os
import numpy as np
import matplotlib.pyplot as plt

import os
import numpy as np

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

        parts = filename.split('_')
        if len(parts) != 2:
            print("Skipping malformed filename {}".format(filename))
            continue

        mol, bas = parts
        data.setdefault(mol, {})
        data[mol].setdefault(bas, {})

        with open(file_path, 'r') as f:
            lines = [line.strip() for line in f if line.strip()]

        i = 0
        while i < len(lines):
            line = lines[i]
            if line.endswith(':'):
                key = line[:-1].strip()
                i += 1
                array_lines = []

                # Accumulate array lines until next key or end of file
                while i < len(lines) and not lines[i].endswith(':'):
                    array_lines.append(lines[i])
                    i += 1

                array_str = ' '.join(array_lines).strip('[]')
                try:
                    arr = np.fromstring(array_str, sep=' ')
                    data[mol][bas][key] = arr
                except Exception as e:
                    print(f"Error parsing array for {mol}_{bas} key {key}: {e}")
            else:
                i += 1

    return data


def plot_comparison_by_index(save, jax, analytical, denergy, grad):
    """
    Plots the values of jax, analytical, denergy, and grad arrays indexed by position.
    """
    x = np.arange(len(jax))
    
    plt.figure(figsize=(14, 6))

    plt.plot(x, jax, label="jax", marker="o")
    plt.plot(x, analytical, label="analytical", marker="s")
    plt.plot(x, denergy, label="denergy", marker="^")
    # plt.plot(x, grad, label="grad", marker="x")
    
    plt.title("Gradient Comparison for " + save)
    plt.xlabel("Index")
    plt.ylabel("Gradient")
    plt.legend()
    plt.grid(True)
    plt.tight_layout()
    plt.savefig("plots/" + save + ".png")


if __name__ == "__main__":
    d = build_dict()
    import pprint
    pprint.pprint(d)

    for molecule in d:
        for basis in d[molecule]:
            plot_comparison_by_index(molecule + "_" + basis,
                                d[molecule][basis]["jax"],
                                d[molecule][basis]["analytical"],
                                d[molecule][basis]["denergy"],
                                d[molecule][basis]["grad"])
