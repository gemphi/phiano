"""Paper 25, second edition: the figures as infographics. Palette is the
validated eight-slot categorical (dataviz reference; adjacent CVD ΔE ≥ 9.1,
normal-vision ≥ 19.6; aqua, yellow and magenta under 3:1 so every mark is
direct-labelled). Colour follows the entity across the paper: counts blue,
operator orange, kin aqua, prior grey, phasor magenta, transformer violet.
Text is always ink, never a series colour. Every number is from the doc named
beside it; nothing here is illustrative unless the caption says so."""
import os
import numpy as np, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch, Circle, Rectangle

OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "fig")
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10, "axes.titleweight": "bold", "axes.edgecolor": "#8a8984", "axes.labelcolor": "#52514e", "xtick.color": "#52514e", "ytick.color": "#52514e"})
INK, INK2, MUTED, GRID, SURF = "#0b0b0b", "#52514e", "#8a8984", "#e6e5e1", "#fcfcfb"
BLUE, ORANGE, AQUA, YELLOW, MAGENTA, GREEN, VIOLET, RED = "#2a78d6", "#eb6834", "#1baf7a", "#eda100", "#e87ba4", "#008300", "#4a3aa7", "#e34948"
SLOTS = [BLUE, ORANGE, AQUA, YELLOW, MAGENTA, GREEN, VIOLET, RED]
COUNTS, OPERATOR, KIN, PRIOR, PHASOR, TRANSFORMER = BLUE, ORANGE, AQUA, "#a3a29c", MAGENTA, VIOLET
SEQ = ["#cde2fb", "#9ec5f4", "#6da7ec", "#3987e5", "#256abf", "#184f95", "#0d366b"]
DIV_NEG, DIV_MID = RED, "#f0efec"
GOOD, CRITICAL = "#0ca30c", "#d03b3b"


def save(fig, name):
    fig.savefig(f"{OUT}/{name}.png", dpi=170, bbox_inches="tight", facecolor=SURF)
    plt.close(fig)


def canvas(w, h):
    fig, ax = plt.subplots(figsize=(w, h))
    fig.patch.set_facecolor(SURF)
    ax.set_xlim(0, w); ax.set_ylim(0, h); ax.axis("off")
    return fig, ax


def card(ax, x, y, w, h, color, title=None, lines=(), big=None, fs=8.6, strip=0.09, fc="white"):
    """A card with a colour strip on the left: the strip carries identity, the text stays ink."""
    ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0.0,rounding_size=0.06", fc=fc, ec=GRID, lw=1.0))
    ax.add_patch(FancyBboxPatch((x, y), strip, h, boxstyle="round,pad=0.0,rounding_size=0.04", fc=color, ec="none"))
    tx, top = x + strip + 0.14, y + h - 0.2
    if big is not None:
        ax.text(tx, top - 0.1, big, fontsize=21, fontweight="bold", color=INK, va="top")
        top -= 0.62
    if title:
        ax.text(tx, top, title, fontsize=fs + 1.2, fontweight="bold", color=INK, va="top")
        top -= 0.3
    for l in lines:
        ax.text(tx, top, l, fontsize=fs, color=INK2, va="top")
        top -= 0.25


def arrow(ax, p, q, color=INK2, lw=1.4, rad=0.0, style="-|>"):
    ax.add_patch(FancyArrowPatch(p, q, arrowstyle=style, mutation_scale=12, color=color, lw=lw, connectionstyle=f"arc3,rad={rad}"))


def chart_axes(ax):
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)
    ax.grid(axis="y", color=GRID, lw=0.8); ax.set_axisbelow(True)


# ------------------------------------------------------------- 1. results at a glance
def fig_glance():
    fig, ax = canvas(12.6, 6.6)
    ax.text(0.3, 6.3, "Phasor Language Models — what was measured", fontsize=15, fontweight="bold", color=INK)
    ax.text(0.3, 5.98, "every number from a run in the record, named by its doc · nothing tuned to a benchmark · no epochs in the base", fontsize=9, color=INK2)
    tiles = [
        (BLUE, "15% → 61%", "one answer, an unseen family", ["code student, Phi-4's 253 answers:", "one shown answer, read into rows", "the moment it is given (docs/124)"]),
        (ORANGE, "100%", "acted on, after one pointing", ["sepia, taught once by pointing at a", "stone and saying colour; 50 later", "orders (builders' game, paper 23)"]),
        (AQUA, "50 / 50", "read back at 5,000 turns", ["10 needles · 20 keys · 20 updates,", "173,000 words, two seeds,", "16 ms an ask (docs/129, 133)"]),
        (YELLOW, "0 · 0 · 0", "epochs · summaries · drops", ["a telling is a row, counted under", "every condition on its path;", "nothing is re-read or let go"]),
        (MAGENTA, "75%", "a new type of a known task", ["Rust that compiles and passes,", "the compiler the only judge", "(248 audited tasks, docs/124)"]),
        (GREEN, "exact", "a story said back", ["drawn into its own scope after", "three words, then continued in", "Emma's language (docs/127–128)"]),
        (VIOLET, "7 of 64", "levels a talk supports", ["word (AAA) to story (TTT), divided", "where it shortens the description:", "+39.9 nats (the strand, docs/142)"]),
        (RED, "0 heads", "trained for a constraint", ["forbid · want · holds · degree:", "rows read, not projections trained", "(phiecs rule.rs, paper 26 §11)"]),
    ]
    for i, (c, big, title, lines) in enumerate(tiles):
        col, row = i % 4, i // 4
        card(ax, 0.3 + col * 3.05, 3.4 - row * 2.3, 2.92, 2.12, c, title=title, lines=lines, big=big, fs=8.0)
    ax.add_patch(FancyBboxPatch((0.3, 0.15), 12.07, 0.7, boxstyle="round,pad=0.0,rounding_size=0.06", fc="#f0efec", ec=GRID, lw=1.0))
    ax.text(0.5, 0.5, "not yet", fontsize=9.5, fontweight="bold", color=INK, va="center")
    ax.text(1.45, 0.62, "on new prose the next word is read at the trigram ceiling: 20.6% top-1.", fontsize=8.6, color=INK2, va="center")
    ax.text(1.45, 0.36, "in the 23% of positions never told after the last word, nothing yet beats frequency (docs/137–138).", fontsize=8.6, color=INK2, va="center")
    save(fig, "f25v_glance")


# ------------------------------------------------------------- 2. two places to keep what you know
def fig_two_places():
    fig, ax = canvas(12.6, 5.6)
    ax.text(0.3, 5.25, "Two places to keep what you know — and the one we use instead", fontsize=12.5, fontweight="bold", color=INK)
    # left: the transformer
    ax.add_patch(FancyBboxPatch((0.3, 0.35), 5.6, 4.55, boxstyle="round,pad=0.0,rounding_size=0.08", fc="white", ec=GRID))
    ax.add_patch(Rectangle((0.3, 4.55), 5.6, 0.35, fc=TRANSFORMER, ec="none"))
    ax.text(0.5, 4.72, "a transformer (and a Context Language Model)", fontsize=10, fontweight="bold", color="white", va="center")
    ax.add_patch(FancyBboxPatch((0.6, 2.75), 2.3, 1.5, boxstyle="round,pad=0.0,rounding_size=0.05", fc="#ecebf7", ec=TRANSFORMER))
    for r in range(5):
        for c in range(7):
            ax.add_patch(Rectangle((0.75 + c * 0.29, 2.95 + r * 0.22), 0.24, 0.17, fc=TRANSFORMER, alpha=0.18 + 0.12 * ((r * 7 + c) % 5), ec="none"))
    ax.text(1.75, 2.55, "weights: closed after epochs", ha="center", fontsize=8.4, color=INK)
    lens = [0.9, 1.6, 2.4, 3.0, 2.0]
    for i, L in enumerate(lens):
        ax.add_patch(Rectangle((3.2, 4.05 - i * 0.3), L * 0.62, 0.2, fc=TRANSFORMER if i < 4 else MUTED, alpha=0.75 if i < 4 else 0.5, ec="none"))
    ax.text(3.2, 2.55, "context: one text, re-read each step", fontsize=8.4, color=INK)
    ax.text(5.15, 2.93, "← summarise · drop", ha="left", fontsize=7.6, color=INK2, style="italic")
    for t, y in [("what was said in this session is not learned", 1.9), ("keeping is a judgment the model makes about its own words", 1.55), ("cost grows with the session: a pass over what survived", 1.2), ("budget: 32K–272K tokens; nudges at 25/50/75%", 0.85)]:
        ax.add_patch(Circle((0.62, y + 0.06), 0.05, fc=TRANSFORMER, ec="none")); ax.text(0.78, y + 0.06, t, fontsize=8.4, color=INK2, va="center")
    # right: the phasor language model
    ax.add_patch(FancyBboxPatch((6.6, 0.35), 5.7, 4.55, boxstyle="round,pad=0.0,rounding_size=0.08", fc="white", ec=GRID))
    ax.add_patch(Rectangle((6.6, 4.55), 5.7, 0.35, fc=COUNTS, ec="none"))
    ax.text(6.8, 4.72, "a Phasor Language Model", fontsize=10, fontweight="bold", color="white", va="center")
    nodes = {"story": (9.45, 4.15), "ch1": (8.3, 3.55), "ch2": (10.6, 3.55), "p1": (7.6, 2.95), "p2": (8.95, 2.95), "p3": (10.6, 2.95)}
    for a, b in [("story", "ch1"), ("story", "ch2"), ("ch1", "p1"), ("ch1", "p2"), ("ch2", "p3")]:
        ax.plot([nodes[a][0], nodes[b][0]], [nodes[a][1], nodes[b][1]], color=SEQ[2], lw=1.6, zorder=1)
    for i, (k, (x, y)) in enumerate(nodes.items()):
        ax.add_patch(FancyBboxPatch((x - 0.42, y - 0.17), 0.84, 0.34, boxstyle="round,pad=0.0,rounding_size=0.08", fc=SEQ[0] if k != "p2" else COUNTS, ec=COUNTS, lw=1.0, zorder=2))
        ax.text(x, y, k, ha="center", va="center", fontsize=8.4, color=INK if k != "p2" else "white", zorder=3)
    ax.add_patch(FancyBboxPatch((7.0, 2.25), 4.9, 0.4, boxstyle="round,pad=0.0,rounding_size=0.05", fc="#eef5fd", ec=COUNTS, lw=1.0))
    ax.text(9.45, 2.45, "(she, next, suddenly) under story/ch1/p2 · +1", ha="center", va="center", fontsize=8.2, color=INK, family="DejaVu Sans Mono")
    for t, y in [("what was said is learned: every row, at the moment it is told", 1.9), ("keeping is what a count does; nothing chooses", 1.55), ("cost is the row; an ask is a sum over the conditions in play", 1.2), ("no budget: the store grows with what is told, not with time", 0.85)]:
        ax.add_patch(Circle((6.92, y + 0.06), 0.05, fc=COUNTS, ec="none")); ax.text(7.08, y + 0.06, t, fontsize=8.4, color=INK2, va="center")
    arrow(ax, (5.95, 3.4), (6.55, 3.4), color=INK2, lw=1.6)
    save(fig, "f25v_two_places")


# ------------------------------------------------------------- 3. the rules
def fig_rules():
    fig, ax = canvas(12.6, 5.4)
    ax.text(0.3, 5.1, "The rules a word follows — each a remark of Wittgenstein's made exact", fontsize=12.5, fontweight="bold", color=INK)
    rules = [
        ("movement · D1", "prior · Π distᵢ/prior", ["meaning is use (§43): how far", "a word moves the answer"]),
        ("post · D2", "heard if it moves ≥ 0.5 nats", ["a word every turn says moves", "nothing; one rare telling is heard"]),
        ("locality · D3", "said by ≤ ½ the siblings", ["a condition belongs to its game", "(§23): a chapter's words draw none"]),
        ("nested · D4", "each level moves the one above", ["X ⇒ Y under c, c a word under", "its own c; Witten–Bell smoothing"]),
        ("now over was", "of equals, the later telling", ["no forgetting: the latest value", "of a key set twice is read first"]),
        ("the cast", "enter what says most, beyond", ["a ray through a bounding-volume", "hierarchy, drawn by what is said"]),
        ("a phasor with a count", "Σ ln⁺(p_s/p) · f(w)", ["the count asks membership;", "the phasor holds a direction"]),
        ("the slow speed", "Fourier operator on functions", ["what a word comes to know about", "words like it; beside, never under"]),
    ]
    for i, (t, f, lines) in enumerate(rules):
        col, row = i % 4, i // 4
        x, y = 0.3 + col * 3.05, 2.6 - row * 2.3
        card(ax, x, y, 2.92, 2.12, SLOTS[i], title=t, lines=[], fs=8.4)
        ax.add_patch(FancyBboxPatch((x + 0.25, y + 1.0), 2.52, 0.48, boxstyle="round,pad=0.0,rounding_size=0.04", fc="#f6f6f4", ec="none"))
        ax.text(x + 1.51, y + 1.24, f, ha="center", va="center", fontsize=7.6, color=INK, family="DejaVu Sans Mono")
        for j, l in enumerate(lines):
            ax.text(x + 0.25, y + 0.68 - j * 0.26, l, fontsize=7.9, color=INK2, va="center")
    ax.text(6.3, 0.05, "set once by a measurement and left: τ = 0.5 since docs/113, smoothing weight one, beam two, gate 'two words and three pairs'", ha="center", fontsize=8.2, color=INK2)
    save(fig, "f25v_rules")


# ------------------------------------------------------------- 4. one pointing
def fig_pointing():
    fig, ax = plt.subplots(figsize=(8.6, 3.5)); fig.patch.set_facecolor(SURF); ax.set_facecolor(SURF)
    cases = ["a family never seen", "the same family, one answer shown", "a new type of a known task", "sepia, after one pointing (of 50 orders)"]
    v = [15, 61, 75, 100]
    y = np.arange(len(cases))[::-1]
    ax.barh(y, v, height=0.52, color=COUNTS, edgecolor=SURF, linewidth=2)
    for yi, vi in zip(y, v):
        ax.text(vi + 1.5, yi, f"{vi}%", va="center", fontsize=10, fontweight="bold", color=INK)
    ax.set_yticks(y); ax.set_yticklabels(cases, fontsize=9, color=INK)
    ax.set_xlim(0, 112); ax.set_xticks([0, 25, 50, 75, 100]); ax.set_xticklabels(["0", "25", "50", "75", "100%"])
    for s in ("top", "right", "left"):
        ax.spines[s].set_visible(False)
    ax.tick_params(axis="y", length=0); ax.grid(axis="x", color=GRID, lw=0.8); ax.set_axisbelow(True)
    ax.annotate("", xy=(61, y[1] + 0.32), xytext=(15, y[0] + 0.32), arrowprops=dict(arrowstyle="-|>", color=INK2, lw=1.4, connectionstyle="arc3,rad=-0.25"))
    ax.text(38, 2.35, "+46 points from one answer", ha="center", fontsize=8.6, color=INK2, style="italic")
    ax.set_title("Learning from one pointing — the code student (docs/124) and the builders' game (paper 23)", fontsize=10, loc="left", color=INK)
    save(fig, "f25v_pointing")


# ------------------------------------------------------------- 5. the session that does not grow (small multiples, one axis each)
def fig_session():
    turns = [25, 100, 400, 2000, 5000]
    words = [1159, 3805, 14054, 69470, 172780]  # docs/129, seed 11 (seed 7 the same)
    fig, axs = plt.subplots(1, 3, figsize=(12.0, 3.4)); fig.patch.set_facecolor(SURF)
    a, b, c = axs
    for x in axs:
        x.set_facecolor(SURF); chart_axes(x); x.set_xscale("log"); x.set_xticks(turns); x.set_xticklabels([str(t) for t in turns], fontsize=8)
        x.set_xlabel("turns in the session", fontsize=8.5)
    a.plot(turns, [w / 1000 for w in words], "-o", color=COUNTS, lw=2, ms=6)
    a.set_yscale("log"); a.set_yticks([1, 10, 100]); a.set_yticklabels(["1", "10", "100"]); a.set_title("what is told grows", fontsize=10, loc="left", color=INK)
    a.text(5000, 172.78 * 1.35, "172,780 words", ha="right", fontsize=8.5, color=INK); a.set_ylabel("words told (thousands)", fontsize=8.5)
    b.plot(turns, [100] * 5, "-o", color=COUNTS, lw=2, ms=6); b.set_ylim(0, 112); b.set_yticks([0, 50, 100]); b.set_yticklabels(["0", "50", "100%"])
    b.set_title("what is read back does not fall", fontsize=10, loc="left", color=INK)
    b.text(160, 88, "50 of 50 at every length, two seeds:\n10 needles · 20 keys · 20 updated keys\nnothing summarised, nothing dropped", fontsize=8, color=INK2, ha="center", va="top")
    ms_turns, ms = [25, 100, 2000, 5000], [2, 3, 10, 16]  # docs/129 (25, 100); docs/133 (2,000, 5,000)
    c.plot(ms_turns, ms, "-o", color=COUNTS, lw=2, ms=6); c.set_ylim(0, 20); c.set_yticks([0, 5, 10, 15, 20]); c.set_ylabel("one ask (ms)", fontsize=8.5)
    for t, m in zip(ms_turns, ms):
        c.text(t, m + 1.1, f"{m} ms", ha="center", fontsize=8.2, color=INK)
    c.set_title("what an ask costs stays small", fontsize=10, loc="left", color=INK)
    fig.text(0.01, -0.04, "docs/129 (one file per turn, Emma as filler) and docs/133 (the store on ids). 400 turns' time was measured only before docs/133 and is left out.", fontsize=7.8, color=INK2)
    fig.tight_layout()
    save(fig, "f25v_session")


# ------------------------------------------------------------- 6. the honest ceiling, in two strata
def fig_strata():
    readers = [("counts (flat)", COUNTS), ("operator, last word", OPERATOR), ("kin of the last word", KIN), ("prior alone", PRIOR)]
    told = [26.5, 11.5, 14.2, 6.9]; never = [1.1, 1.3, 0.5, 3.4]
    fig, axs = plt.subplots(1, 2, figsize=(11.0, 3.6), gridspec_kw={"width_ratios": [1, 1]}); fig.patch.set_facecolor(SURF)
    for ax, vals, title, top in [(axs[0], told, "heard: the word that came had been told\nafter the last word or pair — 77% of positions", 30), (axs[1], never, "unheard: it had never been told after them\n— 23% of positions, the open problem", 4.2)]:
        ax.set_facecolor(SURF); chart_axes(ax)
        x = np.arange(4)
        ax.bar(x, vals, width=0.62, color=[c for _, c in readers], edgecolor=SURF, linewidth=2)
        for xi, vi in zip(x, vals):
            ax.text(xi, vi + top * 0.025, f"{vi}%", ha="center", fontsize=9.5, fontweight="bold", color=INK)
        ax.set_xticks(x); ax.set_xticklabels([r for r, _ in readers], fontsize=8.2, color=INK)
        ax.set_ylim(0, top); ax.set_ylabel("top-1", fontsize=8.5)
        ax.set_title(title, fontsize=9, loc="left", color=INK)
    fig.text(0.01, -0.03, "held-out Emma, 5% of paragraphs, 10,578 positions (docs/137–138). Each panel has its own scale: the right is the open problem, where nothing yet beats frequency.", fontsize=7.8, color=INK2)
    fig.tight_layout()
    save(fig, "f25v_strata")


# ------------------------------------------------------------- 7. the phasor against the count
def fig_capacity():
    R = [64, 256, 1024, 4096]
    fig, ax = plt.subplots(figsize=(7.8, 3.5)); fig.patch.set_facecolor(SURF); ax.set_facecolor(SURF); chart_axes(ax)
    ax.plot(R, [20] * 4, "-", color=COUNTS, lw=2.2)
    ax.plot(R, [11, 14, 16, 18], "-o", color=PHASOR, lw=2, ms=7, markeredgecolor=SURF, markeredgewidth=2)
    ax.plot(R, [14, 15, 15, 17], "-o", color=VIOLET, lw=2, ms=7, markeredgecolor=SURF, markeredgewidth=2)
    ax.set_xscale("log"); ax.set_xticks(R); ax.set_xticklabels([str(r) for r in R]); ax.set_xlabel("R, samples per scope phasor", fontsize=8.5)
    ax.set_ylim(8, 21.5); ax.set_ylabel("latest value read, of 20", fontsize=8.5)
    ax.text(4096 * 1.08, 20, "count table: 20/20", va="center", fontsize=8.5, color=INK)
    ax.text(4096 * 1.08, 18, "phasor, 400 turns", va="center", fontsize=8.5, color=INK)
    ax.text(4096 * 1.08, 16.6, "phasor, 2,000 turns", va="center", fontsize=8.5, color=INK)
    ax.set_xlim(50, 4096 * 3.2)
    ax.legend(["count table", "phasor cast, 400 turns", "phasor cast, 2,000 turns"], loc="lower right", fontsize=7.8, frameon=False)
    ax.set_title("A superposition has a capacity; a table has none (docs/136)", fontsize=10, loc="left", color=INK)
    save(fig, "f25v_capacity")


# ------------------------------------------------------------- 8. the strand: a talk's own levels
STRAND = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "data", "talks", "world_models.strand")


def fig_strand():
    rows = [l.rstrip("\n").split("\t") for l in open(os.path.join(STRAND, "levels.tsv"))][1:]
    k = np.array([int(r[0]) for r in rows]); codon = [r[1] for r in rows]; units = [int(r[3]) for r in rows]; nats = np.array([float(r[4]) for r in rows])
    dna = open(os.path.join(STRAND, "story.dna")).read().split("\n")
    i = next(j for j, l in enumerate(dna) if l.startswith("TGC "))
    beats = []
    for l in dna[i + 1:]:
        if not l.startswith("  "):
            break
        span, words = l.strip().split("\t")
        beats.append((span, words))
    fig = plt.figure(figsize=(12.6, 7.0)); fig.patch.set_facecolor(SURF)
    ax = fig.add_axes([0.05, 0.1, 0.36, 0.78]); ax.set_facecolor(SURF)
    colors = [COUNTS if v > 0 else DIV_NEG for v in nats]
    ax.barh(k, nats, height=0.78, color=colors, edgecolor=SURF, linewidth=0.6)
    ax.axvline(0, color=INK2, lw=0.8)
    ax.axhspan(56.5, 63.5, color="#eef5fd", zorder=0)
    ax.set_ylim(-1, 64); ax.set_xlim(-70, 30)
    show = [63, 61, 57, 47, 35, 23, 11, 0]
    ax.set_yticks(show); ax.set_yticklabels([f"{codon[63 - s]}  {units[63 - s]} units" for s in show], fontsize=8.2, family="DejaVu Sans Mono", color=INK)
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)
    ax.set_xlabel("nats a level adds over the level above (net of what saying its divisions costs)", fontsize=8.2)
    ax.text(29, 52.5, "information gained,\nTTT → TGC:\n+39.9 nats", ha="right", va="center", fontsize=8.6, color=INK)
    ax.text(-68, 30, "below TGC every level\ncosts more to say than\nit saves: filled by size,\nlargest unit first", va="center", fontsize=8.4, color=INK2)
    ax.set_title("the talk as 64 levels, TTT (story) to AAA (word)", fontsize=10.5, loc="left", color=INK)
    # right: the seven beats the information supports
    ax2 = fig.add_axes([0.45, 0.06, 0.53, 0.86]); ax2.set_xlim(0, 7); ax2.set_ylim(0, 8.2); ax2.axis("off")
    ax2.text(0.05, 7.95, "TGC ䷹ — the seven beats the talk's information supports", fontsize=11, fontweight="bold", color=INK, va="top")
    ax2.text(0.05, 7.62, "each beat in the words it says beyond the talk as a whole (D4), in the order said; nothing generated", fontsize=8.3, color=INK2, va="top")
    for j, (span, words) in enumerate(beats):
        y = 6.5 - j * 0.95
        a, b = (int(x) for x in span.split("-"))
        card(ax2, 0.05, y, 6.85, 0.82, SLOTS[j], fs=8.4)
        ax2.text(0.32, y + 0.58, f"beat {j + 1}", fontsize=8.2, color=INK2, va="center")
        ax2.text(1.15, y + 0.58, words, fontsize=10.2, fontweight="bold", color=INK, va="center")
        ax2.add_patch(Rectangle((1.15, y + 0.17), 5.5, 0.14, fc=GRID, ec="none"))
        ax2.add_patch(Rectangle((1.15 + 5.5 * a / 730, y + 0.17), 5.5 * (b - a) / 730, 0.14, fc=SLOTS[j], ec="none"))
        ax2.text(6.68, y + 0.58, f"tokens {a}–{b}", ha="right", va="center", fontsize=7.4, color=INK2)
    fig.text(0.05, 0.015, "strand = phiplm::prose::strand (docs/142): the talk divided where it shortens its description (Witten–Bell code, cut cost after the talk's own marks), pruned bottom-up (MDL); prior = Emma.", fontsize=7.8, color=INK2)
    save(fig, "f25v_strand")


# ------------------------------------------------------------- 9. the cast
def fig_cast():
    fig, ax = canvas(12.6, 5.0)
    ax.text(0.3, 4.7, "The cast: a question drawn down the tree by what each scope says beyond its parent", fontsize=12.5, fontweight="bold", color=INK)
    ax.text(0.3, 4.4, "\"what is the code for the vault?\" asked after 2,000 turns (docs/126, 129)", fontsize=9, color=INK2)
    root = (4.4, 3.75)
    blocks = [("b000", 1.0), ("b001", 2.7), ("b002", 4.4), ("b003", 6.1), ("… b019", 7.8)]
    turns = [("t00212", 2.9, True), ("t00227", 4.4, False), ("t00251", 5.9, False)]
    def node(x, y, label, on, w=1.2):
        ax.add_patch(FancyBboxPatch((x - w / 2, y - 0.2), w, 0.4, boxstyle="round,pad=0.0,rounding_size=0.08", fc=COUNTS if on else "white", ec=COUNTS if on else GRID, lw=1.3, zorder=3))
        ax.text(x, y, label, ha="center", va="center", fontsize=8.8, color="white" if on else INK2, zorder=4)
    node(*root, "session", True, w=1.5)
    for name, x in blocks:
        on = name == "b002"
        ax.plot([root[0], x], [root[1] - 0.2, 2.75 + 0.2], color=COUNTS if on else GRID, lw=2.2 if on else 1.0, zorder=1)
        node(x, 2.75, name, on)
    for name, x, on in turns:
        ax.plot([4.4, x], [2.75 - 0.2, 1.75 + 0.2], color=COUNTS if on else GRID, lw=2.2 if on else 1.0, zorder=1)
        node(x, 1.75, name, on, w=1.4)
    ax.add_patch(FancyBboxPatch((0.6, 0.35), 4.6, 0.62, boxstyle="round,pad=0.0,rounding_size=0.06", fc="#eef5fd", ec=COUNTS))
    ax.text(2.9, 0.66, "read under t00212: the vault is ⇒ 4471", ha="center", va="center", fontsize=8.8, color=INK, family="DejaVu Sans Mono")
    arrow(ax, (2.9, 1.55), (2.9, 0.99), color=COUNTS, lw=1.6)
    notes = [("enters", "the children saying most of the asked words beyond their parent"), ("prunes", "a word most siblings say (D3): 'the', 'is' draw nowhere"), ("hears", "the leaf only if it says two of the words and the last three pairs"), ("prefers", "the later of equals: now over was")]
    for j, (a, b) in enumerate(notes):
        y = 3.2 - j * 0.6
        ax.add_patch(Circle((8.85, y + 0.05), 0.07, fc=SLOTS[j], ec="none"))
        ax.text(9.05, y + 0.05, a, fontsize=9, fontweight="bold", color=INK, va="center")
        ax.text(9.05, y - 0.2, b, fontsize=7.8, color=INK2, va="center")
    save(fig, "f25v_cast")


# ------------------------------------------------------------- 10. beside the Context Language Model
def fig_clm():
    fig, ax = canvas(12.6, 6.4)
    ax.text(0.3, 6.1, "Beside the Context Language Model — what each design keeps, and what it costs", fontsize=12.5, fontweight="bold", color=INK)
    ax.text(5.6, 5.62, "Context Language Model", ha="center", fontsize=9.5, fontweight="bold", color=INK)
    ax.text(9.5, 5.62, "Phasor Language Model", ha="center", fontsize=9.5, fontweight="bold", color=INK)
    ax.add_patch(Rectangle((4.0, 5.82), 3.2, 0.08, fc=TRANSFORMER, ec="none")); ax.add_patch(Rectangle((7.9, 5.82), 3.2, 0.08, fc=COUNTS, ec="none"))
    rows = [
        ("a telling is kept", "as tokens, until summarised", 0, "as a row, under every condition", 1),
        ("learned from a session", "a procedure or an editing policy", 0, "what was said, every row, at once", 1),
        ("budget", "32K–272K tokens", 0, "none: grows with what is told", 1),
        ("one pointing", "not a mechanism of the design", 0, "15% → 61%; sepia 100%", 1),
        ("read by a person", "a summary in the model's words", 0, "the rows themselves, exact", 1),
        ("an instruction in the input", "an editable channel (their caveat)", 0, "rows about its words (D19)", 1),
        ("general language", "a 9–27B model's whole language", 1, "what it was told: 158K words, 253 answers", 0),
        ("the relation never told", "the model's prior over everything", 1, "frequency, for now (23% of prose)", 0),
    ]
    for j, (what, clm, cg, plm, pg) in enumerate(rows):
        y = 5.0 - j * 0.6
        if j % 2 == 0:
            ax.add_patch(Rectangle((0.3, y - 0.27), 12.0, 0.54, fc="#f6f6f4", ec="none"))
        ax.text(0.45, y, what, fontsize=8.8, fontweight="bold", color=INK, va="center")
        for x, txt, good in [(4.0, clm, cg), (7.9, plm, pg)]:
            ax.add_patch(Circle((x + 0.12, y), 0.11, fc=GOOD if good else "#d6d5cf", ec="none"))
            ax.text(x + 0.12, y - 0.01, "✓" if good else "–", ha="center", va="center", fontsize=8, color="white" if good else INK2, fontweight="bold")
            ax.text(x + 0.32, y, txt, fontsize=8.4, color=INK, va="center")
    ax.text(0.3, 0.12, "✓ = the stronger side on that row, as each paper states it. Facts from Shao et al. (2026) and our record, not a benchmark of one against the other. The last two rows are theirs.", fontsize=7.9, color=INK2)
    save(fig, "f25v_clm")


# ------------------------------------------------------------- 11. nested levels (illustrative shape)
def fig_levels():
    words = ["the", "a", "her", "suddenly", "had", "4471", "and"]
    prior = np.array([0.30, 0.18, 0.12, 0.02, 0.10, 0.003, 0.15]); scope = np.array([0.20, 0.10, 0.10, 0.25, 0.12, 0.08, 0.10])
    word = np.array([0.08, 0.05, 0.06, 0.55, 0.12, 0.05, 0.05]); pair = np.array([0.03, 0.03, 0.03, 0.80, 0.05, 0.02, 0.03])
    levels = [("prior: what the kind usually names", prior), ("+ the scope the cast reached", scope), ("+ the last word", word), ("+ the last pair", pair)]
    fig, axs = plt.subplots(1, 4, figsize=(12.0, 2.9), sharey=True); fig.patch.set_facecolor(SURF)
    for j, (ax, (t, d)) in enumerate(zip(axs, levels)):
        d = d / d.sum(); ax.set_facecolor(SURF); chart_axes(ax)
        ax.bar(range(len(words)), d, color=SEQ[2 + j], edgecolor=SURF, linewidth=1.5)
        ax.set_xticks(range(len(words))); ax.set_xticklabels(words, rotation=40, fontsize=7.8, color=INK)
        ax.set_title(t, fontsize=8.6, loc="left", color=INK)
        i = int(np.argmax(d)); ax.text(i, d[i] + 0.02, f"{d[i]:.2f}", ha="center", fontsize=8, color=INK)
    axs[0].set_ylabel("share of the next word", fontsize=8.5)
    fig.text(0.01, -0.05, "D4 nested: each level moves what the level above arrived at (Counts::read_levels, docs/126). Shapes illustrative; the measured numbers are §4's.", fontsize=7.8, color=INK2)
    fig.tight_layout()
    save(fig, "f25v_levels")


# ------------------------------------------------------------- 12. reading in the strand's units
BENCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "data", "talks", "strand_bench")


def fig_reading():
    """docs/145: held-out words read in each reader's scopes (counted weights), and the 64-level curve on cross joins."""
    suites = [("cross_joins", "four kinds joined"), ("synthetic_lambda_0.4", "synthetic, clear topics"), ("emma_joins", "Emma chapters joined"), ("synthetic_lambda_0.2", "synthetic, faint topics"), ("the_talk", "the talk")]
    readers = [("counted_strand_flat", "strand's units", BLUE), ("counted_strand", "strand's units, down its tree", ORANGE), ("counted_labels", "true labels", AQUA), ("counted_equal_parts", "equal parts, strand's count", YELLOW)]
    gains = {}
    for s, _ in suites:
        rows = []
        for seed in (7, 11):
            lines = [l.rstrip("\n").split("\t") for l in open(os.path.join(BENCH, f"read_seed{seed}", f"read_{s}_texts.tsv"))]
            head = lines[0]
            rows += [dict(zip(head, map(float, r))) for r in lines[1:]]
        held = sum(r["held"] for r in rows)
        whole = sum(r["counted_whole"] * r["held"] for r in rows) / held
        gains[s] = [whole - sum(r[c] * r["held"] for r in rows) / held for c, _, _ in readers]
    fig = plt.figure(figsize=(12.6, 5.4)); fig.patch.set_facecolor(SURF)
    ax = fig.add_axes([0.2, 0.14, 0.34, 0.72]); ax.set_facecolor(SURF)
    h = 0.19
    for i, (s, name) in enumerate(suites):
        for j, (_, rn, col) in enumerate(readers):
            y = len(suites) - 1 - i + (1.5 - j) * h
            g = gains[s][j]
            ax.barh(y, g, height=h - 0.03, color=col, edgecolor=SURF, linewidth=1.0)
            if i == 0 or s == "the_talk":
                ax.text(max(g, 0) + 0.008, y, f"{g:+.3f}", va="center", fontsize=7.4, color=INK2)
    ax.set_yticks(range(len(suites))); ax.set_yticklabels([n for _, n in suites][::-1], fontsize=8.8, color=INK)
    ax.axvline(0, color=INK2, lw=0.8)
    for s in ("top", "right", "left"):
        ax.spines[s].set_visible(False)
    ax.tick_params(axis="y", length=0)
    ax.grid(axis="x", color=GRID, lw=0.8); ax.set_axisbelow(True)
    ax.set_xlabel("nats a held-out word saved against reading the text as one scope", fontsize=8.2)
    ax.set_title("which scopes read a text's own held-out words best", fontsize=10.5, loc="left", color=INK)
    for j, (_, rn, col) in enumerate(readers):
        ax.add_patch(Rectangle((0.255, 3.35 - j * 0.27), 0.018, 0.16, fc=col, ec="none", transform=ax.transData, clip_on=False))
        ax.text(0.28, 3.43 - j * 0.27, rn, va="center", fontsize=8, color=INK)
    # right: the 64 levels on cross joins, both rules
    ax2 = fig.add_axes([0.63, 0.14, 0.35, 0.72]); ax2.set_facecolor(SURF); chart_axes(ax2)
    curves = []
    for seed in (7, 11):
        lines = [l.rstrip("\n").split("\t") for l in open(os.path.join(BENCH, f"read_seed{seed}", "read_cross_joins.tsv"))][1:]
        curves.append(np.array([[float(x) for x in r[1:]] for r in lines]))
    c = np.mean(curves, axis=0)
    u, wb, ct = c[:, 0], c[:, 1], c[:, 2]
    keep = u <= 400
    ax2.plot(u[keep], wb[keep], color=ORANGE, lw=2); ax2.plot(u[keep], ct[keep], color=BLUE, lw=2)
    ax2.set_xscale("log")
    i = int(np.argmin(ct)); ax2.plot([u[i]], [ct[i]], "o", ms=8, color=BLUE, mec=SURF, mew=2)
    sup = np.mean([float(l.split("\t")[1]) for seed in (7, 11) for l in open(os.path.join(BENCH, f"read_seed{seed}", "read_cross_joins_texts.tsv")).read().splitlines()[1:]])
    ax2.axvline(sup, color=INK2, lw=0.8, ls=(0, (3, 3)))
    ax2.text(sup * 1.04, 7.05, f"the strand's\nsupported count\n({sup:.1f} units)", fontsize=7.8, color=INK2, va="top")
    ax2.text(u[i] / 1.25, ct[i] - 0.06, f"best: {u[i]:.0f} units", ha="right", va="top", fontsize=7.8, color=INK)
    ax2.text(300, wb[keep][-1] + 0.05, "Witten–Bell", ha="right", va="bottom", fontsize=8.4, color=INK)
    ax2.text(300, ct[keep][-1] - 0.05, "counted weights", ha="right", va="top", fontsize=8.4, color=INK)
    ax2.set_xlabel("units at the level (log scale)", fontsize=8.2); ax2.set_ylabel("nats a held-out word", fontsize=8.2)
    ax2.set_title("four kinds joined: the 64 levels read under each rule", fontsize=10.5, loc="left", color=INK)
    fig.text(0.02, 0.015, "strand --read, seeds 7 and 11, 40 texts a suite, five folds (every fifth token held out). Left: counted weights (deleted interpolation, the text's own leave-one-out). Docs/145.", fontsize=7.8, color=INK2)
    save(fig, "f25v_reading")


if __name__ == "__main__":
    for f in [fig_glance, fig_two_places, fig_rules, fig_pointing, fig_session, fig_strata, fig_capacity, fig_strand, fig_cast, fig_clm, fig_levels, fig_reading]:
        f(); print("made", f.__name__)
