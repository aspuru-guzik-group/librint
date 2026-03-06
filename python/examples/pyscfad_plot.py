import matplotlib.pyplot as plt
import numpy as np

# Molecule and basis set names
molecules = [
    "H2 sto-2g", "H2 sto-3g", "LIH sto-2g", "LIH sto-3g", 
    "HF sto-2g", "HF sto-3g", "H2O sto-2g", "H2O sto-3g", 
    "CH4 sto-2g", "CH4 sto-3g", "NH3 sto-2g", "NH3 sto-3g"
]

# Time data (seconds per run)
librpyscf_times = np.array([
    0.002629, 0.017973, 0.073447, 0.394320, 
    0.073813, 0.393081, 0.186336, 1.017388, 
    0.802495, 5.812259, 0.400552, 2.446966
])
jax_times = np.array([
    0.276372, 0.300839, 0.508663, 0.507960, 
    0.445162, 0.439033, 0.482200, 0.480149, 
    0.435037, 0.449251, 0.570611, 0.568460
])

# Plotting
x = np.arange(len(molecules))  # X-axis positions
width = 0.35  # Width of the bars

fig, ax = plt.subplots(figsize=(10, 6))

# Bar plots
bars1 = ax.bar(x - width / 2, librpyscf_times, width, label='librpyscf.denergy', color='blue')
bars2 = ax.bar(x + width / 2, jax_times, width, label='jax.value_and_grad', color='orange')

# Formatting
ax.set_xlabel("Molecules and Basis Sets")
ax.set_ylabel("Time (seconds per run)")
ax.set_title("Comparison of librpyscf.denergy vs jax.value_and_grad")
ax.set_xticks(x)
ax.set_xticklabels(molecules, rotation=45, ha='right')
ax.legend()

plt.tight_layout()
plt.show()
plt.savefig("pyscfad_plot.pdf")
