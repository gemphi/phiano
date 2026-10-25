# Personas — the tycoons of the technological eras, and the AI breed

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

One hundred and twenty-eight personas, in four rings of thirty-two. Each file follows the spec (`docs/SPEC.md`): a name, a voice (a register, never a quotation), four seeds, six kinds, the intents it acts under, the kit context it watches, and a standing in egg (`kudos`).

- **The makers** — fifty-one tycoons of the technological eras, from Gutenberg's press and Watt's engine through Carnegie, Siemens, Benz and Eastman to Noyce, Jobs, Bezos, Huang and the phone-makers of Shenzhen; they race alone, or for their company's team.
- **The AI players** — the labs as teams, each with founders and its model: meta (Zuckerberg, Wang, Llama), google (Pichai, Hassabis, Brin, Dean, Shazeer, Gemini), anthropic (Dario and Daniela Amodei, Kaplan, Olah, Clark, Claude), xai (Musk, Grok), openai (Altman, Brockman, GPT), microsoft (Nadella, Suleyman, Ballmer, Copilot), deepseek (Liang Wenfeng, DeepSeek), mistral (Mensch, Mistral), baidu (Robin Li, ERNIE), alibaba (Jack Ma, Qwen), ssi (Sutskever, SSI), apple (Jobs, Cook), thinking-machines (Murati).
- **The direction of AI** — the debate, alone: Hinton, LeCun, Bengio, Karpathy, Chollet, Marcus, Stuart Russell, Bostrom, Yudkowsky, Tegmark, Fei-Fei Li, Kai-Fu Lee, Andrew Ng — and Pepper, the humanoid greeter.
- **The thinkers** — the set game's own: Russell and Whitehead, Frege, Cantor, Peirce, Boole, De Morgan, Tarski, Gödel, Turing, Shannon, von Neumann, Wittgenstein, Searle, Austin, Grice, Chomsky, Saussure, Kuhn, Popper, Leibniz, Euler, Fourier, Bentov, Kuramoto, Hilbert, Noether, Lovelace, Babbage, Aristotle, Hume, Feynman.

Kudos is a handicap only (docs/160): the race is decided by what each world has been **told**, and a team is told by being quested (`persona_quest`) or engaged (`race --engage <team> <rows.tsv>`). The files are templates: edit the seeds, the kinds, the kudos; add a persona by adding a file. `tests/personas.rs` keeps the ring whole: 128 files, every name and slug one, every racing team with founders and a model, and race, circus and sets running over all of them.

The docs suite: `docs/SPEC.md` (the template), `docs/SIRI.md` (Apple's App Intents read through the kit), `docs/MUSE.md` (what Meta did with Muse Spark, and ours on an open model), `docs/CIRCUS.md` (two personas talk), `docs/SETS.md` (two personas think: language as a set game).

Kudos is a handicap only (docs/160): the race is decided by what each world has been **told**, and a team is told by being quested (`persona_quest`) or engaged (`race --engage <team> <rows.tsv>`). The files are templates: edit the seeds, the kinds, the kudos; add a persona by adding a file.

```text
cargo run --release -p phiecs --example persona_quest -- quests crates\phiano\personas     # phi-4 on your machine; a folder, so PowerShell needs no glob
cargo run --release -p phiano --example race -- quests crates\phiano\personas
cargo run --release -p phiano --example circus -- quests crates\phiano\personas gates zuckerberg --turns 12
cargo run --release -p phiano --example think -- quests crates\phiano\personas russell whitehead
```
