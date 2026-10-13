import numpy as np, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Wedge, FancyBboxPatch, FancyArrowPatch, Circle, Rectangle
import colorsys, os

OUT = "/home/claude/papers/fig"
os.makedirs(OUT, exist_ok=True)
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10, "axes.titleweight": "bold"})
INK = "#1f2430"; ACC = "#c8451f"; ACC2 = "#1f6fb2"; ACC3 = "#2e8b57"; GREY = "#9aa0a6"

def save(fig, name):
    fig.savefig(f"{OUT}/{name}.png", dpi=170, bbox_inches="tight", facecolor="white")
    fig.savefig(f"{OUT}/{name}.svg", bbox_inches="tight", facecolor="white")
    plt.close(fig)

def hue(k, n=64, s=0.75, v=0.92):
    return colorsys.hsv_to_rgb(k / n, s, v)

# ---------------------------------------------------------------- fig1 relation circle
def fig_relation_circle():
    fig, ax = plt.subplots(figsize=(7.2, 7.2))
    ax.set_aspect("equal"); ax.axis("off")
    for k in range(64):
        a0 = 90 - k * 360 / 64; a1 = a0 - 360 / 64
        if k < 8: fc, r = ACC, 1.0
        elif k < 32: fc, r = ACC2, 0.93
        else: fc, r = GREY, 0.86
        ax.add_patch(Wedge((0, 0), r, a1, a0, width=0.22, facecolor=fc, edgecolor="white", lw=1.2, alpha=0.9))
        th = np.deg2rad((a0 + a1) / 2)
        if k % 4 == 0:
            ax.text(1.12 * np.cos(th), 1.12 * np.sin(th), str(k), ha="center", va="center", fontsize=8, color=INK)
    # a word: phasor on each channel (random illustrative phases, evidence-weighted length)
    rng = np.random.default_rng(3)
    for k in range(64):
        ph = rng.uniform(0, 2 * np.pi); ev = np.log1p(rng.integers(0, 40) if k < 32 else rng.integers(0, 3))
        th = np.deg2rad(90 - (k + 0.5) * 360 / 64)
        cx, cy = 0.62 * np.cos(th), 0.62 * np.sin(th)
        L = 0.04 + 0.03 * ev
        ax.plot([cx, cx + L * np.cos(ph)], [cy, cy + L * np.sin(ph)], color=hue(k), lw=2.2, solid_capstyle="round")
        ax.add_patch(Circle((cx, cy), 0.012, color=INK))
    ax.text(0, 0.12, "one word =\n64 phasors", ha="center", va="center", fontsize=12, color=INK, weight="bold")
    ax.text(0, -0.12, "z_k = e^{iφ_k}\nevidence_k = ln(1+n_k)", ha="center", va="center", fontsize=8, color=INK)
    
    ax.set_xlim(-1.3, 1.3); ax.set_ylim(-1.3, 1.3)
    ax.text(-1.28, -1.22, "■ 0–7 offset channels (position ±1..±4)", color=ACC, fontsize=9)
    ax.text(-1.28, -1.30, "■ 8–31 marker channels (of, to, and, …; owned)", color=ACC2, fontsize=9)
    ax.text(0.28, -1.22, "■ 32–63 unclaimed (ranking objective moves these)", color=GREY, fontsize=9)
    ax.set_title("The relation circle: the C⁶⁴ carrier of a Phiano word", color=INK)
    save(fig, "f1_relation_circle")

# ---------------------------------------------------------------- fig1 perplexity ladder
def fig_perplexity():
    labels = ["manifold\ncoherence", "function-space\nidentity", "hinge\noperator", "joint\ntraining", "mixture-NLL\noperator", "Kneser-Ney\n(baseline)", "best mixture\nγ=0.1"]
    vals = [176.67, 156.33, 149.30, 134.09, 134.16, 132.07, 125.47]
    cols = [ACC, ACC, ACC2, ACC2, ACC2, GREY, ACC3]
    fig, ax = plt.subplots(figsize=(8.6, 4))
    b = ax.bar(labels, vals, color=cols, width=0.62)
    for r, v in zip(b, vals): ax.text(r.get_x() + r.get_width() / 2, v + 2, f"{v:.1f}", ha="center", fontsize=9, color=INK)
    ax.axhline(132.07, ls="--", color=GREY, lw=1); ax.text(6.45, 133.5, "KN 132.07", ha="right", fontsize=8, color=GREY)
    ax.set_ylim(100, 190); ax.set_ylabel("held-out perplexity (lower is better)")
    ax.set_title("Rust Book corpus, 6,468 sentences: where each stage lands (measured)")
    for s in ["top", "right"]: ax.spines[s].set_visible(False)
    save(fig, "f1_perplexity_ladder")

# ---------------------------------------------------------------- fig1 ranking
def fig_ranking():
    fig, ax = plt.subplots(figsize=(7.5, 3.8))
    x = np.arange(3); w = 0.26
    flat = [2.6, 2.8, 11.6]; evid = [6.7, 18.9, 27.9]; dirr = [15.5, 18.2, 31.9]
    ax.bar(x - w, flat, w, color=GREY, label="flat v1 space (one angle per word)")
    ax.bar(x, evid, w, color="#9cc3e6", label="evidenced, one side (v4)")
    ax.bar(x + w, dirr, w, color=ACC2, label="evidenced, two sides, directed (v6)")
    for i in range(3):
        ax.text(x[i] - w, flat[i] + 0.6, f"{flat[i]}×", ha="center", fontsize=8)
        ax.text(x[i], evid[i] + 0.6, f"{evid[i]}×", ha="center", fontsize=8)
        ax.text(x[i] + w, dirr[i] + 0.6, f"{dirr[i]}×", ha="center", fontsize=8)
    ax.set_xticks(x, ["identity", "pointwise (no FFT)", "Fourier operator"])
    ax.set_ylabel("MRR ÷ chance (pool ≈ 2,200)"); ax.set_ylim(0, 37)
    ax.set_title("WikiText-2 next-word ranking: representation × reader (measured)")
    ax.legend(frameon=False, fontsize=8)
    for s in ["top", "right"]: ax.spines[s].set_visible(False)
    save(fig, "f1_ranking")

# ---------------------------------------------------------------- fig1 learning loop
def box(ax, xy, w, h, text, fc="white", ec=INK, fs=9):
    ax.add_patch(FancyBboxPatch(xy, w, h, boxstyle="round,pad=0.02,rounding_size=0.04", fc=fc, ec=ec, lw=1.4))
    ax.text(xy[0] + w / 2, xy[1] + h / 2, text, ha="center", va="center", fontsize=fs, color=INK)

def arrow(ax, a, b, color=INK, text=None, rad=0.0):
    ax.add_patch(FancyArrowPatch(a, b, arrowstyle="-|>", mutation_scale=14, color=color, lw=1.4, connectionstyle=f"arc3,rad={rad}"))
    if text: ax.text((a[0] + b[0]) / 2, (a[1] + b[1]) / 2 + 0.05, text, ha="center", fontsize=8, color=color)

def fig_learning_loop():
    fig, ax = plt.subplots(figsize=(9, 4.2)); ax.axis("off"); ax.set_xlim(0, 9); ax.set_ylim(0, 4.2)
    box(ax, (0.2, 2.6), 1.7, 1.0, "sentence\n\"the cat sat on the mat\"", fc="#fff4ee")
    box(ax, (2.4, 2.6), 1.9, 1.0, "windows ±4\n+ marker words\n→ relation k per pair", fc="#eef4fb")
    box(ax, (4.8, 2.6), 1.9, 1.0, "Kuramoto step on\nchannel k only\nφ_a[k] ← φ_a[k]+η·sin(φ_b[k]−φ_a[k])", fc="#eef4fb", fs=8)
    box(ax, (7.2, 2.6), 1.6, 1.0, "evidence[k] += 1\n(u16, log-read)", fc="#eefbf2")
    box(ax, (4.8, 0.7), 1.9, 1.0, "same-channel\nnegative: repel a\nrandom word on k", fc="#fbeeee", fs=8)
    box(ax, (7.2, 0.7), 1.6, 1.0, "ranking objective\nmoves unclaimed\nchannels 32–63", fc="#f3f3f3", fs=8)
    box(ax, (0.2, 0.7), 1.9, 1.0, "read: similarity_with\nevidence-weighted\n⟨a,b⟩ = Σ w_k ā_k b_k", fc="#fff4ee", fs=8)
    arrow(ax, (1.9, 3.1), (2.4, 3.1)); arrow(ax, (4.3, 3.1), (4.8, 3.1)); arrow(ax, (6.7, 3.1), (7.2, 3.1))
    arrow(ax, (5.75, 2.6), (5.75, 1.7), color=ACC, text="+ contrast")
    arrow(ax, (8.0, 2.6), (8.0, 1.7), color=GREY)
    arrow(ax, (4.8, 1.2), (2.1, 1.2), color=ACC2, text="what the lexicon now knows")
    ax.text(4.5, 4.0, "How a Phiano lexicon learns: one relation channel at a time", ha="center", fontsize=12, weight="bold", color=INK)
    ax.text(4.5, 0.25, "attraction alone collapsed offset+1 dispersion 0.89→0.32; with the same-channel negative it stays 0.89 (measured)", ha="center", fontsize=8, color=GREY)
    save(fig, "f1_learning_loop")

# ---------------------------------------------------------------- fig1 persona fingerprints
def fig_personas():
    rng = np.random.default_rng(7); n = 64
    fig, axes = plt.subplots(1, 3, figsize=(10, 3.4))
    names = [("Engineer", [2, 3, 20, 21, 22], ACC2), ("Poet", [40, 41, 44, 50, 51], ACC), ("Lawyer", [8, 9, 10, 30, 31], ACC3)]
    for ax, (nm, peaks, c) in zip(axes, names):
        h = rng.gamma(1.2, 0.3, n)
        for p in peaks: h[p] += rng.uniform(2, 4)
        h /= h.sum()
        ax.bar(range(n), h, color=[hue(k) for k in range(n)], width=1.0)
        ax.set_title(f"{nm}: dominant {sorted(peaks)[:3]}…", fontsize=9)
        ax.set_xlabel("relation channel / sector"); ax.set_yticks([])
        for s in ["top", "right", "left"]: ax.spines[s].set_visible(False)
        ent = -(h[h > 0] * np.log(h[h > 0])).sum()
        ax.text(63, h.max() * 0.9, f"entropy {ent:.2f}", ha="right", fontsize=8, color=GREY)
    fig.suptitle("Persona = a histogram over the circle (Fingerprint::extract). Schematic shapes, real formula.", fontsize=10, color=INK)
    save(fig, "f1_personas")

# ---------------------------------------------------------------- fig2 FNO block
def fig_fno():
    fig, ax = plt.subplots(figsize=(10, 3.6)); ax.axis("off"); ax.set_xlim(0, 10); ax.set_ylim(0, 3.6)
    box(ax, (0.1, 1.3), 1.2, 1.0, "context\nfunction\nx ∈ C⁶⁴", fc="#fff4ee")
    box(ax, (1.6, 1.3), 1.1, 1.0, "lift\nW₀x+b₀\n→ width w", fc="#eef4fb")
    # layer
    ax.add_patch(FancyBboxPatch((3.0, 0.3), 4.6, 3.0, boxstyle="round,pad=0.02", fc="#f7f9fc", ec=GREY, lw=1, ls="--"))
    ax.text(5.3, 3.12, "Fourier layer (× L, shared or not)", ha="center", fontsize=9, color=GREY)
    box(ax, (3.2, 1.9), 0.9, 0.8, "FFT\n(unitary)", fc="#eef4fb")
    box(ax, (4.3, 1.9), 1.2, 0.8, "mode mix\nR_m · ĥ_m\nm < modes", fc="#eefbf2")
    box(ax, (5.7, 1.9), 0.9, 0.8, "IFFT", fc="#eef4fb")
    box(ax, (4.3, 0.5), 1.2, 0.8, "pointwise\nW h + b", fc="#fbeeee")
    box(ax, (6.8, 1.3), 0.7, 0.8, "+\nGELU", fc="white")
    box(ax, (7.9, 1.3), 1.0, 1.0, "project\nW_p h\n→ y ∈ C⁶⁴", fc="#eef4fb")
    box(ax, (9.1, 1.3), 0.85, 1.0, "cosine\nvs table\n→ rank", fc="#fff4ee", fs=8)
    arrow(ax, (1.3, 1.8), (1.6, 1.8)); arrow(ax, (2.7, 1.8), (3.2, 2.3), rad=-0.2); arrow(ax, (2.7, 1.8), (4.3, 0.9), rad=0.2)
    arrow(ax, (4.1, 2.3), (4.3, 2.3)); arrow(ax, (5.5, 2.3), (5.7, 2.3)); arrow(ax, (6.6, 2.3), (6.8, 1.9), rad=-0.2)
    arrow(ax, (5.5, 0.9), (6.8, 1.5), rad=0.2); arrow(ax, (7.5, 1.7), (7.9, 1.8)); arrow(ax, (8.9, 1.8), (9.1, 1.8))
    ax.text(5.0, 0.12, "h ← σ( F⁻¹( R · F h ) + W h + b )   —  8,360 parameters at width 4, 2 layers, 32 modes", ha="center", fontsize=9, color=INK)
    save(fig, "f2_fno_block")

# ---------------------------------------------------------------- fig2 Riemann sphere
def fig_riemann():
    fig = plt.figure(figsize=(10, 5))
    ax = fig.add_subplot(121, projection="3d"); ax.set_axis_off()
    u, v = np.mgrid[0:2 * np.pi:40j, 0:np.pi:20j]
    ax.plot_surface(np.cos(u) * np.sin(v), np.sin(u) * np.sin(v), np.cos(v), color="#dfe8f3", alpha=0.35, linewidth=0)
    t = np.linspace(0, 2 * np.pi, 200)
    ax.plot(np.cos(t), np.sin(t), 0 * t, color=INK, lw=1.2)           # equator = unit circle
    for k in range(64):
        th = 2 * np.pi * k / 64
        ax.scatter(np.cos(th), np.sin(th), 0, color=hue(k), s=14)
    # a point in the plane with |z|>1 and its stereographic image
    for z, c in [(1.8 * np.exp(1j * 0.9), ACC), (0.45 * np.exp(1j * 3.6), ACC2)]:
        X, Y = z.real, z.imag; d = 1 + X * X + Y * Y
        P = (2 * X / d, 2 * Y / d, (X * X + Y * Y - 1) / d)
        ax.scatter(X, Y, 0, color=c, s=30, marker="x"); ax.scatter(*P, color=c, s=30)
        ax.plot([0, X], [0, Y], [1, 0], color=c, lw=0.8, ls=":")
        ax.plot([X, P[0]], [Y, P[1]], [0, P[2]], color=c, lw=0.8, ls=":")
    ax.scatter(0, 0, 1, color=INK, s=20); ax.text(0, 0, 1.15, "∞ (north pole)", fontsize=8)
    ax.set_xlim(-1.3, 1.3); ax.set_ylim(-1.3, 1.3); ax.set_zlim(-1.0, 1.1); ax.view_init(22, -55); ax.set_box_aspect((1,1,0.85), zoom=1.6)
    ax.set_title("Ĉ = C ∪ {∞}: the closed home of one channel", fontsize=10)
    ax2 = fig.add_subplot(122); ax2.axis("off"); ax2.set_xlim(0, 1); ax2.set_ylim(0, 1)
    txt = ("Each of the 64 channels lives on the Riemann sphere.\n"
           "• unit circle (equator) = pure phase, the lexicon's carrier\n"
           "• |z| = evidence-scaled amplitude, read as ln(1+n)\n"
           "• 0 and ∞ are the two fixed points of ‘no relation’\n"
           "• Möbius maps z ↦ (az+b)/(cz+d) are the closed\n"
           "  symmetry group: rotation (bind), inversion (unbind),\n"
           "  dilation (evidence), translation (sub-band shift)\n"
           "• 64 spheres × one word = a point of (Ĉ)⁶⁴ ≅ (S²)⁶⁴\n\n"
           "A neural operator on the circle acts on Fourier modes\n"
           "of the *whole word*; a Möbius map acts on one channel.\n"
           "Phinet 2 uses both: operators for learning, Möbius\n"
           "maps for exact, invertible algebra.")
    ax2.text(0, 0.98, txt, va="top", fontsize=9, color=INK, family="DejaVu Sans")
    save(fig, "f2_riemann_sphere")

# ---------------------------------------------------------------- fig2 composition depth
def fig_depth():
    levels = [1, 2, 3, 4]
    series = {"phase subtraction only": ([19, 10, 5, 2], GREY), "cleanup K=1": ([70, 3, 2, 1], "#a9c7e8"),
              "cleanup K=2": ([64, 56, 2, 2], "#5d9bd3"), "cleanup K=3": ([66, 52, 45, 3], ACC2), "curriculum 1→2→3": ([55, 44, 34, 7], ACC)}
    fig, ax = plt.subplots(figsize=(8, 3.9)); x = np.arange(4); w = 0.16
    for i, (nm, (v, c)) in enumerate(series.items()):
        ax.bar(x + (i - 2) * w, v, w, label=nm, color=c)
    ax.set_xticks(x, [f"level {l}" for l in levels]); ax.set_ylabel("recovery MRR ÷ chance (704 words)")
    ax.set_title("Holographic composition: an operator recovers every depth it was shown (measured)")
    ax.legend(frameon=False, fontsize=8, ncol=3); ax.set_ylim(0, 80)
    for s in ["top", "right"]: ax.spines[s].set_visible(False)
    save(fig, "f2_depth")

# ---------------------------------------------------------------- fig2 loss comparison
def fig_loss():
    fig, ax = plt.subplots(figsize=(8, 3.6))
    rows = ["hinge, uniform decoys", "hinge, freq^0.75 decoys", "hinge, freq decoys", "mixture NLL γ_t=1.0", "mixture NLL γ_t=0.5", "mixture NLL γ_t=0.2"]
    alone = [149.30, 153.55, 175.30, 134.16, 136.14, 140.39]; mix = [125.64, 125.64, 125.64, 125.56, 125.49, 125.47]
    y = np.arange(len(rows)); ax.barh(y - 0.18, alone, 0.36, color=ACC2, label="operator alone (γ=1)")
    ax.barh(y + 0.18, mix, 0.36, color=ACC3, label="best mixture with n-gram tables")
    for i in range(len(rows)):
        ax.text(alone[i] + 1, y[i] - 0.18, f"{alone[i]:.1f}", va="center", fontsize=8); ax.text(mix[i] + 1, y[i] + 0.18, f"{mix[i]:.2f}", va="center", fontsize=8)
    ax.set_yticks(y, rows); ax.invert_yaxis(); ax.set_xlim(100, 190); ax.set_xlabel("perplexity")
    ax.axvline(132.07, ls="--", color=GREY, lw=1); ax.text(132.5, -0.6, "Kneser-Ney", fontsize=8, color=GREY)
    ax.legend(frameon=False, fontsize=8, loc="lower right"); ax.set_title("Train on what is read: the objective is worth 15 points, the decoy none (measured)")
    for s in ["top", "right"]: ax.spines[s].set_visible(False)
    save(fig, "f2_loss")

# ---------------------------------------------------------------- fig2 pytorch vs phinet
def fig_pytorch():
    fig, ax = plt.subplots(figsize=(10, 4.6)); ax.axis("off")
    rows = [("unit of data", "real/complex Tensor, any rank", "Tensor [batch, width, space] of C⁶⁴"),
            ("autograd", "dynamic tape, ~2,000 ops", "dynamic Tape, 17 ops, all finite-difference checked"),
            ("complex gradients", "Wirtinger, conj convention", "g = ∂L/∂Re + i·∂L/∂Im (documented, one convention)"),
            ("FFT", "torch.fft, cuFFT/MKL", "own unitary radix-2, power-of-two spaces, differentiable"),
            ("the operator", "you assemble nn.Modules", "Fno is the primitive; layers, modes, width are config"),
            ("ranking losses", "write your own / torch.nn", "hinge, cosine_table, mixture_nll built in"),
            ("training the table", "nn.Embedding + optimizer", "Joint: table as tape parameter with proximal tether"),
            ("composition", "not a primitive", "hrr::bind/unbind/compose on the circle"),
            ("hardware", "CUDA, MPS, XLA, CPU", "CPU only, single core today (parallel cosine_table is next)"),
            ("size", "~ 10⁶ lines, Python + C++", "~ 2,000 lines of Rust, two dependencies"),
            ("persistence", "pickle / safetensors", "to_bytes/from_bytes (bincode), .fno files"),
            ("what it is for", "any differentiable program", "functions on a closed circle, ranked against a table")]
    ax.set_xlim(0, 10); ax.set_ylim(0, len(rows) + 1.2)
    ax.text(0.1, len(rows) + 0.6, "aspect", weight="bold"); ax.text(2.6, len(rows) + 0.6, "PyTorch", weight="bold", color=ACC); ax.text(6.0, len(rows) + 0.6, "phinet", weight="bold", color=ACC2)
    for i, (a, p, q) in enumerate(rows):
        y = len(rows) - i - 0.2
        if i % 2 == 0: ax.add_patch(Rectangle((0, y - 0.35), 10, 0.9, fc="#f4f6f9", ec="none"))
        ax.text(0.1, y, a, fontsize=8.5, weight="bold", color=INK); ax.text(2.6, y, p, fontsize=8.5, color=INK); ax.text(6.0, y, q, fontsize=8.5, color=INK)
    ax.set_title("PyTorch is a language for differentiable programs; phinet is one sentence in it, spoken in Rust", fontsize=10)
    save(fig, "f2_pytorch_vs_phinet")

# ---------------------------------------------------------------- fig3 torus
def fig_torus():
    fig = plt.figure(figsize=(9, 4.4)); ax = fig.add_subplot(121, projection="3d"); ax.set_axis_off()
    R, r = 1.0, 0.38
    u, v = np.mgrid[0:2 * np.pi:80j, 0:2 * np.pi:30j]
    X = (R + r * np.cos(v)) * np.cos(u); Y = (R + r * np.cos(v)) * np.sin(u); Z = r * np.sin(v)
    C = np.array([[hue((uu / (2 * np.pi) * 64) % 64) for uu in row] for row in u])
    ax.plot_surface(X, Y, Z, facecolors=C, alpha=0.85, linewidth=0, shade=True)
    ax.view_init(38, -60); ax.set_xlim(-1.4, 1.4); ax.set_ylim(-1.4, 1.4); ax.set_zlim(-0.8, 0.8)
    ax.set_title("T² = (phase θ) × (band n·α)", fontsize=10)
    ax2 = fig.add_subplot(122); ax2.axis("off"); ax2.set_xlim(0, 1); ax2.set_ylim(0, 1)
    ax2.text(0, 0.98, ("Phical's manifold as implemented (phical/manifold.rs):\n\n"
                       "• major circle: θ ∈ [0,2π) → wavelength 380–750 nm → colour\n"
                       "• minor circle: n·α, α = 1/137, the sub-band ‘quantum’ axis\n"
                       "• geodesic  d = √(Δθ² + φ·(Δn·α)²)   (φ = golden ratio)\n"
                       "• curvature κ = α(n+1)/φ\n"
                       "• standing-wave nodes at multiples of the golden angle\n"
                       "• time: ν = c/λ, ω = 2πν, period T = 1/ν  (chiton)\n\n"
                       "Phical 3 closes the torus into a sphere per channel and\n"
                       "stacks 64 of them: the same θ axis becomes the equator of\n"
                       "channel k's Riemann sphere, and n·α becomes latitude."),
             va="top", fontsize=9, color=INK)
    save(fig, "f3_torus")

# ---------------------------------------------------------------- fig3 C64 / Q64 juncture
def fig_juncture():
    fig, axes = plt.subplots(1, 3, figsize=(11, 3.9))
    ax = axes[0]; ax.set_aspect("equal"); ax.axis("off")
    t = np.linspace(0, 2 * np.pi, 400); ax.plot(np.cos(t), np.sin(t), color=INK, lw=1.5)
    for th in [0.4, 1.9, 3.3, 4.9]:
        ax.plot([0, np.cos(th)], [0, np.sin(th)], color=ACC2, lw=1.5); ax.add_patch(Circle((np.cos(th), np.sin(th)), 0.04, color=ACC2))
    ax.set_title("C64: continuous phase\nz = e^{iφ}, φ ∈ [0, 2π)", fontsize=9); ax.set_xlim(-1.2, 1.2); ax.set_ylim(-1.2, 1.2)
    ax = axes[1]; ax.set_aspect("equal"); ax.axis("off")
    for k in range(64):
        th = 2 * np.pi * k / 64; ax.add_patch(Circle((np.cos(th), np.sin(th)), 0.035, color=hue(k)))
    for k in [4, 19, 34, 50]:
        th = 2 * np.pi * k / 64; ax.plot([0, np.cos(th)], [0, np.sin(th)], color=INK, lw=1.2)
        ax.text(1.22 * np.cos(th), 1.22 * np.sin(th), f"{k:06b}", ha="center", va="center", fontsize=7, family="monospace")
    ax.set_title("Q64: 64 lattice points\nq = round(64·φ/2π) — a 6-bit hexagram", fontsize=9); ax.set_xlim(-1.4, 1.4); ax.set_ylim(-1.4, 1.4)
    ax = axes[2]; ax.axis("off"); ax.set_xlim(0, 1); ax.set_ylim(0, 1)
    ax.text(0, 0.98, ("The juncture (both directions exist in code):\n\n"
                      "quantise   C64 → Q64   SpectralPhasor packs each φ\n"
                      "           into 8 bits (256 quanta); Q64 is the 6-bit\n"
                      "           coarsening: Hexagram::from_phase, sector id\n\n"
                      "lift       Q64 → C64   phase_angle() = 2πq/64,\n"
                      "           sector_to_point, the FNO input\n\n"
                      "Q64 is where symbols live: 64 hexagrams, 64 sectors,\n"
                      "Phinum64 classes, 64 octave layers.\n"
                      "C64 is where learning lives: Kuramoto steps, gradients,\n"
                      "Fourier modes.\n\n"
                      "Round trip error ≤ π/64 per channel; the lattice is the\n"
                      "smallest one whose points are also the 64 relation\n"
                      "channels — the same number names both the alphabet\n"
                      "and the carrier."), va="top", fontsize=8.6, color=INK)
    fig.suptitle("The C64 / Q64 juncture: one circle, read continuously or in 64 symbols", fontsize=10, weight="bold")
    save(fig, "f3_juncture")

# ---------------------------------------------------------------- fig3 topology
def fig_topology():
    fig, ax = plt.subplots(figsize=(10, 3.4)); ax.axis("off"); ax.set_xlim(0, 10); ax.set_ylim(0, 3.4)
    names = ["Vertex\nword", "Edge\nphrase\nlength, coupling", "Path\nsentence\ncoherence, winding", "Surface\nstory\narea, genus", "Region\nconcept\ncentroid, density"]
    rng = np.random.default_rng(1)
    for i, nm in enumerate(names):
        cx = 1 + i * 2.0
        ax.add_patch(FancyBboxPatch((cx - 0.9, 0.2), 1.8, 3.0, boxstyle="round,pad=0.02", fc="#f7f9fc", ec=GREY, lw=1))
        ax.text(cx, 0.55, nm, ha="center", va="center", fontsize=8.5, color=INK)
        if i == 0: ax.add_patch(Circle((cx, 2.2), 0.12, color=ACC))
        if i == 1:
            ax.plot([cx - 0.5, cx + 0.5], [2.0, 2.4], color=ACC2, lw=2); [ax.add_patch(Circle(p, 0.1, color=ACC)) for p in [(cx - 0.5, 2.0), (cx + 0.5, 2.4)]]
        if i == 2:
            xs = cx + np.array([-0.6, -0.2, 0.1, 0.5]); ys = np.array([1.8, 2.5, 2.1, 2.7])
            ax.plot(xs, ys, color=ACC2, lw=2); [ax.add_patch(Circle((x, y), 0.09, color=ACC)) for x, y in zip(xs, ys)]
        if i == 3:
            for j in range(3):
                xs = cx + np.array([-0.6, -0.2, 0.2, 0.6]); ys = 1.7 + j * 0.45 + rng.uniform(-0.1, 0.1, 4)
                ax.plot(xs, ys, color=ACC2, lw=1.5); [ax.add_patch(Circle((x, y), 0.06, color=ACC)) for x, y in zip(xs, ys)]
            ax.plot([cx - 0.2, cx - 0.2], [1.7, 2.6], color=GREY, lw=1, ls=":"); ax.plot([cx + 0.2, cx + 0.2], [1.7, 2.6], color=GREY, lw=1, ls=":")
        if i == 4:
            ax.add_patch(Circle((cx, 2.2), 0.6, fc="#fdeee8", ec=ACC, lw=1.2))
            for _ in range(9): ax.add_patch(Circle((cx + rng.normal(0, 0.25), 2.2 + rng.normal(0, 0.25)), 0.06, color=ACC))
    ax.set_title("phical/topology.rs: linguistic units as simplices on the manifold", fontsize=10)
    save(fig, "f3_topology")

# ---------------------------------------------------------------- fig3 operators grid
def fig_operators():
    fig, ax = plt.subplots(figsize=(10, 6.6)); ax.axis("off"); ax.set_xlim(0, 10); ax.set_ylim(0, 6.6)
    have = [("relax_phase", "Kuramoto step along\nmanifold gradient"), ("spectral_convolution", "wavelength overlap ×\nresonance × coupling"),
            ("temporal_project", "θ → (ω, T)"), ("bind_to_node", "snap to golden-angle\nstanding wave"), ("interference", "√(A_aA_b)·cos Δφ"),
            ("geodesic / curvature", "d on T², κ = α(n+1)/φ")]
    new = [("möbius(a,b,c,d)", "exact per-channel\nrotation/inversion/dilation"), ("stereo / unstereo", "circle ⇄ sphere, evidence\n→ latitude"),
           ("spherical_fno", "operator on S² harmonics\nY_lm, not just e^{ikθ}"), ("laplace_beltrami", "diffusion on the sphere:\nsmooth a persona"),
           ("hexagram_lift / quantise", "Q64 ⇄ C64 juncture\nwith error bound"), ("riemann_zeta_filter", "mode weights 1/m^s:\nthe Riemann prior"),
           ("phase_transport", "parallel-transport a word\nalong a path (sentence)"), ("winding_number", "degree of a path on\nthe circle = a topological\ninvariant of a sentence"),
           ("cross_channel_coupling", "K_{kl} between channels:\nlet ‘of’ talk to ‘to’")]
    ax.text(0.1, 6.35, "Operators Phical has today (phical/operators.rs, manifold.rs)", weight="bold", color=ACC2, fontsize=10)
    for i, (n, d) in enumerate(have):
        x = 0.1 + (i % 3) * 3.3; y = 5.5 - (i // 3) * 1.05
        ax.add_patch(FancyBboxPatch((x, y - 0.4), 3.1, 0.9, boxstyle="round,pad=0.02", fc="#eef4fb", ec=ACC2, lw=1))
        ax.text(x + 0.1, y + 0.3, n, fontsize=8.5, weight="bold", family="monospace", color=INK, va="center"); ax.text(x + 0.1, y - 0.1, d, fontsize=7.5, color=INK, va="center")
    ax.text(0.1, 3.55, "Operators Phical 3 proposes (specified in this paper, not yet in code)", weight="bold", color=ACC, fontsize=10)
    for i, (n, d) in enumerate(new):
        x = 0.1 + (i % 3) * 3.3; y = 2.75 - (i // 3) * 1.1
        ax.add_patch(FancyBboxPatch((x, y - 0.45), 3.1, 0.95, boxstyle="round,pad=0.02", fc="#fdeee8", ec=ACC, lw=1))
        ax.text(x + 0.1, y + 0.3, n, fontsize=8.5, weight="bold", family="monospace", color=INK, va="center"); ax.text(x + 0.1, y - 0.12, d, fontsize=7.2, color=INK, va="center")
    save(fig, "f3_operators")

# ---------------------------------------------------------------- fig3 persona learning
def fig_persona_learning():
    fig, axes = plt.subplots(1, 2, figsize=(10, 3.6))
    ax = axes[0]; e = np.arange(0, 41)
    for nm, k, c in [("Engineer", 0.18, ACC2), ("Poet", 0.11, ACC), ("Lawyer", 0.14, ACC3)]:
        ax.plot(e, 1 - np.exp(-k * e) * (1 + 0.15 * np.sin(e / 2)), color=c, lw=2, label=nm)
    ax.set_xlabel("episodes (sentences fed)"); ax.set_ylabel("fingerprint stability\n(cosine to final)"); ax.set_ylim(0, 1.05)
    ax.legend(frameon=False, fontsize=8); ax.set_title("Schematic: fingerprints settle as evidence accrues", fontsize=9)
    for s in ["top", "right"]: ax.spines[s].set_visible(False)
    ax = axes[1]; ax.axis("off"); ax.set_xlim(0, 1); ax.set_ylim(0, 1)
    ax.text(0, 0.98, ("How the system learns a persona (as coded):\n\n"
                      "1  Persona::from_examples → Trainer::train_sentence\n"
                      "   on every example: Kuramoto steps + evidence\n"
                      "2  Fingerprint::extract: sentence wave → sector,\n"
                      "   plus each content word's sector weighted by\n"
                      "   0.3/(1+amplitude) — rare words count more\n"
                      "3  histogram normalised; entropy = ‘diversity’\n"
                      "4  likelihood(text) = normalised Σ w_i ln p_i\n"
                      "5  Impersonator: RiverFlow from dominant sectors,\n"
                      "   score = 0.4·quality + 0.6·sector fit, retrain on\n"
                      "   the top 8 flows, ≤ 4 rounds\n\n"
                      "Phinet 2 adds: a persona is a *tethered copy of the\n"
                      "table* trained with Joint on that persona's text only."),
            va="top", fontsize=8.6, color=INK)
    save(fig, "f3_persona_learning")

for f in [fig_relation_circle, fig_perplexity, fig_ranking, fig_learning_loop, fig_personas, fig_fno, fig_riemann, fig_depth, fig_loss, fig_pytorch, fig_torus, fig_juncture, fig_topology, fig_operators, fig_persona_learning]:
    f(); print("ok", f.__name__)

# ---------------------------------------------------------------- fig1 collapse by frequency tier (measured)
def fig_collapse():
    tiers = ["top 100", "100–500", "500–2,000", "2,000–6,165"]
    off2 = [0.003, 0.026, 0.723, 0.890]; off4 = [0.003, 0.016, 0.609, 0.770]
    of2 = [0.495, 0.826, 0.947, 0.978]; of4 = [0.315, 0.670, 0.895, 0.980]
    fig, ax = plt.subplots(figsize=(8.4, 3.8)); x = np.arange(4); w = 0.2
    ax.bar(x - 1.5 * w, off2, w, color=ACC, label="offset+1, 2 passes"); ax.bar(x - 0.5 * w, off4, w, color="#e8a48f", label="offset+1, 4 passes")
    ax.bar(x + 0.5 * w, of2, w, color=ACC2, label="'of' marker, 2 passes"); ax.bar(x + 1.5 * w, of4, w, color="#9cc3e6", label="'of' marker, 4 passes")
    ax.set_xticks(x, tiers); ax.set_xlabel("words ranked by frequency"); ax.set_ylabel("circular dispersion (1 = spread, 0 = one angle)")
    ax.set_ylim(0, 1.05); ax.legend(frameon=False, fontsize=8, ncol=2)
    ax.set_title("Where the collapse lives: frequent words share one angle on the offset channels (measured)")
    for s in ["top", "right"]: ax.spines[s].set_visible(False)
    save(fig, "f1_collapse")

# ---------------------------------------------------------------- fig1 the paradigm
def fig_paradigm():
    fig, ax = plt.subplots(figsize=(10, 4.6)); ax.axis("off"); ax.set_xlim(0, 10); ax.set_ylim(0, 4.6)
    ax.text(5, 4.3, "X as Y:  a word is nothing but what it relates to, under what conditions", ha="center", fontsize=12.5, weight="bold", color=INK)
    # left: the old view
    ax.add_patch(FancyBboxPatch((0.2, 0.5), 2.6, 3.3, boxstyle="round,pad=0.02", fc="#f7f7f7", ec=GREY, lw=1))
    ax.text(1.5, 3.5, "word as a point", ha="center", fontsize=10, weight="bold", color=GREY)
    ax.add_patch(Circle((1.5, 2.1), 0.16, color=GREY)); ax.text(1.5, 1.7, "x ∈ R^d", ha="center", fontsize=9, color=GREY)
    ax.text(1.5, 0.85, "meaning = position\ncontext = nearby points", ha="center", fontsize=8, color=GREY)
    # middle: word as polygon of relations
    ax.add_patch(FancyBboxPatch((3.2, 0.5), 3.4, 3.3, boxstyle="round,pad=0.02", fc="#fff4ee", ec=ACC, lw=1.2))
    ax.text(4.9, 3.5, "word as a polygon of relations", ha="center", fontsize=10, weight="bold", color=ACC)
    cx, cy = 4.9, 2.05; rels = ["next", "of", "to", "that", "and", "is", "in", "…"]
    for i, r in enumerate(rels):
        th = np.pi / 2 + i * 2 * np.pi / len(rels); L = 0.55 + 0.25 * ((i * 7) % 3) / 2
        ax.plot([cx, cx + L * np.cos(th)], [cy, cy + L * np.sin(th)], color=hue(i * 8), lw=2.5)
        ax.text(cx + (L + 0.22) * np.cos(th), cy + (L + 0.22) * np.sin(th), r, ha="center", va="center", fontsize=7.5, color=INK)
    ax.add_patch(Circle((cx, cy), 0.07, color=INK))
    ax.text(4.9, 0.8, "each side: which relation (channel k)\nat what angle (φ_k), seen how often (n_k)", ha="center", fontsize=8, color=INK)
    # right: the dance
    ax.add_patch(FancyBboxPatch((7.0, 0.5), 2.8, 3.3, boxstyle="round,pad=0.02", fc="#eef4fb", ec=ACC2, lw=1.2))
    ax.text(8.4, 3.5, "meaning as a dance", ha="center", fontsize=10, weight="bold", color=ACC2)
    t = np.linspace(0, 2 * np.pi, 200)
    ax.plot(8.4 + 0.7 * np.cos(t), 2.05 + 0.7 * np.sin(t), color=GREY, lw=0.8, ls=":")
    for i, (a, c) in enumerate([(0.3, ACC), (1.1, ACC2), (2.0, ACC3), (2.9, "#9b59b6")]):
        ax.add_patch(Circle((8.4 + 0.7 * np.cos(a), 2.05 + 0.7 * np.sin(a)), 0.09, color=c))
        ax.add_patch(FancyArrowPatch((8.4 + 0.7 * np.cos(a), 2.05 + 0.7 * np.sin(a)), (8.4 + 0.7 * np.cos(a + 0.45), 2.05 + 0.7 * np.sin(a + 0.45)), arrowstyle="-|>", mutation_scale=9, color=c, lw=1.2, connectionstyle="arc3,rad=0.3"))
    ax.text(8.4, 0.8, "conditions (a sentence) couple the\nsides that are in play; angles move\ntoward each other: dφ/dt = sin(Δφ)", ha="center", fontsize=8, color=INK)
    save(fig, "f1_paradigm")

fig_collapse(); fig_paradigm(); print("extra ok")
