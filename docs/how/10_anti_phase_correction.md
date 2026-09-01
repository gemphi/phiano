# HOW 10 — Anti-Phase Correction

> _Unlearning in one step. No retraining run, no gradient, no checkpoint rollback:
> a π-radian pulse and the association is gone._

---

## 1. The mechanism

```rust
// src/trainer/mod.rs :: Trainer::correct_mistake
pub fn correct_mistake(&self, facet: &mut Facet, wrong_phrase: &str, correct_phrase: &str) {
    for token in &Tokenizer::tokenize(wrong_phrase) {
        if let Some(phasor) = facet.lexicon.get_mut(token) {
            phasor.phase = (phasor.phase + PHASE_REPULSION).rem_euclid(TWO_PI);  // += π
            phasor.amplitude = (phasor.amplitude * 0.8).max(AMPLITUDE_INITIAL);
        }
    }
    self.train_sentence(facet, correct_phrase);
}
```

`PHASE_REPULSION = π`. Adding π to a phase is multiplication by −1 in the complex
plane:

$$A e^{i(\theta + \pi)} = -A e^{i\theta}$$

A word that was reinforcing the sentence wave now cancels it, exactly.

Two effects, applied together:

1. **Phase inversion** — the association flips sign.
2. **Amplitude decay** — familiarity ×0.8, floored at `AMPLITUDE_INITIAL` = 1.0.

Then the correct phrase is trained normally, so the corrected association is
reinforced in the same call.

---

## 2. Worked example

Suppose the model has wrongly associated `rust` with `slow`:

| word | θ | A |
|:--|--:|--:|
| `rust` | 1.00 | 1.60 |
| `slow` | 1.05 | 1.40 |

Their wave contribution: $1.6e^{i1.00} + 1.4e^{i1.05}$
= (0.86449 + 1.34635i) + (0.69308 + 1.21571i) = 1.55757 + 2.56206i,
|Z| = **2.99816** — near-maximal reinforcement (max would be 3.0).

Now `correct rust is slow -> rust is fast`:

**Pulse applied to** `rust`, `is`, `slow`:

| word | θ before | θ after (+π) | A before | A after (×0.8) |
|:--|--:|--:|--:|--:|
| `rust` | 1.00 | 4.14159 | 1.60 | 1.28 |
| `slow` | 1.05 | 4.19159 | 1.40 | 1.12 |

New joint wave: $1.28e^{i4.14159} + 1.12e^{i4.19159}$
= (−0.69159 − 1.07708i) + (−0.55446 − 0.97257i) = −1.24605 − 2.04965i

The pair still reinforces *each other* (they moved together), but both are now
**antipodal to where they were**, i.e. in the opposite sector of the manifold —
sector 10 rather than sector 10+32. Relative to every other word in the lexicon,
their association has inverted.

Then `train_sentence(facet, "rust is fast")` pulls `rust` back toward `fast` and
the corrected pairing is established.

### The cost of one correction

| operation | cost |
|:---|:---|
| tokenize wrong phrase | O(L) |
| π pulse + amplitude decay | O(L) map lookups |
| train correct phrase | O(L) |
| **total** | **O(L)**, microseconds |

Compare with the alternatives: fine-tuning an LLM to unlearn one association is
hours of GPU time with no guarantee of locality; RLHF is a data-collection
project; prompt-patching does not change the model at all. Here it is
sub-millisecond, local, and immediate.

**This is a genuine architectural advantage and it should be one of the project's
headline claims.**

---

## 3. Where it goes wrong

### (a) π is too blunt

The pulse is the maximum possible rotation. There is no "slightly wrong". A
single correction moves a word to the far side of the manifold, destroying every
*other* association it had.

`rust` is not only wrongly near `slow` — it is also correctly near `memory`,
`safety`, `cargo`, `borrow`. The π pulse breaks all of them at once to fix one.

**Fix — graded correction proportional to the error:**

```rust
pub fn correct_graded(&self, facet: &mut Facet, wrong: &str, correct: &str, strength: f64) {
    let wrong_toks  = Tokenizer::tokenize(wrong);
    let correct_toks = Tokenizer::tokenize(correct);
    // only push apart the tokens that are in `wrong` but NOT in `correct`
    let offenders: Vec<&String> = wrong_toks.iter()
        .filter(|t| !correct_toks.contains(t)).collect();

    let target = self.compute_centroid_phase(facet, &correct_toks);
    for token in offenders {
        if let Some(p) = facet.lexicon.get_mut(token) {
            // rotate AWAY from the corrected meaning, by `strength`, not by π
            let away = -(target - p.phase).sin();
            p.phase = (p.phase + strength * away).rem_euclid(TWO_PI);
        }
    }
    self.train_sentence(facet, correct);
}
```

With `strength = 0.3`, a correction nudges rather than teleports, and `is` — a
function word appearing in both phrases — is left alone entirely, which the
current implementation does not do.

### (b) Function words are punished

`correct "rust is slow" -> "rust is fast"` currently applies the π pulse to
`is`, even though `is` appears in the corrected phrase too. `is` occurs in a
large fraction of all sentences; inverting it degrades the model globally to fix
one specific fact. The `offenders` filter above is the two-line fix.

### (c) Amplitude decay cannot go below 1.0

```rust
phasor.amplitude = (phasor.amplitude * 0.8).max(AMPLITUDE_INITIAL);   // floor = 1.0
```

A word corrected repeatedly bottoms out at 1.0 and cannot be pushed further. It
can never become *less* familiar than a brand-new word, so there is no
representation for "I have actively learned that this is wrong" as distinct from
"I have never seen this". Those are different epistemic states and the model
should be able to hold both.

**Fix:** floor at something below the initial value, e.g. 0.3, and let low
amplitude mean actively-distrusted:

```rust
phasor.amplitude = (phasor.amplitude * 0.8).max(0.3);
```

### (d) The correction leaves no trace

`correct_mistake` returns `()`. There is no log of what was corrected, when, or
whether the correction held. So corrections cannot be replayed after a reload,
audited, or undone.

**Fix — a correction ledger, persisted alongside the facet:**

```rust
#[derive(Serialize, Deserialize)]
pub struct Correction { pub wrong: String, pub correct: String, pub ts_ms: u64, pub strength: f64 }

#[derive(Serialize, Deserialize, Default)]
pub struct CorrectionLog { pub entries: Vec<Correction> }
```

Then `replay(&facet)` re-applies every correction after a fresh ingest, so
user-taught fixes survive re-training from source. For a system whose pitch is
*personal, on-device, continually-taught*, that persistence is not a nice-to-have.

---

## 4. What this buys

- **O(1) targeted unlearning** — the single most valuable property the
  architecture has that gradient-trained models do not.
- **Immediate effect** — the next query reflects the correction; no retraining, no
  redeploy.
- **Locality** — only the named tokens move. Nothing else in the lexicon is
  touched. (Contrast: a gradient step touches every parameter.)
- **Symmetry with training** — correction and learning use the same primitive
  (a phase rotation), so there is no separate unlearning subsystem to maintain.

---

## 5. The ceiling

Correction operates on **words**, and the errors people want to correct are
usually about **relations**.

`"rust is slow"` is wrong not because `rust` and `slow` are individually
mispositioned but because the *proposition* is false. Moving both words does not
represent "these two things are not related in this way" — it represents "these
two things are elsewhere now", which also breaks their true relations.

This is the same limit as HOW 03 and HOW 06 seen from a third angle: without a
binding operator there is no object called "the association between rust and
slow" to modify. There are only word positions.

---

## 6. How it generalises

With binding (HOW 03 §6), correction becomes precise:

```rust
// suppress a specific bound proposition, not the words in it
pub fn correct_proposition(facet: &mut Facet, subj: &str, verb: &str, obj: &str, strength: f64) {
    let z = bind_proposition(facet, subj, verb, obj);
    // record an inhibitory trace at that exact point in the manifold
    facet.inhibitions.push((z, strength));
}
```

and retrieval subtracts inhibitory traces before ranking. Now `rust is slow` is
suppressed while `rust is memory-safe` and `rust is fast` are untouched, because
they are different bound points.

Also worth adding, independently of binding:

1. **Graded strength** (§3a) — the single highest-value change here.
2. **Skip tokens shared with the correct phrase** (§3b) — two lines.
3. **Amplitude floor below initial** (§3c) — one constant.
4. **Persisted correction log with replay** (§3d) — makes teaching durable.
5. **Confirmation loop:** after correcting, re-evaluate the wrong phrase and
   report the delta, so the user can see the correction took:
   ```
   corrected. "rust is slow" coherence 0.94 → 0.11
   ```

---

## 7. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| Pulse is exactly π | `PHASE_REPULSION` in `src/config/constants.rs` |
| Amplitude ×0.8, floored at 1.0 | `correct_mistake` |
| Function words in both phrases are still punished | no set difference is computed |
| Correct phrase is trained immediately after | `self.train_sentence(facet, correct_phrase)` |
| No return value, no log | signature returns `()` |
| This is the only negative coupling in the system | grep `PHASE_REPULSION` |

---

**Next:** [HOW 11 — Generation](11_generation.md).
