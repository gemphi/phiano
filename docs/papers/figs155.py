"""docs/155 and papers 28–35: the story as a forest of trees.

Every figure is drawn from data/forest/forest.json (written by `bin/forest`)
and data/forest/strand/levels.tsv (written by `bin/strand`) — nothing here is
invented. Palette validated for light surface #fcfcfb; every mark carries its
own label, since two of the six are under 3:1 contrast.

    python3 docs/papers/figs155.py          # all
    python3 docs/papers/figs155.py forest   # one
"""
import json
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Circle, FancyArrowPatch, FancyBboxPatch, Rectangle

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "..", "..", "data", "forest")
FIG = os.path.join(HERE, "fig")
INK, INK2, GRID, SURF = "#0b0b0b", "#52514e", "#e6e5e1", "#fcfcfb"
BLUE, ORANGE, AQUA, VIOLET, GOLD, ROSE = "#2a78d6", "#eb6834", "#1baf7a", "#7a4fd0", "#c9a227", "#c2436b"
PRIOR = "#a3a29c"
plt.rcParams.update(
    {
        "font.family": "DejaVu Sans",
        "font.size": 10,
        "axes.edgecolor": "#8a8984",
        "axes.labelcolor": INK2,
        "xtick.color": INK2,
        "ytick.color": INK2,
    }
)

D = json.load(open(os.path.join(DATA, "forest.json")))


def board(w, h, cols=1):
    fig, axs = plt.subplots(1, cols, figsize=(w, h))
    fig.patch.set_facecolor(SURF)
    for ax in [axs] if cols == 1 else axs:
        ax.set_facecolor(SURF)
        ax.axis("off")
    return fig, axs


def save(fig, name, note):
    fig.text(0.005, -0.02, note, fontsize=7.6, color=INK2)
    fig.tight_layout()
    fig.savefig(os.path.join(FIG, name), dpi=170, bbox_inches="tight", facecolor=SURF)
    print(name)


def plain(ax, grid="y"):
    ax.set_facecolor(SURF)
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)
    ax.grid(axis=grid, color=GRID, lw=0.8)
    ax.set_axisbelow(True)


def wrap(text, n, prefix=""):
    """`text` broken onto lines of at most `n` characters, the first prefixed."""
    out, line = [], prefix
    for w in text.split():
        if len(line) + len(w) + 1 > n and line.strip() != prefix.strip():
            out.append(line)
            line = w
        else:
            line = (line + " " + w).strip() if line.strip() else prefix + w
    out.append(line)
    return "\n".join(out[:4])


def chip(ax, x, y, w, h, text, color, sub=None, fs=8.6):
    ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0.02,rounding_size=0.12", fc=SURF, ec=color, lw=1.8))
    ax.text(x + w / 2, y + h * (0.62 if sub else 0.5), text, ha="center", va="center", fontsize=fs, color=INK)
    if sub:
        ax.text(x + w / 2, y + h * 0.26, sub, ha="center", va="center", fontsize=fs - 1.4, color=INK2)


def arrow(ax, a, b, color, ls="-", lw=1.6):
    ax.add_patch(FancyArrowPatch(a, b, arrowstyle="-|>", mutation_scale=11, color=color, lw=lw, linestyle=ls))


# ── 1. the forest, and a ray cast through it ────────────────────────────────
def fig_forest():
    fig, ax = board(12.4, 6.0)
    ax.set_xlim(0, 24)
    ax.set_ylim(0, 12)
    ax.set_title("The forest: two roots, and six words cast down one of them", loc="left", fontsize=11, color=INK)
    sc = D["scopes"]
    cast = sc["casts"][0]
    path = cast["found"][0]["path"]
    chip(ax, 9.0, 10.2, 6.0, 1.3, "scope:text", BLUE, "the forest floor · %d paragraphs" % D["language"]["paragraphs"])
    chip(ax, 2.0, 7.8, 8.0, 1.3, "scope:text/emma", BLUE, "%d paragraphs told" % sc["emma_paragraphs"])
    chip(ax, 14.0, 7.8, 8.0, 1.3, "scope:text/story", ORANGE, "this story, 1 paragraph")
    arrow(ax, (12.0, 10.2), (6.0, 9.1), BLUE)
    arrow(ax, (12.0, 10.2), (18.0, 9.1), ORANGE)
    # Emma's chapters, with the one the cast entered marked.
    hit = path[2].split("/")[-1] if len(path) > 2 else ""
    for i, ch in enumerate(["ch01", "ch02", "ch03", hit or "ch04", "…", "ch55"]):
        x = 1.2 + i * 1.5
        on = ch == hit
        chip(ax, x, 5.4, 1.35, 1.0, ch, AQUA if on else PRIOR, fs=7.6)
        arrow(ax, (6.0, 7.8), (x + 0.67, 6.4), AQUA if on else GRID, lw=1.8 if on else 0.8)
    leaf = path[3].split("/")[-1] if len(path) > 3 else ""
    chip(ax, 3.3, 3.0, 3.6, 1.0, leaf, AQUA, "the paragraph it entered", fs=8)
    arrow(ax, (5.1, 5.4), (5.1, 4.0), AQUA)
    chip(ax, 15.5, 5.4, 5.0, 1.0, "ch2/p1", ORANGE, fs=8)
    arrow(ax, (18.0, 7.8), (18.0, 6.4), ORANGE)
    # The ray.
    ax.add_patch(FancyArrowPatch((21.6, 1.1), (6.9, 3.4), arrowstyle="-|>", mutation_scale=14, color=VIOLET, lw=2.2))
    ax.text(21.8, 0.6, '"%s"' % cast["words"], ha="right", fontsize=9.2, color=VIOLET)
    ax.text(7.2, 2.2, "%d of its conditions said along the path,\n%d by this paragraph alone" % (cast["found"][0]["said"], cast["found"][0]["own"]), fontsize=8.4, color=INK)
    ax.text(0.2, 0.4, "A node holds a distribution, not a box. The test is D4's movement in nats; the pruning is the beam.", fontsize=8.6, color=INK2)
    save(fig, "f155_forest.png", "bin/forest · the first of %d casts, each six words of the story cast into Emma alone (docs/155)." % len(sc["casts"]))


# ── 2. the sixteen trees ────────────────────────────────────────────────────
FAMILY = [
    ("drawn through", BLUE, [1, 9, 16]),
    ("built by splitting", ORANGE, [2, 3, 8]),
    ("read at every level", AQUA, [4, 5, 6, 13, 15]),
    ("the condition itself", VIOLET, [7, 12]),
    ("neighbourhood, not nesting", ROSE, [10, 11]),
    ("the machine's own", GOLD, [14]),
]


def fig_trees():
    fig, ax = board(12.4, 8.4)
    ax.set_xlim(0, 24)
    ax.set_ylim(-0.4, 16.6)
    ax.set_title("The sixteen trees, and where a BVH sits among them", loc="left", fontsize=11, color=INK)
    color = {}
    for _, c, ids in FAMILY:
        for i in ids:
            color[i] = c
    for i, t in enumerate(D["trees"]):
        col, row = i % 4, i // 4
        x, y = 0.3 + col * 6.0, 12.2 - row * 3.4
        n = i + 1
        chip(ax, x, y, 5.6, 2.9, "", color[n])
        ax.text(x + 0.3, y + 2.42, "%d" % n, fontsize=9, color=color[n], weight="bold")
        ax.text(x + 0.95, y + 2.42, t["name"].split("· ")[1], fontsize=9.2, color=INK)
        ax.text(x + 0.3, y + 1.96, wrap(t["node"], 42, "node: "), fontsize=7.0, color=INK2, va="top")
        ax.text(x + 0.3, y + 1.02, wrap(t["ray"], 42, "ray: "), fontsize=7.0, color=INK2, va="top")
        m = t["measured"]
        if m:
            k, v = next((k, v) for k, v in m.items() if not isinstance(v, list))
            ax.text(x + 5.3, y + 0.2, "%s %s" % (v, k.replace("_", " ")), fontsize=7.4, color=color[n], ha="right")
    for j, (label, c, ids) in enumerate(FAMILY):
        x, y = 0.3 + (j % 3) * 8.0, 1.0 - (j // 3) * 0.7
        ax.add_patch(Rectangle((x, y), 0.45, 0.28, fc=c, ec=SURF))
        ax.text(x + 0.65, y + 0.14, "%s (%s)" % (label, ", ".join(str(i) for i in ids)), fontsize=7.8, color=INK, va="center")
    save(fig, "f155_trees.png", "Every entry names its node, its child relation and its ray; the colour is the family, the number the entry in docs/155 §2.")


# ── 3. the strand: sixty-four levels of one story ───────────────────────────
def fig_strand():
    rows = [l.split("\t") for l in open(os.path.join(DATA, "strand", "levels.tsv")).read().strip().split("\n")[1:]]
    units = [int(r[3]) for r in rows]
    nats = [float(r[4]) for r in rows]
    fig, axs = plt.subplots(1, 2, figsize=(12.4, 4.0))
    fig.patch.set_facecolor(SURF)
    for ax in axs:
        plain(ax)
    axs[0].plot(range(len(units)), units, color=BLUE, lw=2)
    axs[0].set_yscale("log")
    axs[0].set_xlabel("level k (TTT = 63, the whole story; AAA = 0, the word)", fontsize=8.6)
    axs[0].set_ylabel("units at this level", fontsize=8.6)
    axs[0].set_title("a. One story at sixty-four resolutions", loc="left", fontsize=10.2, color=INK)
    for k, lab in [(63, "TTT 1 unit"), (52, "TCA 12"), (32, "ATT 32"), (0, "AAA %d" % units[-1])]:
        i = 63 - k
        axs[0].annotate(lab, (i, units[i]), textcoords="offset points", xytext=(6, 6), fontsize=7.8, color=INK)
    axs[1].plot(range(len(nats)), nats, color=ORANGE, lw=2)
    axs[1].axhline(0, color=PRIOR, lw=1.2, ls=(0, (4, 3)))
    axs[1].set_xlabel("level k", fontsize=8.6)
    axs[1].set_ylabel("nats gained over the level above", fontsize=8.6)
    axs[1].set_title("b. What each division buys", loc="left", fontsize=10.2, color=INK)
    best = min(range(len(nats)), key=lambda i: nats[i])
    axs[1].annotate("%s, %d units" % (rows[best][1], units[best]), (best, nats[best]), textcoords="offset points", xytext=(8, -4), fontsize=7.8, color=ORANGE)
    save(fig, "f155_strand.png", "bin/strand on this story with Emma as the prior: the cut tree pruned to m(k) units, docs/142–145.")


# ── 4. heard and never heard ────────────────────────────────────────────────
def fig_words():
    ws = [w for w in D["words"] if w["word"].isalpha()]
    heard = sorted([w for w in ws if w["heard_in_emma"] > 0], key=lambda w: -w["heard_in_emma"])
    new = [w for w in ws if w["heard_in_emma"] == 0]
    fig, axs = plt.subplots(1, 2, figsize=(12.4, 5.2), gridspec_kw={"width_ratios": [1.5, 1]})
    fig.patch.set_facecolor(SURF)
    plain(axs[0], grid="x")
    show = heard[:9] + heard[-9:]
    y = range(len(show))
    axs[0].barh(list(y), [w["heard_in_emma"] for w in show][::-1], height=0.62, color=BLUE, edgecolor=SURF, lw=2)
    axs[0].set_yticks(list(y))
    axs[0].set_yticklabels([w["word"] for w in show][::-1], fontsize=8)
    axs[0].set_xscale("log")
    axs[0].set_xlabel("times Emma says the word", fontsize=8.6)
    axs[0].set_title("a. What the language had already heard", loc="left", fontsize=10.2, color=INK)
    for i, w in enumerate(show[::-1]):
        axs[0].text(w["heard_in_emma"] * 1.1, i, "%d · post %.2f nats" % (w["heard_in_emma"], w["post_nats"]), va="center", fontsize=7.4, color=INK)
    plain(axs[1], grid="x")
    axs[1].barh(range(len(new)), [1] * len(new), height=0.62, color=ROSE, edgecolor=SURF, lw=2)
    axs[1].set_yticks(range(len(new)))
    axs[1].set_yticklabels([w["word"] for w in new], fontsize=8)
    axs[1].set_xticks([])
    axs[1].set_xlim(0, 2.6)
    axs[1].set_title("b. The %d words it never heard" % len(new), loc="left", fontsize=10.2, color=INK)
    for i, w in enumerate(new):
        fits = (w["never_heard"] or {}).get("both_sides", [])
        best = fits[0][0] if fits else "—"
        axs[1].text(1.08, i, "reads as “%s”" % best, va="center", fontsize=7.4, color=INK)
    save(fig, "f155_words.png", "bin/forest, Emma told without this story: `says` counts, D2's post in nats, and the best word between the two sides.")


# ── 5. the relation circle ──────────────────────────────────────────────────
def fig_phasor():
    ents = {e["word"]: e for e in D["world"]["entities"]}
    pick = [w for w in ["love", "heart", "machine", "his", "that", "business"] if w in ents]
    fig, axs = board(12.4, 4.6, cols=2)
    ax = axs[0]
    ax.set_xlim(-1.5, 1.5)
    ax.set_ylim(-1.5, 1.5)
    ax.set_aspect("equal")
    ax.set_title("a. A word has no position — only angles", loc="left", fontsize=10.2, color=INK)
    ax.add_patch(Circle((0, 0), 1.0, fc="none", ec="#8a8984", lw=1))
    cols = [BLUE, ORANGE, AQUA, VIOLET, GOLD, ROSE]
    import math

    for i, w in enumerate(pick):
        ch = {c["kind"]: c for c in ents[w]["channels"]}
        c = ch.get("before") or list(ch.values())[0]
        th = c["theta"]
        r = 0.45 + 0.5 * min(1.0, c["amplitude"] / 3.0)
        ax.add_patch(FancyArrowPatch((0, 0), (r * math.cos(th), r * math.sin(th)), arrowstyle="-|>", mutation_scale=11, color=cols[i], lw=2))
        ax.text(1.12 * math.cos(th), 1.12 * math.sin(th), w, fontsize=8.6, color=cols[i], ha="center", va="center")
    ax.text(0, -1.42, "channel `before`: the angle is which relation, the length ln(1+evidence)", fontsize=8, color=INK2, ha="center")
    ax = axs[1]
    pairs = D["world"]["resonant_pairs"][:8]
    ax.axis("on")
    plain(ax, grid="x")
    ax.barh(range(len(pairs)), [p["resonance"] for p in pairs][::-1], height=0.6, color=VIOLET, edgecolor=SURF, lw=2)
    ax.set_yticks(range(len(pairs)))
    ax.set_yticklabels(["%s · %s" % (p["a"], p["b"]) for p in pairs][::-1], fontsize=8)
    ax.set_xlim(0, 1.08)
    ax.set_xlabel("resonance: the cosine over every channel both have moved", fontsize=8.4)
    ax.set_title("b. Who ends up near whom", loc="left", fontsize=10.2, color=INK)
    for i, p in enumerate(pairs[::-1]):
        ax.text(p["resonance"] + 0.012, i, "%.3f" % p["resonance"], va="center", fontsize=7.6, color=INK)
    save(fig, "f155_phasor.png", "bin/forest: the story told to a phiecs world — %d words, %d edges, %d relations under a condition." % (D["world"]["words"], D["world"]["edges"], D["world"]["under"]))


# ── 6. the readers, forward and backwards ───────────────────────────────────
def fig_waves():
    rs = D["waves"]["readers"]
    fig, axs = plt.subplots(1, 2, figsize=(12.4, 4.0))
    fig.patch.set_facecolor(SURF)
    for ax in axs:
        plain(ax)
    for ax, key, title in [(axs[0], "forward", "a. Forward: which word comes next"), (axs[1], "backward", "b. Backwards, by the conjugate filter — never trained")]:
        for j, r in enumerate(rs):
            xs = [p["at"] for p in r["positions"]]
            ys = [(p[key]["rank"] if p[key]["rank"] is not None else 2000) + 1 for p in r["positions"]]
            ax.plot(xs, ys, lw=1.2, color=[BLUE, ORANGE][j], marker="o", ms=2.6, label=r["reader"])
        ax.set_yscale("log")
        ax.set_ylabel("rank of the word the story actually says", fontsize=8.4)
        ax.set_xlabel("position in the story", fontsize=8.6)
        ax.set_title(title, loc="left", fontsize=10.2, color=INK)
        ax.axhline(1, color=PRIOR, lw=1.2, ls=(0, (4, 3)))
        ax.text(1, 1.15, "first", fontsize=7.6, color=INK)
    axs[0].legend(frameon=False, fontsize=7.8, loc="upper right")
    note = " · ".join("%s: %d of %d first forward, %d backwards" % (r["reader"], r["top1_forward"], r["asked"], r["top1_backward"]) for r in rs)
    save(fig, "f155_waves.png", "bin/forest on this story, operators trained on Emma alone (docs/152–154). " + note)


# ── 7. the BVH correspondence ───────────────────────────────────────────────
def fig_bvh():
    fig, axs = board(12.4, 4.8, cols=2)
    ax = axs[0]
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 8)
    ax.set_aspect("equal")
    ax.set_title("a. A bounding-volume hierarchy", loc="left", fontsize=10.2, color=INK)
    boxes = [(0.6, 1.2, 8.8, 5.6, BLUE, "root"), (1.0, 1.6, 3.8, 4.8, AQUA, "left"), (5.4, 2.4, 3.6, 3.4, PRIOR, "right"), (1.4, 2.0, 1.6, 1.6, VIOLET, "leaf")]
    for x, y, w, h, c, lab in boxes:
        ax.add_patch(Rectangle((x, y), w, h, fc="none", ec=c, lw=1.8, ls="-" if c != PRIOR else (0, (3, 3))))
        ax.text(x + 0.12, y + h - 0.32, lab, fontsize=7.6, color=c)
    ax.add_patch(FancyArrowPatch((0.2, 7.4), (2.4, 2.6), arrowstyle="-|>", mutation_scale=12, color=ORANGE, lw=2))
    ax.text(2.6, 2.3, "a ray", fontsize=8.4, color=ORANGE)
    ax.text(0.6, 0.5, "hit the box, test the children; miss, skip the subtree", fontsize=8, color=INK2)
    ax = axs[1]
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 8)
    ax.set_title("b. The same walk over conditions", loc="left", fontsize=10.2, color=INK)
    rows = [
        ("volume", "a scope's distribution over what it says"),
        ("contains", "a scope says everything its subtree says"),
        ("the ray", "a few words and their pairs"),
        ("hit test", "D4: does this child say them more than the whole tree?"),
        ("pruning", "the beam — the two best children, the rest dropped"),
        ("leaf", "the paragraph the words are read under"),
        ("miss", "no scope is entered: the reading is the language's"),
    ]
    for i, (a, b) in enumerate(rows):
        y = 6.9 - i * 0.95
        ax.text(0.2, y, a, fontsize=8.6, color=VIOLET, weight="bold")
        ax.text(2.2, y, b, fontsize=8.4, color=INK)
    save(fig, "f155_bvh.png", "phiplm::relate::tree — the correspondence the module's own docs name, written out.")


# ── 8. what fills a word the language never heard ───────────────────────────
def fig_slot():
    new = [w for w in D["words"] if w["heard_in_emma"] == 0 and w["never_heard"]]
    pick = [w for w in new if w["word"] in ("threshold", "ego", "launching", "machine", "outcomes", "loveful")] or new[:6]
    fig, ax = board(12.4, 5.4)
    ax.set_xlim(0, 24)
    ax.set_ylim(0, 13)
    ax.set_title("A word the language never heard, read from both of its sides", loc="left", fontsize=11, color=INK)
    toks = D["story"]["tokens"]
    for i, w in enumerate(pick[:6]):
        y = 11.0 - i * 1.85
        at = w["at"]
        left = " ".join(toks[max(0, at - 3):at])
        right = toks[at + 1] if at + 1 < len(toks) else "—"
        ax.text(0.2, y + 0.5, "… " + left, fontsize=8.6, color=INK2, ha="left")
        chip(ax, 6.4, y, 3.0, 1.1, w["word"], ROSE, "never heard", fs=9)
        ax.text(9.8, y + 0.5, right + " …", fontsize=8.6, color=INK2)
        for j, (word, score) in enumerate((w["never_heard"]["both_sides"] or [])[:3]):
            chip(ax, 12.8 + j * 3.6, y, 3.3, 1.1, word, [VIOLET, AQUA, PRIOR][j], "%.1f" % score, fs=8.4)
    ax.text(12.8, 0.4, "what fits between the two sides: ln p(v | everything said) + ln p(the next word | v)", fontsize=8.4, color=INK2)
    ax.text(0.2, 0.4, "the slot", fontsize=8.4, color=ROSE)
    save(fig, "f155_slot.png", "bin/forest: Emma told without this story, so each of these %d words is genuinely new to it." % len(new))


FIGS = {"forest": fig_forest, "trees": fig_trees, "strand": fig_strand, "words": fig_words, "phasor": fig_phasor, "waves": fig_waves, "bvh": fig_bvh, "slot": fig_slot}

if __name__ == "__main__":
    os.makedirs(FIG, exist_ok=True)
    for name in sys.argv[1:] or FIGS:
        FIGS[name]()
