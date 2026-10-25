"""Paper 27 and docs/152-153: the reading as a wave.

1. f27_mirror.png  - the schematic: four-wave mixing beside a relation read as a wave.
2. f27_open.png    - the open stratum, every reader on one axis, frequency as a reference.
3. f27_back.png    - the relation read backwards, told and never-told pairs.

Numbers are read from data/runs/waves/*.md (the corrected readings, docs/152)
and data/runs/mixed_prior/*.txt. Palette as figs150.py (validated: all checks
pass on #fcfcfb; the aqua is below 3:1, so every bar carries its label)."""
import os, re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyArrowPatch, Rectangle, Circle

HERE = os.path.dirname(os.path.abspath(__file__))
RUNS = os.path.join(HERE, "..", "..", "data", "runs")
INK, INK2, GRID, SURF = "#0b0b0b", "#52514e", "#e6e5e1", "#fcfcfb"
ORANGE, BLUE, AQUA, PRIOR = "#eb6834", "#2a78d6", "#1baf7a", "#a3a29c"
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10, "axes.edgecolor": "#8a8984", "axes.labelcolor": INK2, "xtick.color": INK2, "ytick.color": INK2})
CELL = r"([\d.]+)% · ([\d.]+)"


def rows(path, name):
    """Every row called `name` in a measurement file, as lists of (top-1, MRR)."""
    text = open(os.path.join(RUNS, path)).read()
    out = []
    for line in text.splitlines():
        if line.startswith(f"| {name} |"):
            out.append([(float(a), float(b)) for a, b in re.findall(CELL, line)])
    return out


def arrow(ax, a, b, col, text=None, at=0.5, dy=0.0, style="-|>", lw=1.8, ls="-"):
    ax.add_patch(FancyArrowPatch(a, b, arrowstyle=style, mutation_scale=14, color=col, lw=lw, linestyle=ls))
    if text:
        ax.text(a[0] + (b[0] - a[0]) * at, a[1] + (b[1] - a[1]) * at + dy, text, ha="center", va="center", fontsize=8.6, color=INK)


def fig_mirror():
    fig, axs = plt.subplots(1, 2, figsize=(12.4, 4.6))
    fig.patch.set_facecolor(SURF)
    for ax in axs:
        ax.set_facecolor(SURF); ax.set_xlim(0, 10); ax.set_ylim(0, 6); ax.axis("off"); ax.set_aspect("equal")
    a = axs[0]
    a.set_title("a. Four-wave mixing: the phase-conjugate wave", loc="left", fontsize=10.2, color=INK)
    a.add_patch(Rectangle((3.8, 1.6), 2.4, 2.8, fc="#efeeea", ec="#8a8984", lw=1))
    a.text(5.0, 1.3, "nonlinear medium", ha="center", fontsize=8.2, color=INK2)
    arrow(a, (0.4, 3.0), (3.8, 3.0), BLUE, "pump 1", dy=0.28)
    arrow(a, (9.6, 3.0), (6.2, 3.0), BLUE, "pump 2", dy=0.28)
    arrow(a, (0.9, 5.7), (3.9, 4.25), ORANGE)
    a.text(2.75, 5.45, "probe  E", fontsize=8.6, color=INK)
    arrow(a, (3.8, 4.0), (0.6, 5.2), AQUA, ls="--")
    a.text(0.4, 4.15, "conjugate  E*", fontsize=8.6, color=INK)
    a.text(5.0, 0.45, r"$E_c \propto$ pump$_1$ · pump$_2$ · $E^*$ : it retraces the probe's path, time reversed", ha="center", fontsize=8.4, color=INK2)
    b = axs[1]
    b.set_title("b. A relation read as a wave on the circle", loc="left", fontsize=10.2, color=INK)
    tips = {1.2: (0.45, 0.5, ORANGE), 5.0: (0.15, 0.66, ORANGE), 8.8: (0.2, 0.62, AQUA)}
    for x, lab in [(1.2, r"$a_w$"), (5.0, r"$H\,a_w$"), (8.8, r"$a_v$")]:
        b.add_patch(Circle((x, 3.2), 0.7, fc="none", ec="#8a8984", lw=1))
        dx, dy, col = tips[x]
        b.add_patch(FancyArrowPatch((x, 3.2), (x + dx, 3.2 + dy), arrowstyle="-|>", mutation_scale=10, color=col, lw=1.6))
        b.text(x, 2.15, lab, ha="center", fontsize=10, color=INK)
    arrow(b, (2.05, 3.55), (4.15, 3.55), BLUE, r"$H = e^{i\theta_k}$", dy=0.38)
    arrow(b, (5.85, 3.55), (7.95, 3.55), INK2, r"Re$\langle H a_w, a_v\rangle$", dy=0.38)
    arrow(b, (7.95, 2.85), (5.85, 2.85), AQUA, ls="--")
    arrow(b, (4.15, 2.85), (2.05, 2.85), AQUA, ls="--")
    b.text(5.0, 4.75, "the kind: an all-pass filter, one phase per mode     the score: a matched filter", ha="center", fontsize=8.0, color=INK2)
    b.text(5.0, 1.45, r"dashed: $\bar H = H^{-1}$, the relation read backwards — never trained", ha="center", fontsize=8.4, color=INK)
    b.text(5.0, 0.45, "amplitude (how often told) is the counts', the pump;  phase (which relation) is the operator's", ha="center", fontsize=8.2, color=INK2)
    fig.tight_layout()
    fig.savefig(os.path.join(HERE, "fig", "f27_mirror.png"), dpi=170, bbox_inches="tight", facecolor=SURF)


def bars(ax, items, ref, title, top):
    ax.set_facecolor(SURF)
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)
    ax.grid(axis="x", color=GRID, lw=0.8); ax.set_axisbelow(True)
    names = [i[0] for i in items][::-1]
    vals = [i[1] for i in items][::-1]
    cols = [i[2] for i in items][::-1]
    ax.barh(range(len(vals)), vals, height=0.62, color=cols, edgecolor=SURF, linewidth=2)
    for y, v in enumerate(vals):
        ax.text(v + top * 0.01, y, f"{v:.1f}%", va="center", fontsize=8.4, color=INK)
    ax.set_yticks(range(len(vals))); ax.set_yticklabels(names, fontsize=8.6)
    if ref is not None:
        ax.axvline(ref, color=PRIOR, lw=1.6, ls=(0, (4, 3)))
        ax.text(ref + top * 0.012, -0.55, f"frequency {ref:.1f}%", fontsize=8.2, color=INK, ha="left", va="center")
        ax.set_ylim(-0.85, len(vals) - 0.5)
    ax.set_xlim(0, top); ax.set_xlabel("top-1 on held-out Emma (%)", fontsize=8.6)
    ax.set_title(title, loc="left", fontsize=10.2, color=INK)


def fig_open():
    fno = rows("waves/fno_dev10_epoch32.md", "never told after either")
    mir = rows("waves/mirror_dev10_epoch128.md", "never told after either")
    counts, freq = fno[0][0][0], fno[0][3][0]
    items = [
        ("the counts (flat)", counts, AQUA),
        ("the mirror, last word", mir[0][0][0], ORANGE),
        ("the FNO, last word", fno[1][0][0], BLUE),
        ("the mirror + the prior, mixed", mir[0][5][0], ORANGE),
        ("the FNO + the prior, mixed", fno[1][3][0], BLUE),
    ]
    fig, ax = plt.subplots(figsize=(8.4, 3.4))
    fig.patch.set_facecolor(SURF)
    bars(ax, items, freq, "Never told after the last word or pair (2,431 positions, 23%)", 4.5)
    fig.tight_layout()
    fig.savefig(os.path.join(HERE, "fig", "f27_open.png"), dpi=170, bbox_inches="tight", facecolor=SURF)


def fig_back():
    fig, axs = plt.subplots(1, 2, figsize=(12.4, 3.0))
    fig.patch.set_facecolor(SURF)
    for ax, name, title, top in [(axs[0], "the pair was told", "The word before, where the pair was told (8,236)", 20), (axs[1], "the pair was never told", "Where it never was (2,443)", 2)]:
        fno, mir = rows("waves/fno_dev10_epoch32.md", name)[0], rows("waves/mirror_dev10_epoch128.md", name)[0]
        items = [
            ("FNO, run from the other end", fno[0][0], BLUE),
            (r"mirror, the conjugate wave $\bar H$", mir[0][0], ORANGE),
            ("mirror, amplitude only", mir[1][0], ORANGE),
        ]
        bars(ax, items, fno[2][0], title, top)
    fig.tight_layout()
    fig.savefig(os.path.join(HERE, "fig", "f27_back.png"), dpi=170, bbox_inches="tight", facecolor=SURF)


fig_mirror()
fig_open()
fig_back()
print("f27_mirror, f27_open, f27_back")
