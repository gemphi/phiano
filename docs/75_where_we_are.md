# 75 · Where we are: what works, what is measured, what Phi-4 can and cannot give us

> In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models — Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle (sphere) with a count of how often it has been played out; meaning is a side effect of pairs of words moving under a condition.

## 1. Is the logic working? — by layer, with the measurement that says so

| layer | what it is | works? | the number (source) |
|---|---|---|---|
| `phasor.rs` | a word as 64 channels, two sides (src/tgt), 32-bit evidence, `on(a,b,k)` | **yes** | v6 format; collapse of docs/64 closed; tests |
| `relational.rs` | one pair key: rarest marker between, else offset | **yes** | `pair_key_is_one_definition`; channel key 52/36/13/8× vs offset 59/19/8/3× (docs/74) |
| `trainer` | coupling, repulsion, ranking-only training | **yes** | WikiText-2 directed ranking 15.5× chance; Austen 7.6× (docs/69, papers/15) |
| `Facet::relational_score` / `relational_fit` | the Difference-Engine reading; Liskov substitution | **yes** | +0.70 learned weight; substitution 238 vs 254 (random) (docs/74 §2b) |
| `phinet` operator | FNO on the circle, hinge / mixture objectives | **yes** | WikiText-2 31.9× chance with operator; Austen 16.1×; **PyTorch parity 55/55 checks at 1e-16** (this doc §3) |
| `phinet::hexagram` | 64 spectral gates, novelty on demand | **yes as operations; no as oracle** | next-hexagram prediction 0.9% vs chance 1.6% (docs/74 §3) |
| `learn` | log-linear scorer, Adam tuner, value audit | **yes** | Rust Book 8.44 → 7.28; Austen 12.94 → 10.95 candidate ppl (docs/73, papers/15) |
| `generate.rs` | trigram sampler + learned scorer + operator + gates + goal | **works, and is the ceiling** | long compositions KN ≈ 11 vs her 187: stitched trigrams; goal pull 2/40 → 6/40 (docs/74) |
| `persona` | fingerprints, attribution | **partly** | attribution 64%; sectors bunch 45–49 (open) |
| `phi-cal` | sphere, Möbius, Q64, winding, heat | **yes** | Paper 3; `Phical::quantum` clamp; Q64 double-quantisation loss < 0.01 |
| perplexity harness | mixture LM | **yes, honest** | directed 142.8 vs one-sided 134.2 — the two-sided lexicon still trails on the tail (docs/69) |

Test suite: 219 lib tests in phiano, 15 in phinet, all passing.

**What is broken or unfinished, plainly:** the composer is a trigram sampler — everything downstream of it (long compositions, goal-directed writing, proposals) hits that ceiling, and the fix is to decode from the operator/relational score directly, which is not written. Persona sectors are skewed. The two-sided lexicon loses ~10 perplexity points on the tail to the one-sided one. Three Austen novels are missing.

## 2. Phi-4 in `refs/`: what is actually there

`refs/Phi-4-multimodal-instruct/` is a Git-LFS clone **without the LFS objects**: every `*.safetensors` (5 GB × 3), `tokenizer.json`, the tech-report PDF and the LoRA adapters are 133–135-byte pointer files. What is real: `modeling_phi4mm.py` (118 KB), `configuration_phi4mm.py`, `processing_phi4mm.py`, the speech/vision encoders, the sample fine-tune scripts, `config.json` (hidden 3072, 32 layers, 24 heads, vocab 200,064, context 131,072), and the tokenizer's `vocab.json` (200,019 pieces) + `merges.txt`. `refs/GLM-5.2/` is the same shape (282 pointer shards, ~744B parameters). `refs/phi4_rust_inference.rs` is the PhiCookBook candle example, which downloads Phi-3-mini GGUF from the Hub at run time.

So: **we cannot train from, distil from, or run Phi-4 with what is on disk.** `git lfs pull` in that folder (≈ 11 GB) would fetch the weights; the build machine here cannot reach huggingface.co at all. Even with the weights, a 5.6B model needs a GPU to be a teacher at any pace.

## 3. What we used from Phi-4 today, measured

**The tokenizer** (`src/bpe.rs`, `data/tokenizers/phi4/`): byte-level BPE from the real `vocab.json` + `merges.txt`, with GPT-2's byte table and pre-tokeniser hand-written (the `regex` crate has no look-ahead). Parity against Hugging Face `tokenizers` on the same files: **10,535 of 10,535 sentences of *Emma* identical**, plus the probe sentences (`torch_bpe_parity.py`). Round-trip decode exact. Caveat: Phi-4's own `tokenizer.json` (o200k pre-tokeniser, digits split in threes) is an LFS pointer, so the o200k variant is not reproduced.

**Does it help the relation circle?** (`bin/bpe_lexicon`, Austen, same trainer, same pool floor):

| units | types | pool ≥20 | chance | directed MRR | × chance | hit@10 |
|---|---|---|---|---|---|---|
| words | 10,501 | 1,228 | 0.0063 | 0.0478 | 7.6× | 0.117 |
| Phi-4 pieces | 9,155 | 1,519 | 0.0052 | 0.0416 | 8.0× | 0.082 |

A wash. With 200k pieces the tokenizer barely splits English (1.31 pieces per word on *Emma*), so it does not dissolve the rare-word problem: rare types fall from 61% to 49%, not to zero. The tokenizer is in the machine and correct; it is not a lever on the numbers.

**The architecture source** is a reference for the distillation path (§5), not something to train.

## 4. Phinet ⇄ PyTorch, the Colab, the 3-D docs

`phinet/examples/torch_reference.rs` dumps every operation, every gradient and one Adam step; `phinet/torch/phinet_torch.py` is the PyTorch side; **55 checks, all pass, max |Δ| 9.9e-16**. `phinet/torch/phinet_colab.ipynb` runs the checks and trains the README's operator in torch (MRR 0.073 → 1.000). PhiDoc pages `docs/phinet/08-operations.md` (the complete catalogue), `09-pytorch-colab.md`, `10-three-dimensions.md`; the interactive page `public/phinet/3d/index.html` (three.js bundled; relation torus, Riemann sphere, sixty-four gates, one Fourier layer) on eight real Austen words exported by `bin/word_json`.

## 5. Completing the system — the order

1. **Decode from the operator, not the trigram tables.** Candidates = the pool ranked by `cosine_table(operator(ctx), table)` mixed with `relational_fit`; the learned scorer already has the features. This is the one change every open number waits on.
2. **Persona sectors**: channel-balanced sector instead of centroid Q64.
3. **Phi-4 as a teacher**, once `git lfs pull` has run on a GPU machine: `torch/phinet_torch.py` makes the operator a torch module, so distillation is a torch loop — Phi-4's next-token distribution restricted to Phiano's pool, KL against `softmax(β · cosine_table)`, then `Fno::from_bytes` the result. Not written, because it cannot be run here.
4. The three missing novels.
