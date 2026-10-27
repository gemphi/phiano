# The circus — Bill Gates talks to Mark Zuckerberg

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle; meaning is a side effect of pairs of words moving under a condition, and they move by the error of what was expected — toward what came, away from what did not.

Two personas talk. Nothing is generated: a line is a row the speaker's world was told, said under one of its **intents**, and the listener **hears** it — the row is told to the listener's world. So a conversation is a sequence of moves in a language game (Wittgenstein, *Investigations* §43: the meaning of a word is its use), and here use is counted.

**A turn.** The speaker takes the word on the table, picks the intent under which its world has the most to say about it, says the heaviest line not already said, and the answer is the next word. A speaker with nothing on the word changes the subject to one of its own seeds. The script is printed as `Name: word — kind → other (weight)`, then what each heard.

```text
cargo run --release -p phiano --example circus -- quests crates\phiano\personas gates zuckerberg --turns 12
cargo run --release -p phiano --example circus -- quests crates\phiano\personas wang musk --word data
```

On the stand-in oracle (a fixed pool, so the words are the pool's), eight turns:

```text
Bill Gates (211 told) and Mark Zuckerberg (161 told), 8 turns

Bill Gates: operating system — does → rope (0.70)
Mark Zuckerberg: rope — has → shore (0.70)
Bill Gates: shore — has → fish (1.30)
Mark Zuckerberg (changing the subject): social graph — has → boat (1.20)
Bill Gates (changing the subject): operating system — has → wave (0.70)
Mark Zuckerberg: wave — does → wind (0.70)
Bill Gates: wind — does → water (0.70)
Mark Zuckerberg (changing the subject): social graph — does → rope (0.70)

Bill Gates heard 3 new edges; Mark Zuckerberg heard 3.
```

The content is the stand-in's; the mechanism is the point: the subject changes exactly when a world runs out, each speaker leaves knowing what the other said, and the race (`examples/race`) counts it. With phi-4 as the oracle the same script is about operating systems and social graphs.

**What a circus act is, in the system:** `circus::talk(a, b, word, turns)` → `Vec<Turn>`; `circus::line(who, word, script)` is the one move; `Agent::tell` is the hearing. There is no generator, no prompt, no temperature.
