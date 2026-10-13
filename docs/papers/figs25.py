"""Figures for paper 25 (Phasor Language Models), paper 24 and docs/139 — same
style as figs.py. Every number drawn here is from the docs it names."""
import numpy as np, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch, Circle
import os

OUT = "/home/claude/papers/fig"
os.makedirs(OUT, exist_ok=True)
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10, "axes.titleweight": "bold"})
INK = "#1f2430"; ACC = "#c8451f"; ACC2 = "#1f6fb2"; ACC3 = "#2e8b57"; GREY = "#9aa0a6"; PALE = "#f4f6f9"


def save(fig, name):
    fig.savefig(f"{OUT}/{name}.png", dpi=170, bbox_inches="tight", facecolor="white")
    fig.savefig(f"{OUT}/{name}.svg", bbox_inches="tight", facecolor="white")
    plt.close(fig)


def box(ax, x, y, w, h, title, lines, fc=PALE, ec=INK, tc=INK, lw=1.4, fs=9.5):
    ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0.02,rounding_size=0.025", fc=fc, ec=ec, lw=lw))
    n = max(len(lines), 1)
    ax.text(x + w / 2, y + h * 0.80, title, ha="center", va="center", fontsize=fs + 1.5, fontweight="bold", color=tc)
    for i, l in enumerate(lines):
        ax.text(x + w / 2, y + h * (0.58 - 0.52 * (i + 0.5) / n + 0.26 / n), l, ha="center", va="center", fontsize=fs, color=INK)


def arrow(ax, p, q, color=INK, lw=1.4, style="-|>", ls="-"):
    ax.add_patch(FancyArrowPatch(p, q, arrowstyle=style, mutation_scale=12, color=color, lw=lw, linestyle=ls))


# ------------------------------------------------------------- the family
def fig_family():
    fig, ax = plt.subplots(figsize=(9.6, 5.9)); ax.set_xlim(0, 10); ax.set_ylim(-0.35, 6.0); ax.axis("off")
    ax.set_title("PML, Phi machine learning: one crate per reference library, dependencies one way", fontsize=12)
    # our crates (left column rising), references (right)
    ours = [
        (0.4, 0.3, "phinet", ["the engine", "Tensor · Tape · FFT · Adam", "hrr · hexagram · ranges"], "torch"),
        (0.4, 1.45, "phifno", ["the sphere", "64 real harmonics, l < 8", "grids · analysis/synthesis"], "torch-harmonics"),
        (0.4, 2.6, "phical", ["the operators", "Fno (circle · sphere · aniso) · Joint", "relations · Liskov · exact geometry"], "neuraloperator · makani"),
        (0.4, 3.75, "phiplm", ["the Phasor Language Model", "Counts · levels · cast · Field · kin", "RelationModel · prose · code"], "context-language-models"),
        (0.4, 4.9, "phiano", ["the applications", "students · games · personas", "67 binaries"], None),
    ]
    for (x, y, name, lines, ref) in ours:
        fc = "#fdf1ec" if name in ("phical", "phiplm") else PALE
        box(ax, x, y, 3.6, 0.95, name, lines, fc=fc, ec=ACC if name in ("phical", "phiplm") else INK)
        if ref:
            ax.add_patch(FancyBboxPatch((6.2, y + 0.18), 3.4, 0.6, boxstyle="round,pad=0.02", fc="white", ec=GREY, lw=1.1, linestyle="--"))
            ax.text(7.9, y + 0.48, ref, ha="center", va="center", fontsize=10, color=GREY, fontweight="bold")
            ax.text(7.9, y + 0.1, "read under gemphi/ext", ha="center", va="center", fontsize=7.5, color=GREY)
            arrow(ax, (6.2, y + 0.48), (4.05, y + 0.48), color=GREY, lw=1.0, style="-", ls="--")
    for i in range(4):
        arrow(ax, (2.2, ours[i][1] + 0.95), (2.2, ours[i + 1][1]), color=INK, lw=1.6)
    # phiecs and phiton on the side
    box(ax, 4.5, 3.75, 1.5, 0.95, "phiecs", ["the world", "words · quest"], fc=PALE)
    arrow(ax, (4.5, 4.22), (4.0, 4.22), color=INK, lw=1.4)
    box(ax, 4.5, 4.9, 1.5, 0.95, "phiton", ["light", "as units"], fc=PALE)
    arrow(ax, (4.5, 5.37), (4.0, 5.37), color=INK, lw=1.4)
    ax.text(5.0, -0.2, "an arrow points from what is used to what uses it; no crate reaches up", ha="center", fontsize=8.5, color=GREY)
    save(fig, "f25_family")


# ------------------------------------------------------------- two places
def fig_two_places():
    fig, axs = plt.subplots(1, 2, figsize=(12, 4.8), gridspec_kw={"wspace": 0.12})
    for ax in axs:
        ax.set_xlim(0, 10); ax.set_ylim(0, 8); ax.axis("off")
    ax = axs[0]
    ax.set_title("a transformer: two places,\nboth the wrong shape", fontsize=11, color=GREY)
    box(ax, 0.5, 4.3, 9, 3.2, "the weights", ["a language, learned over epochs", "closed: nothing said afterwards enters", "except by more epochs"], fc="#f0f0f0", ec=GREY, tc=GREY)
    box(ax, 0.5, 0.4, 9, 3.2, "the context", ["this session as one text, re-read every step", "grows with time; summarised and dropped at budget", "(a CLM lets the model edit it)"], fc="#f0f0f0", ec=GREY, tc=GREY)
    ax = axs[1]
    ax.set_title("a Phasor Language Model: one place,\nthe row under its conditions", fontsize=11, color=ACC)
    box(ax, 0.3, 5.6, 9.4, 1.9, "a telling is a row", ["(about, kind, other) · she@story/ch1/p2#4  next  suddenly", "counted under its kind (the prior) and under each condition"], fc="#fdf1ec", ec=ACC, tc=ACC, fs=8.5)
    # tree
    nodes = {"story": (4.5, 4.6), "ch1": (4.5, 3.5), "p2": (3.0, 2.4), "p3": (6.0, 2.4), "she": (1.5, 1.2), "pair: she suddenly": (4.6, 1.2)}
    edges = [("story", "ch1"), ("ch1", "p2"), ("ch1", "p3"), ("p2", "she"), ("p2", "pair: she suddenly")]
    for a, b in edges:
        ax.plot([nodes[a][0], nodes[b][0]], [nodes[a][1], nodes[b][1]], color=INK, lw=1.2, zorder=1)
    for n, (x, y) in nodes.items():
        c = ACC if n in ("p2", "she", "pair: she suddenly") else INK
        ax.add_patch(FancyBboxPatch((x - (1.4 if len(n) > 6 else 0.7), y - 0.26), (2.8 if len(n) > 6 else 1.4), 0.52, boxstyle="round,pad=0.02", fc="white", ec=c, lw=1.3, zorder=2))
        ax.text(x, y, n, ha="center", va="center", fontsize=8.5, color=c, zorder=3)
    ax.text(8.6, 3.0, "the conditions\nare a tree —\nthe file system —\nand each level\nis a word with\ncounts", ha="center", va="center", fontsize=8.5, color=INK)
    ax.text(5, 0.35, "nothing is re-read; nothing is dropped; a new file on disk is new rows", ha="center", fontsize=8.5, color=ACC)
    save(fig, "f25_two_places")


# ------------------------------------------------------------- the cast
def fig_cast():
    fig, ax = plt.subplots(figsize=(10, 5.2)); ax.set_xlim(0, 10); ax.set_ylim(0, 6); ax.axis("off")
    ax.set_title("the cast: the words said so far are drawn down the tree by what each scope says (docs/126, 129)", fontsize=11)
    ax.text(0.3, 5.55, 'asked: "the code for the vault is"  →  the last six words + the last three pairs', fontsize=9.5, color=INK)
    lv = {"session": (5, 4.6)}
    blocks = [("b000", 1.2), ("b001", 2.9), ("b002", 4.6), ("b003", 6.3), ("b019", 8.6)]
    turns = [("t00212", 3.2, True), ("t00227", 4.6, False), ("t00251", 6.0, False)]
    ax.add_patch(FancyBboxPatch((4.2, 4.35), 1.6, 0.5, boxstyle="round,pad=0.02", fc="white", ec=INK, lw=1.3))
    ax.text(5, 4.6, "session", ha="center", va="center", fontsize=9)
    for name, x in blocks:
        enter = name == "b002"
        c = ACC if enter else GREY
        ax.plot([5, x], [4.35, 3.55], color=c, lw=1.6 if enter else 0.9, ls="-" if enter else ":")
        ax.add_patch(FancyBboxPatch((x - 0.6, 3.05), 1.2, 0.5, boxstyle="round,pad=0.02", fc="#fdf1ec" if enter else "white", ec=c, lw=1.3))
        ax.text(x, 3.3, name, ha="center", va="center", fontsize=8.5, color=c)
        ax.text(x, 2.75, "says vault" if enter else ("says: nothing asked" if name != "b019" else "…"), ha="center", fontsize=7.5, color=c)
    ax.text(7.5, 3.3, "…", fontsize=12, color=GREY)
    for name, x, enter in turns:
        c = ACC if enter else GREY
        ax.plot([4.6, x], [3.05, 2.2], color=c, lw=1.6 if enter else 0.9, ls="-" if enter else ":")
        ax.add_patch(FancyBboxPatch((x - 0.6, 1.7), 1.2, 0.5, boxstyle="round,pad=0.02", fc="#fdf1ec" if enter else "white", ec=c, lw=1.3))
        ax.text(x, 1.95, name, ha="center", va="center", fontsize=8.5, color=c)
    ax.text(3.2, 1.4, "says: vault, code, pair: vault is", ha="center", fontsize=7.5, color=ACC)
    ax.text(3.2, 0.95, "the leaf is heard: it says ≥ 2 asked words and the last pairs", ha="center", fontsize=8, color=ACC)
    ax.text(3.2, 0.55, "read under it:  the code for the vault is → 4471", ha="center", fontsize=9, color=INK, fontweight="bold")
    ax.text(7.6, 1.2, "rules in play\n• a word counts for a scope only if ≤ half its\n  siblings say it (D3: is draws nowhere)\n• among equals the scope told later stands first\n  (phiecs Condition: now over was)\n• a scope says its pairs too", ha="left", va="center", fontsize=7.8, color=INK, bbox=dict(boxstyle="round", fc=PALE, ec=GREY))
    save(fig, "f25_cast")


# ------------------------------------------------------------- D4 nested levels
def fig_levels():
    words = ["the", "a", "her", "suddenly", "had", "4471", "and"]
    prior = np.array([0.30, 0.18, 0.12, 0.02, 0.10, 0.003, 0.15]); prior /= prior.sum()
    scope = np.array([0.20, 0.10, 0.10, 0.25, 0.12, 0.08, 0.10]); scope /= scope.sum()
    word = np.array([0.08, 0.05, 0.06, 0.55, 0.12, 0.05, 0.05]); word /= word.sum()
    pair = np.array([0.03, 0.03, 0.03, 0.80, 0.05, 0.02, 0.03]); pair /= pair.sum()
    fig, axs = plt.subplots(1, 4, figsize=(11, 3.1), sharey=True)
    for ax, d, t, c in zip(axs, [prior, scope, word, pair], ["the prior\nwhat 'next' usually names", "moved by the scope\n(a paragraph of the story)", "then by the last word\n'she'", "then by the pair\n'she suddenly'"], [GREY, ACC2, ACC, ACC]):
        ax.bar(range(len(words)), d, color=c, width=0.7)
        ax.set_xticks(range(len(words))); ax.set_xticklabels(words, rotation=45, ha="right", fontsize=8)
        ax.set_title(t, fontsize=9); ax.spines[["top", "right"]].set_visible(False)
    axs[0].set_ylabel("probability")
    fig.suptitle("D4 nested: each level moves what the level above arrived at; a level that was never told moves nothing (illustrative shape)", fontsize=10, y=1.04)
    save(fig, "f25_levels")


# ------------------------------------------------------------- phasor cast vs R
def fig_capacity():
    R = [64, 256, 1024, 4096]
    at400 = [11, 14, 16, 18]; at2000 = [14, 15, 15, 17]
    fig, ax = plt.subplots(figsize=(7.2, 3.8))
    ax.axhline(20, color=INK, lw=1.6); ax.text(70, 20.3, "count cast: 20 of 20 at every length (membership, no capacity)", fontsize=8.5, color=INK)
    ax.plot(R, at400, "o-", color=ACC, label="phasor cast, 400 turns")
    ax.plot(R, at2000, "s--", color=ACC2, label="phasor cast, 2,000 turns")
    ax.set_xscale("log", base=2); ax.set_xticks(R); ax.set_xticklabels([str(r) for r in R])
    ax.set_xlabel("R, the length of every word's and scope's function"); ax.set_ylabel("updated keys read (of 20)")
    ax.set_ylim(8, 21.5); ax.spines[["top", "right"]].set_visible(False); ax.legend(loc="lower right", fontsize=8.5, frameon=False)
    ax.set_title("a scope's phasor is a superposition with a capacity in R (docs/136)", fontsize=10)
    save(fig, "f25_capacity")


# ------------------------------------------------------------- strata
def fig_strata():
    labels = ["counts\n(flat)", "operator\n(last word)", "kin of the\nlast word", "prior\nalone"]
    told = [26.5, 11.5, 14.2, 6.9]; never = [1.1, 1.3, 0.5, 3.4]
    fig, axs = plt.subplots(1, 2, figsize=(10, 3.9))
    for ax, v, t, n in zip(axs, [told, never], ["told after\nthe last word or the last pair", "never told\nafter either"], ["8,147 positions (77%)", "2,431 positions (23%)"]):
        cols = [ACC, ACC2, ACC3, GREY]
        b = ax.bar(range(4), v, color=cols, width=0.65)
        for r, x in zip(b, v):
            ax.text(r.get_x() + r.get_width() / 2, r.get_height() + 0.4, f"{x:.1f}%", ha="center", fontsize=9)
        ax.set_xticks(range(4)); ax.set_xticklabels(labels, fontsize=8.5)
        ax.set_title(f"the word that came was {t}\n{n}", fontsize=9); ax.spines[["top", "right"]].set_visible(False)
        ax.set_ylabel("top-1")
    axs[0].set_ylim(0, 30); axs[1].set_ylim(0, 5)
    fig.suptitle("held-out Emma, 10,578 positions: where the count has heard, it reads; where it has not, nothing yet beats frequency (docs/137–138)", fontsize=10, y=1.03)
    save(fig, "f25_strata")


# ------------------------------------------------------------- one pointing
def fig_pointing():
    fig, ax = plt.subplots(figsize=(7.2, 3.4))
    cases = ["a new type of\na known task", "a family\nnever seen", "the same family,\none answer shown"]
    v = [75, 15, 61]
    b = ax.bar(range(3), v, color=[ACC2, GREY, ACC], width=0.6)
    for r, x in zip(b, v):
        ax.text(r.get_x() + r.get_width() / 2, x + 1.5, f"{x}%", ha="center", fontsize=10, fontweight="bold")
    ax.set_xticks(range(3)); ax.set_xticklabels(cases, fontsize=9); ax.set_ylim(0, 90)
    ax.set_ylabel("functions that work (248 tasks)"); ax.spines[["top", "right"]].set_visible(False)
    ax.annotate("one row, read the moment it is given", xy=(2, 61), xytext=(1.05, 80), fontsize=9, color=ACC, arrowprops=dict(arrowstyle="->", color=ACC))
    ax.set_title("the code student on Phi-4's 253 answers: learning from one pointing (docs/124)", fontsize=10)
    save(fig, "f25_pointing")


# ------------------------------------------------------------- session that does not grow
def fig_session():
    turns = [25, 100, 400, 2000, 5000]; words = [0.9, 3.5, 14.1, 69.7, 173]
    fig, ax = plt.subplots(figsize=(7.2, 3.4))
    ax.plot(turns, words, "o-", color=ACC2); ax.set_xscale("log"); ax.set_xlabel("turns in the session"); ax.set_ylabel("words told (thousands)", color=ACC2)
    ax2 = ax.twinx(); ax2.plot(turns, [50] * 5, "s-", color=ACC); ax2.set_ylim(0, 60); ax2.set_ylabel("read back, of 50 asked", color=ACC, fontsize=9)
    ax2.text(30, 52.5, "50 of 50 at every length, two seeds; one ask 2–16 ms; nothing summarised", fontsize=8.5, color=ACC)
    ax.spines[["top"]].set_visible(False); ax2.spines[["top"]].set_visible(False)
    ax.set_title("our context tests (docs/129): what is kept grows with what was said;\nwhat is read back (10 needles · 20 keys · 20 updates) does not fall", fontsize=9.5)
    save(fig, "f25_session")


if __name__ == "__main__":
    for f in [fig_family, fig_two_places, fig_cast, fig_levels, fig_capacity, fig_strata, fig_pointing, fig_session]:
        f(); print("made", f.__name__)
