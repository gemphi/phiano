"""docs/150: the operator's long run, read from data/runs/operator_r64_s7/log.md.
Two panels, one per stratum, each on its own scale (no dual axes); the
operator is the line, the other readers are flat references labelled
directly. Palette and ink as in figs25v.py."""
import os, re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
LOG = os.path.join(HERE, "..", "..", "data", "runs", "operator_r64_s7", "log.md")
INK, INK2, GRID, SURF = "#0b0b0b", "#52514e", "#e6e5e1", "#fcfcfb"
ORANGE, BLUE, AQUA, PRIOR = "#eb6834", "#2a78d6", "#1baf7a", "#a3a29c"
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10, "axes.edgecolor": "#8a8984", "axes.labelcolor": INK2, "xtick.color": INK2, "ytick.color": INK2})

rows = re.findall(r"## Epoch (\d+) · hinge loss ([\d.]+).*?told after the last word or pair \| \d+ \| ([\d.]+)% · ([\d.]+) \| ([\d.]+)% · ([\d.]+) \| ([\d.]+)% · ([\d.]+) \| ([\d.]+)% · ([\d.]+) \|.*?never told after either \| \d+ \| ([\d.]+)% · ([\d.]+) \| ([\d.]+)% · ([\d.]+) \| ([\d.]+)% · ([\d.]+) \| ([\d.]+)% · ([\d.]+) \|", open(LOG).read(), re.S)
ep = [int(r[0]) for r in rows]
told = {k: [float(r[i]) for r in rows] for k, i in [("counts", 2), ("operator", 4), ("kin", 6), ("prior", 8)]}
open_ = {k: [float(r[i]) for r in rows] for k, i in [("counts", 10), ("operator", 12), ("kin", 14), ("prior", 16)]}

fig, axs = plt.subplots(1, 2, figsize=(12.4, 4.4))
fig.patch.set_facecolor(SURF)
for ax, data, title, top in [(axs[0], told, "told after the last word or pair (77% of positions)", 30), (axs[1], open_, "never told after either (23% of positions)", 4.5)]:
    ax.set_facecolor(SURF)
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)
    ax.grid(axis="y", color=GRID, lw=0.8); ax.set_axisbelow(True)
    for k, col, name in [("counts", BLUE, "counts"), ("kin", AQUA, "kin of the last word"), ("prior", PRIOR, "frequency")]:
        v = data[k][-1]
        ax.axhline(v, color=col, lw=1.6, ls=(0, (4, 3)))
        ax.text(97.5, v, f"{name} {v:.1f}%", va="center", fontsize=8.4, color=INK)
    ax.plot(ep, data["operator"], color=ORANGE, lw=2, marker="o", ms=5, mec=SURF, mew=1.5)
    ax.text(ep[-1], data["operator"][-1] + top * 0.04, f"operator {data['operator'][-1]:.1f}%", ha="right", fontsize=8.6, color=INK)
    ax.set_xlim(0, 96); ax.set_ylim(0, top)
    ax.set_xlabel("epochs trained", fontsize=8.6); ax.set_ylabel("top-1 on held-out Emma (%)", fontsize=8.6)
    ax.set_title(title, fontsize=10.2, loc="left", color=INK)
fig.text(0.01, -0.03, "operator_run, R = 64, seed 7: phinet's Joint FNO on 185,209 told `word next word` rows; measured every 8 epochs on 10,578 held-out positions (docs/150).", fontsize=7.8, color=INK2)
fig.tight_layout()
fig.savefig(os.path.join(HERE, "fig", "f150_operator_run.png"), dpi=170, bbox_inches="tight", facecolor=SURF)
print(len(ep), "checkpoints")
