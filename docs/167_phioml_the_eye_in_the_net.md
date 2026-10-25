# 167 · phi-oml brought into phinet: the eye in the net

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. What was found at `phi/crates/phi-oml` and `phi/crates/macos/src/core_ml`

`phi-oml` (lib `phi_ml`, 19,193 lines) is the phi workspace's platform-agnostic ML framework — its own README says it "replaces all Apple-proprietary dependencies (CoreML, MLCompute, Metal) with portable Rust equivalents". Inside: **PhiFly**, the active-vision resonance engine (`AOCSignal` = phase + amplitude, interference by complex addition; `cpu_resonance` = one complex multiply by a micro-jitter, O(N), twelve flops a signal, "phase rotation, no backprop"); **VeeNux**, a symbolic spatial planner (killer-heuristic, corridor search); a **Quantum Semantic Engine** over `phi/lib/phical`'s `qnt::{PhiTon, Q64, Gate, QueBit}`; an **nlp** module whose types are Noun, Verb, Adjective … and **Set, Union, Difference** — the set game again; an inference pipeline with `learn_association` / `query_semantic`; wgpu/WGSL compute; a portable training graph. `macos/src/core_ml` is the thin Apple-facing bridge: `MLModelConfiguration`, `MLComputeUnits` (CpuOnly / CpuAndGpu / **NeuralEngineOnly** / All), a `Resonance.metal` shader, and re-exports of phi-oml — the shape of CoreML with phi-oml as the engine behind it.

Note the name collision: `phi/lib/phical` and `phinet/crates/phical` are different crates sharing a name and a `Q64`. The port below avoids the phi workspace entirely, as asked — phinet uses no phi crates.

## 2. What Apple uses its ML for

CoreML is Apple's on-device inference runtime: models compiled to run on the CPU, GPU or the **Neural Engine**, never in the cloud by default. What runs on it is mostly **sensors and language in small, constant loops**: Vision's face, body and hand tracking (the very frameworks phikit reads as rows); FaceID's depth matching; camera semantics (subject lift, text in images); keyboard prediction and dictation; Siri's intent matching and, since 2026, Apple Foundation Models with Gemini for the new Siri (docs/161's SIRI.md); photo search; handwriting. The pattern worth copying is not a model — it is **many small models at the edge of the sensors, each the size of its job**, fed by exactly the tracking streams phikit models. phi-oml's PhiFly is the same pattern taken further: vision done with phases instead of weights.

## 3. What is applicable, and what was ported today

PhiFly's `AOCSignal` **is one channel of our phasor** — polar phase+amplitude, interference by complex addition — and its resonance is the complex multiply every operator here is made of. So the port is small and honest: **`crates/phioml`** (phinet's own, `phinet::oml`, no dependency on the phi workspace):

- `signal.rs` — [`Aoc`]: `new`, `seeded` (the one seed of docs/159, so a signal is born where the word's phasor is), `to_complex` on `phistd::c64`, `interfere`, `superpose`, `resonance(jitter)` = phi-oml's `cpu_resonance` as one multiply and one `arg()`; `jitter(angle)` a unit rotation. phi-oml's f32/glam pair became `c64`; the math is identical.
- `flow.rs` — [`Flow`]: the flow field. A scene is named signals; a **twitch** (one micro-jitter) gives each thing a phase shift scaled by amplitude — parallax, `d ∝ 1/Δx`, the near moves fast — binned into near/mid/far against the field's own median. The new step, ours: **`Flow::rows`** tells the field as rows (`hand within scope:sight`, `hand does near (weight = shift)`, `scope:sight says hand`), so a scene enters a `World` by the same door as a sentence, a kit, or a quest. Tested down to `World::from_rows`.
- `phinet::sys::eye::see(agent, flow, angle)` — seeing is telling: the twitch's rows go through `Agent::tell`, honouring the persona's `context` (the stage ring's dancers, whose context watches the body, can watch sight the same way). Tested: after one twitch, `(ask dancer hand does)` answers `near`.

Not ported, deliberately: VeeNux (a planner — phiecs already has `plan`/`reach`, currently in the dead-code review), the wgpu backend (nothing here needs a GPU yet), the training graph (phistd's tape is the one autograd), and phi-oml's nlp types (Noun/Verb/Set/Union — phinet's sets algebra and WordNet tables already are that, with counts).

## 4. Why this matters to the continuous learner

The loop the whole project has been building toward closes one more sensor: prose tells rows, code tells rows, the kit tells rows, the quest tells rows — and now **sight tells rows**, with the weight carried by parallax rather than by count alone. An agent that moves (a twitch is the cheapest movement there is) generates its own data, which is phi-oml's thesis and Wittgenstein's: use first, meaning after. And because `Aoc` lives on the same `c64` and the same seed as every phasor, what the eye computes can `couple` straight into what the language holds — the convergence docs/159 asked for, met from the vision side first.

## 5. Status

Done 2026-10-24: `crates/phioml` (signal + flow, 2 tests), `sys::eye` (1 test), facade `phinet::oml`, workspace 393 tests / 0 warnings / every file under 200 lines. Next: the `SpectralPhasor` ↔ `phiecs::Phasor` convergence with `Aoc` as the shared channel form; a `Flow` fed from phikit's real track rows (the Body agent); BCPL reader (docs/166).
