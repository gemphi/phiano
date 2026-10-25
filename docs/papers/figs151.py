"""docs/151: the operator trained on offsets, against the operator trained on
the next word alone (docs/150). Two panels, one per stratum, each on its own
scale; lines are the readings, frequency a flat labelled reference. Read from
the two runs' log.md (and measured/epoch_{4,8}.md for the offsets readings of
the checkpoints logged before ahead.rs existed). Palette as in figs150.py."""
import os, re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
RUNS = os.path.join(HERE, "..", "..", "data", "runs")
INK, INK2, GRID, SURF = "#0b0b0b", "#52514e", "#e6e5e1", "#fcfcfb"
ORANGE, BLUE, PRIOR, VIOLET = "#eb6834", "#2a78d6", "#a3a29c", "#7a4fd0"
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10, "axes.edgecolor": "#8a8984", "axes.labelcolor": INK2, "xtick.color": INK2, "ytick.color": INK2})
CELL = r"([\d.]+)% · [\d.]+"


def strata(text):
    """(told, open) top-1 of the operator's `last · next` reading and frequency."""
    t = re.search(r"told after the last word or pair \| \d+ \| " + " \\| ".join([CELL] * 4), text)
    o = re.search(r"never told after either \| \d+ \| " + " \\| ".join([CELL] * 4), text)
    return (float(t[2]), float(o[2]), float(t[4]), float(o[4]))


def offsets(text):
    """(told, open) top-1 of the offsets summed: the last cell of the table headed so."""
    if "the offsets summed" not in text:
        return None
    text = text.split("the offsets summed", 1)[1]
    t = re.findall(r"told after the last word or pair \| \d+ \| (?:" + CELL + r" \| ){3}" + CELL + r" \|\n", text)
    o = re.findall(r"never told after either \| \d+ \| (?:" + CELL + r" \| ){3}" + CELL + r" \|\n", text)
    return (float(t[-1][-1]), float(o[-1][-1])) if t and o else None


def run(name):
    text = open(os.path.join(RUNS, name, "log.md")).read()
    out = {}
    for block in re.split(r"\n## Epoch ", text)[1:]:
        e = int(block.split(" ", 1)[0])
        out[e] = (strata(block), offsets(block))
    for e in (4, 8):
        p = os.path.join(RUNS, name, "measured", f"epoch_{e}.md")
        if os.path.exists(p) and e in out and out[e][1] is None:
            out[e] = (out[e][0], offsets(open(p).read()))
    return out


plain, ahead = run("operator_r64_s7"), run("operator_r64_s7_ahead3")
fig, axs = plt.subplots(1, 2, figsize=(12.4, 4.4))
fig.patch.set_facecolor(SURF)
for ax, s, title, top in [(axs[0], 0, "told after the last word or pair (77% of positions)", 18), (axs[1], 1, "never told after either (23% of positions)", 4.5)]:
    ax.set_facecolor(SURF)
    for side in ("top", "right"):
        ax.spines[side].set_visible(False)
    ax.grid(axis="y", color=GRID, lw=0.8); ax.set_axisbelow(True)
    freq = list(ahead.values())[-1][0][2 + s]
    ax.axhline(freq, color=PRIOR, lw=1.6, ls=(0, (4, 3)))
    ax.text(32.5, freq, f"frequency {freq:.1f}%", va="center", fontsize=8.4, color=INK)
    lines = [
        (sorted((e, v[0][s]) for e, v in plain.items() if e <= 32), BLUE, "next word only: last · next"),
        (sorted((e, v[0][s]) for e, v in ahead.items()), ORANGE, "with offsets: last · next"),
        (sorted((e, v[1][s]) for e, v in ahead.items() if v[1]), VIOLET, "with offsets: the three summed"),
    ]
    for pts, col, name in lines:
        ax.plot([p[0] for p in pts], [p[1] for p in pts], color=col, lw=2, marker="o", ms=5, mec=SURF, mew=1.5, label=name)
    ax.set_xlim(0, 32); ax.set_ylim(0, top)
    ax.set_xlabel("epochs trained", fontsize=8.6); ax.set_ylabel("top-1 on held-out Emma (%)", fontsize=8.6)
    ax.set_title(title, fontsize=10.2, loc="left", color=INK)
axs[1].legend(frameon=False, fontsize=8.2, loc="lower right")
fig.text(0.01, -0.03, "operator_run, R = 64, seed 7: with offsets, 548,992 told rows of `next`, `next2`, `next3`; next word only, 185,209 rows (docs/150). 10,578 held-out positions (docs/151).", fontsize=7.8, color=INK2)
fig.tight_layout()
fig.savefig(os.path.join(HERE, "fig", "f151_offsets.png"), dpi=170, bbox_inches="tight", facecolor=SURF)
print(sorted(ahead), "checkpoints with offsets")
