# What Meta did with Muse Spark — and the same, better, on an open model

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Muse Spark** (Meta Superintelligence Labs, announced 8 April 2026) is the first model of Meta's Muse series: "small and fast by design", powering the Meta AI app and meta.ai, with a May 2026 update bringing voice, shopping and the glasses. Meta did not call it open-weight — it is in private preview via API to select partners, and Meta "hope[s] to open-source future versions". What the Meta AI app does with it, feature by feature, and what we give our users on an open model (Llama, phi-4 — any oracle), with the one thing we add: everything the persona knows is a row with a count you can read.

| Muse Spark in the Meta AI app | ours, on an open model | the better |
|---|---|---|
| **Instant** and **Thinking** modes | a quest at `depth: 0` (ask the seed, stop) or `depth: 2` (walk out from it) | the depth is in the persona file; the walk is written to disk step by step |
| **Subagents in parallel** — trip planning split into itinerary, comparison, activities | a **team**: every member is told, asked and races; `Race::engage` tells all members one set of rows | the members' worlds are separate files; what each learned is inspectable |
| **Personal context** — local public posts, Reels and maps recommendations | **told + watched**: the person's rows (what they said, what the tracker saw under `context`) | the context is rows the person can read and delete |
| **Live camera AI** (May update) | **watch**: phikit's track rows — anchors, frames, the face's shapes — are rows | the camera's output is a sentence in the same tree |
| **Voice**: interrupt, switch language mid-chat, images during voice | the **voice** field + `ask_voices`: sentences in the persona's register, saved as `sentences.tsv` | the voice is a condition on the quest, not a hidden prompt |
| **Shopping mode** — Marketplace + web with a map | a persona whose seeds are the catalogue, asked `used-for`, `has`, `part-of` | the kinds are declared; the answers are counted |
| **Side chats**, **@meta.ai** in group chats | the **circus**: two personas talk; each line is heard by the other | talking changes what each knows, and the race shows it |
| **Glasses** (Ray-Ban, Oakley; Display this summer) | the **device** scope of the kit, watched under `context: device` | the device is told as rows, like the body |
| proprietary, private preview | **open**: the oracle is any program that answers text with text; the persona's knowledge is yours | swap the model; keep the rows |

## Why "better" is honest

Muse Spark is a stronger model than any oracle we run locally today; this is not a claim about the model. It is a claim about the **structure around it**: Meta's personalization lives inside the model and the app, ours lives in `relations.tsv` beside the persona file. A user can read what their persona knows, tell it more, watch it learn, and race it against another — and every one of those is a measurement (docs/160: a team told nothing cannot win on kudos).

Sources: [Introducing Muse Spark: Meta's Most Powerful Model Yet — Meta Newsroom, 8 April 2026](https://about.fb.com/news/2026/04/introducing-muse-spark-meta-superintelligence-labs/) · [Muse Spark — Wikipedia](https://en.wikipedia.org/wiki/Muse_Spark) · [Meta rolls out Muse Spark — Cybernews](https://cybernews.com/ai-news/meta-muse-spark-personal-ai-model/)
