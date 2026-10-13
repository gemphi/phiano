# 67 - Three More Measurements

> _What `docs/66` left open: the mixture, depth three, one operator._

`cargo run --release --bin phinet_more` (`data/rust_book_corpus.txt`, 5 epochs, one seed, the same split as before; Kneser-Ney 132.07, unigram under the trigram affine 125.64).

---

## 1. The mixture — a hypothesis refuted

The operator's phase term alone read 134–149 and the unigram 126, and mixing them gained a fifth of a point: they predict nearly the same thing. The hypothesis was that this is the decoys' fault — a uniform decoy teaches the operator to beat a random word, which the unigram already does, so decoys drawn in proportion to frequency should teach it to beat the likely words and leave it with something the unigram lacks. `phinet::FrequencySampler` and `Fno::train_epoch_with` make that one change.

```
decoys                        valid MRR    alone (γ = 1)    best mix
uniform (as before)             0.1425        149.30        125.64 at γ = 0
frequency^0.75                  0.1479        153.55        125.64 at γ = 0
frequency                       0.0716        175.30        125.64 at γ = 0
```

It is wrong. Frequency-proportional decoys make the ranking slightly better inside the pool and the full-vocabulary distribution worse, and the mixture still wants γ = 0. The softmax over 6,221 words is not limited by which decoy the hinge saw; it is limited by what a cosine between a 64-sample function and a word can express about the *next* word once the trigram tables have taken their share. That is a statement about the objective, not the sampling. The next thing to try is the residual proper — train the operator's output against the trigram affine's leftover probability mass, not against the next word — and that is a different loss, not a different decoy.

## 2. Depth three — the pattern holds

Cleanups trained on levels `1..=K`, read at levels 1–4, held-out words, 704 composable, chance MRR 0.0101.

```
level    phase only     cleanup K=1     cleanup K=2     cleanup K=3
1        0.192   19x    0.711   70x     0.654   64x     0.664   66x
2        0.100   10x    0.033    3x     0.571   56x     0.531   52x
3        0.048    5x    0.017    2x     0.025    2x     0.456   45x
4        0.017    2x    0.014    1x     0.016    2x     0.030    3x
```

Exactly the rule `docs/66` guessed at: the operator recovers every depth it was trained on, at 45–70× chance, and is at chance one deeper. It does not saturate at three — level 3 through the K=3 cleanup is 45×, where phase subtraction alone is 5×. Training on a level costs the levels below it a little (level 1: 70× → 66×); an operator that has to clean three kinds of crosstalk cleans each slightly less well than one that only knows one kind.

So recursion depth in this architecture is a training decision, not a representational limit: nest as deep as you are prepared to show the cleanup.

## 3. One operator for both objectives

The LM operator and the cleanup are the same architecture. One network, LM epochs interleaved with all three cleanup levels, same total epochs as each specialist:

```
                                  LM valid MRR   recover L1   recover L2   recover L3   perplexity alone
specialists                          0.1479        0.664        0.531        0.456           149.30
one shared operator                  0.1450        0.635        0.513        0.419           149.22
```

It loses 2% on the LM task and 4–8% on recovery, and its perplexity is the same. Nothing the one task learns breaks the other. 8,360 parameters carry both next-word prediction and three levels of holographic cleanup on the same relation circle.

## 4. What changed in the code

| Where | What |
|---|---|
| `phinet/src/operator.rs` | `Fno::train_epoch_with(…, sampler, …)` — decoys from any `Fn(target, seed) -> usize`; `train_epoch` is the uniform case. `FrequencySampler` — decoys ∝ `weight^alpha`. |
| `phiano/src/bin/phinet_more.rs` | The three measurements. |

## 5. Next

* The residual loss. Replace the hinge target with the trigram affine's leftover mass: for each position, the operator should raise the words the n-gram tables under-predict and nothing else. That is the only route left to a γ above zero, and it is a change to the loss.
* Depth as a curriculum. Since depth is bought by training, train the cleanup on levels 1–K with K rising as the lower levels stabilise, and find where the shared parameter budget runs out.
