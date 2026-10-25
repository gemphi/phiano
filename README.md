# phiano — the instrument

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

Phiano is where the models are played and measured. The arena (`src/arena/`) lines up one reader per model — counts (the line to beat), concepts, the phasor engine, phical's conjugate mirror, phifno's operator — and scores them on one stream, every next word predicted before it is told (the prequential protocol, no model ever scored on what it has seen). The scribe (`src/scribe/`) learns to WRITE programs against the [phiclm](https://github.com/gemphi/phiclm) judge with no model underneath. The prose tree (`phiplm::prose`, driven from here) discovers construction families by being curious — interest points at what is not yet known, and what recurs is what is real.

## The examples are the demonstrations

```
cargo run --release -p phiano --example person   -- <corpus> --chunks 8     # the learning curve, honest
cargo run --release -p phiano --example talk     -- <corpus> "a prompt"     # continuation in the learned voice
cargo run --release -p phiano --example workshop -- murphy.txt --key ...    # the 129-unit course, graded against the Key it was never told
cargo run --release -p phiano --example arena    -- <corpus>                # every model, one table
cargo run --release -p phiano --example teacher  -- <corpus> --oracle ollama:phi4   # the quest loop
cargo run --release -p phiano --example writer                               # judge-taught program writing
```

## The record

`docs/001–195` is the round-by-round history, one doc per rung, measurements verbatim and the conditions of satisfaction fixed before each run. Where it stands (docs/190–194): a whole book told in 3.6 s with no retraining; phasors lead on glosses (32.9% vs counts 29.9%), trail on prose (13.9% vs 21.4%); Key completion 15.7% vs 20.1%; continuation holds a construction and then decays — the two named open problems. What this is not, stated in writing (docs/194): not a dialogue agent, not comprehension, not a transformer.

Part of the [phinet](https://github.com/gemphi/phinet) workspace; the student half of [phient](https://github.com/gemphi/phient). MIT OR Apache-2.0.
