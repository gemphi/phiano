# 161 · The persona spec, Siri and Muse read through the kit, and the circus

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Thirty-two, and a spec

`crates/phinet/personas/` now holds 32 personas: nineteen tycoons, one per technological era from Gutenberg's press to Huang's GPU (Gutenberg, Watt, Westinghouse, Morita and Nadella joined the sixteen of docs/160), and the AI breed in five teams — meta, google, anthropic, xai, openai — each with its founders and its model. Every file follows one meta-description, `personas/docs/SPEC.md`: name, voice (a register, never a quotation), seeds, kinds, **intents** (the kinds it acts under when it speaks; default its kinds), **context** (the kit scopes it watches — `body/face`, `track`, `device`; default all), per_seed, depth, relations, inherit, inverse, transitive, sources, team, kudos. `Persona` gained `intents` and `context`; `Agent::watch` honours `context` (a persona watching `body/face` is told the face and nothing of the track — tested).

## 2. Siri through the kit, Muse through an open model

`personas/docs/SIRI.md` maps Apple's App Intents (WWDC26 session 343) onto the system one row at a time: an intent schema is a kind, an entity is a Word, the Spotlight semantic index is Counts + ScopeTree, an `IntentValueQuery` is `ask`, a donation is `tell`, on-screen awareness is `watch` under `context`, entity ownership is `Under` and the World's rules, and the model behind Siri (Apple Foundation Models with Gemini) is the oracle — asked, never trusted. phikit already reads the Apple frameworks as rows, so the mapping is a table, not a port.

`personas/docs/MUSE.md` takes what Meta ships with Muse Spark (announced 8 April 2026; Instant and Thinking modes, subagents in parallel, personal context, live camera, voice, shopping, side chats, the glasses; proprietary, private-preview API) and gives each feature its equivalent here on an open model — depth, teams, told + watched, the track scope, the voice field, the catalogue persona, the circus, the device scope — with the one honest "better": the personalization is rows beside the persona file that the user can read, tell, watch and race, and the model is outside the persona, so it can be swapped and the rows kept.

## 3. The circus

`phinet::sys::circus`: two beings talk. A line is a row the speaker's world holds, said under one of its intents (the heaviest not already said); the listener hears it — the row is told to its world; the answer is the next word; a speaker with nothing on the word changes the subject to one of its seeds. No generator, no prompt. `examples/circus.rs`: `circus <root> <personas> gates zuckerberg --turns 12`. On the stand-in oracle, eight turns between Gates and Zuckerberg: three subject changes, three edges heard each way (`personas/docs/CIRCUS.md`). Talking changes the race, because hearing is telling.

## 4. Measured (stand-in oracle, 32 × 4 seeds × 24 questions, depth 1)

| team | told | per member | kudos |
|---|---|---|---|
| google | 534 | 178 | 104 |
| meta | 509 | 170 | 88 |
| anthropic | 500 | 167 | 72 |
| xai | 379 | 190 | 72 |
| openai | 356 | 178 | 56 |

The race table gained a per-member column, since a team is told per member: xai, two strong, leads per member while google leads in total. The content is the stand-in's pool; with phi-4 on the user's machine (`persona_quest -- quests crates\phinet\personas`, a folder — PowerShell does not expand `*.md`, which is the panic the user hit and the runner now accepts a directory or the literal pattern) the same tables are about search, graphs and rockets.

## 5. Status

Done 2026-10-12: 388 tests, 0 warnings, every `.rs` under 200 lines; synced. Next: docs/159 step 5, the phasor convergence, a `Body` agent on live tracking rows, and personas quested by phi-4.
