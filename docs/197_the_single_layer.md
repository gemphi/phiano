# 197 · The single layer — what Apple's architecture teaches the brain

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## How Apple makes many utilities run on one ML layer

Extracted from Apple's own foundation-models write-up and the analyses around it ([Apple](https://machinelearning.apple.com/research/introducing-apple-foundation-models), [DEV](https://dev.to/amarrella/the-aiml-concepts-behind-apple-intelligence-adm), [McGuinness](https://patmcguinness.substack.com/p/apple-gets-intelligence), [Everhem](https://development.everhem.com/architecture-apple-intelligence-quantifying-siris-pivot-agentic-ai)):

1. **One base model, many adapters.** A single ~3B on-device foundation model serves every feature — summarization, mail replies, notification priority, writing tools — not as separate models but as small LoRA adapters swapped onto the one base per task. The base is never retrained for a feature; a feature is a thin, cheap specialization loaded when its moment comes.
2. **A declared action vocabulary.** App Intents expose what every app can do as typed, named actions; Siri does not understand apps, it reads a vocabulary the apps declared. Intelligence is routed through structure that was TOLD, not scraped.
3. **An orchestrator that knows what it has.** A routing layer decides per request: the on-device base, which adapter, or the bigger server model (Private Cloud Compute) when the ask exceeds the device. The brain's first act is knowing its own inventory.
4. **Personal context as an index**, not as training: what the user's device knows is kept queryable and consulted at ask time, never baked into weights.

## The port, term by term

| Apple | Phient | state |
|---|---|---|
| one base model | one phasor world (phiecs): every feature reads the same moving phasors | built |
| LoRA adapter per feature | a condition CHANNEL per construction family: opened lazily, thin, never retrains the base (docs/188) | built — this is the same move, found independently |
| App Intents vocabulary | the kit (phiecs::kit): the body's, the track's, the device's actions and things TOLD as typed rows | built for Apple's own interfaces |
| orchestrator / PCC routing | **phient, the brain**: holds the arena (every model, one table), knows what it has — a phiano in phinet, a phiora database — and routes the ask; an outside oracle (ollama) is its overflow | the crate exists (formerly phiplm); the serving loop is the next rung |
| personal context index | the tree of knowing (prose): queryable at ask time, grown by telling, never baked | built |

The lesson taken, stated once: **the single layer is not one model doing everything — it is one base that everything else is a cheap condition on.** That is already this paradigm's sentence ("meaning is a side effect of pairs of words moving under a condition"), which is why the port is mostly a naming, not a rebuild. What Apple has that we do not is the orchestrator in production: phient must become the process that loads the world, knows the inventory, and answers asks — the ollama of Phient systems.

## Status

2026-10-25. phiplm renamed phient (the brain); the old phient seam lives on as `phiano::seam`; workspace green. Next rung for the brain: a `phient serve` that holds worlds and routes asks, measured by the same honest protocol it already carries.
