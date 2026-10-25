# Siri, personal — read through the Apple crates

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

Apple makes Siri personal through **App Intents** (WWDC26 session 343): an app describes its *actions* as intents conforming to system schemas, its *content* as entities with a display representation, and how to *find* that content as queries; it *donates* what the person did in the UI so Siri has personal context, and it *annotates* what is on screen so Siri can resolve "that one". Trust is handled by ownership: shared or public content gets a confirmation before an action with side effects. The 2026 Siri runs on Apple Foundation Models developed with Google's Gemini and reaches into Messages, Mail and Photos for personal context, with on-screen awareness and in-app actions.

phikit already reads Apple's frameworks (ARKit, Vision, IOKit) as rows — the face's shapes, the body's joints, tracking's anchors, frames and sessions, the kernel's registry — so the mapping is one table:

| Siri / App Intents | ours | where |
|---|---|---|
| `@AppIntent(schema: …)` — an action with standard parameters | an **intent**: a kind the persona acts under; the row `about kind other` is the action with its parameter | `Persona::intents`, `circus::line` |
| `@AppEntity` with a `DisplayRepresentation` | a **Word** in the World, with its `has-word` / `says` rows | `phiecs::World`, `phikit::tell` |
| `IndexedEntity` → Spotlight semantic index (match by meaning) | **Counts + ScopeTree**: the cast draws a thing from its words | `phiplm::relate::{Counts, ScopeTree}` |
| `IntentValueQuery` — content too large or live to index | **ask**: what a word calls for under a kind, heaviest first | `Agent::ask` → `World::calls_for` |
| `IntentDonationManager.donate` — personal context from UI use | **tell**: every use is a row, counted at once; nothing is donated twice because the count is the memory | `Agent::tell` → `World::tell` |
| on-screen awareness (`appEntityIdentifier` annotations) | **watch**: the kit's rows under the persona's `context` scopes — what the tracker sees is what is on screen | `Agent::watch`, `Persona::context` |
| `EntityOwnership` → automatic confirmation | **Under** (direction of fit) and the World's rules (`forbid`, `want`) | `phiecs::world::rule`, `phiano::cognitive::dof` |
| the model behind it (Apple Foundation Models + Gemini) | **the oracle**: phi-4, Llama or any program that answers text with text — asked, never trusted; its answers are evidence with counts | `phiecs::quest::Oracle` |

What is the same: an assistant is personal because of what it has been told about the person and what it can see. What is different, and better here: the personal context is **rows you can read** (`relations.tsv`), the memory is a **count** rather than a vector, the "semantic index" is the same tree the language model reads, and the model is **outside** the persona — swap the oracle and the persona keeps everything it was told.

Sources: [Explore advanced App Intents features for Siri and Apple Intelligence — WWDC26](https://developer.apple.com/videos/play/wwdc2026/343/) · [Apple confirms Google Gemini will power next-generation Siri — MacRumors](https://www.macrumors.com/2026/01/12/google-gemini-next-generation-siri/) · [Google confirms Gemini-powered Siri coming later this year — MacRumors](https://www.macrumors.com/2026/04/22/google-gemini-powered-siri-2026/)
