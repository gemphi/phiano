---
id: the-forest
title: "The Forest: One Told Story Read by Everything the System Holds"
description: Phient's paper on the scope tree as a bounding-volume hierarchy. One eighty-six-token story, told once into Jane Austen's Emma, read by the structures already there. The five-part correspondence between a BVH and a tree of conditions — volume, contains, ray, hit test, pruning — written out against the code; the bounding volumes of every word of the story; all twenty-seven casts with their attractions in nats; and the honest limit: the cast is drawn by what a paragraph says, and most of what these stretches say is function words.
sidebar_position: 28
badge: paper
---

# The Forest: One Told Story Read by Everything the System Holds

## The Scope Tree as a Bounding-Volume Hierarchy, and Twenty-Seven Casts Read Honestly

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

### Abstract

A user told the system a story of eighty-six tokens, written in capitals and with its own misspellings, and it was read by everything the system holds. This paper reports the first part of that reading: the scope tree the story joined, and what happened when it was cast back into it. The language is Jane Austen's *Emma* — 2,315 paragraphs with the story in, 2,314 without, 2,969,690 rows counted, 69,896 words in the tree. The central argument is that this tree is a bounding-volume hierarchy, exactly in five parts: the volume is a scope's distribution over what it says; *contains* is that a scope says everything its subtree says; the ray is a few words and their pairs; the hit test is D4's movement in nats; the pruning is the beam. The volumes, measured, are mostly not tight: `to` is said in 1,630 of the 2,314 held-out paragraphs and `the` in 1,486, against `sunday` in 2, `holding` in 1, and sixteen words of the story in none. Twenty-seven stretches of six words were cast; 22 of the 27 attractions are negative, the worst −40.963 nats, and of the 162 heaviest words listed for the winning leaves, 150 are function words, pronouns or auxiliaries. The cast is drawn by what a paragraph *says*, and most of what these stretches say is `it was a his that`. No understanding is claimed.

## 1. What was told

One story was told to the system, once, by the person who wrote it:

```
IT WAS ON A SUNDAY AFTERNOON WHEN IT FINALLY DOWNED ON OUR GENTLEMAN THAT HE
HAS EXCEEDED THE THRESHOLD OF HOLDING ONTO HIS EGO ANDN HAPPENED TO BE IN A
GREAT POSITION AND HE WAS IN THE PROCESS OF LAUNCHING HIS BUSINESS WITH HIS
AMERICAN COUNTER PARTS IN THAT MOMENT IT SUDDENLY OCCURED TO HIS WITTY SENSES
THAT SOMEHOW EVERTIHNG HAD ALWAYS WORKED FOR THE BEST OUTCOMES AND THAT MOVED
HIS HEART TO BE A STRONG AND LOVEFUL MACHINE THAT INSULATED HIS ACTION THAT
LOVE.
```

Eighty-six tokens, 58 distinct words. The reader lower-cases and changes nothing else: the misspellings `andn` and `evertihng`, and the coinage `loveful`, are in the lexicon as the words they are. The story was put in `data/text/story/ch2.txt`, beside the language it was joining — *Emma*, 2,315 paragraphs with the story told, 2,314 without it, 2,969,690 rows over 69,896 words — and the language was read again.

Nothing about its place was declared. `read_tree` walks the directory, and the nesting it finds is the nesting it tells: `scope:text` holds `scope:text/story` holds `scope:text/story/ch2` holds `scope:text/story/ch2/p1`, each told `within` the one above. That is the only structural row; everything else is a word said under the kind `says`, under *every* scope on its path. The folder is the condition, and the condition is a word (docs/126). Every number here is from one run of `bin/forest`, made on 2026-10-12 and tabulated in `docs/papers/forest/FOREST_FACTS.md`.

## 2. The scope tree, and what a scope says

A node is a scope: a text, a chapter, a paragraph — and a word too, since a kind and a condition are words. `ScopeTree::from_rows` keeps only a map from parent to children, built from the rows where `r.kind == "within" && r.about.starts_with(SCOPE)`.

A scope's table under `says` is a count, not yet a model; the smoothing makes it one. `Counts::dist` mixes the table toward a base with weight one — one sighting is never certainty — and a child's base is its parent's distribution, itself mixed toward the prior: Witten and Bell's zero-frequency estimate applied level by level (docs/126, 131). What matters below is what a scope says *beyond* its parent: `c · ln((c/t_s) / p_parent)`, the `beyond` function of `casts.rs`.

| word | nats | word | nats |
|---|---|---|---|
| his | 3.85 | action | 0.64 |
| that | 2.13 | afternoon | 0.64 |
| a | 1.93 | always | 0.64 |
| and | 1.06 | american | 0.64 |
| in | 1.06 | andn | 0.64 |
| it | 1.06 | best | 0.64 |

*Table 1. What `scope:text/story/ch2/p1` says beyond its parent `scope:text/story`, in nats (`casts.rs::beyond`). The story's chapter holds one paragraph, so the parent used is the text above it.*

The word this paragraph adds most is `his`, at 3.85 nats, because `his` occurs six times in 86 tokens and the gain is a count times a log ratio. Below `and`, `in` and `it` is a flat shelf at 0.64 — the gain of a word said once where the base expected half a sighting — on which the story's content words sit tied in alphabetical order. This is what information gain over a count table measures: a repeated word is evidence about a paragraph, a word said once almost none. When the same quantity steers a search, the search is steered by the repeated words, which in English are `it`, `was`, `a`, `his` and `that`.

## 3. The scope tree is a bounding-volume hierarchy

A renderer uses a bounding-volume hierarchy to avoid testing a ray against every triangle: each node holds a volume enclosing everything beneath it, so a ray that misses it skips the subtree. The scope tree is that structure with the geometry taken out and a distribution in its place — its module documentation has said so since docs/126, *a word is not aware of where it belongs: it is drawn into scopes by what it says, the way a ray is drawn through a bounding-volume hierarchy* — and the correspondence holds in five parts.

![The BVH correspondence](fig/f155_bvh.png)

*Figure 1. Left, a bounding-volume hierarchy: hit the box and test the children, miss and skip the subtree. Right, the same walk over conditions, term by term. Drawn from `phiplm::relate::tree`'s own module documentation.*

**The volume is a distribution.** A BVH node holds six numbers enclosing the geometry below; a scope holds `dist(scope, SAYS, values, base)`, a distribution over the lexicon, every entry positive, enclosing what was said below. It is what a box is, a cheap summary testable without visiting the subtree: `dist_weighted` scales the base by `w/(total+w)` and adds the scope's entries by a cached index, so a paragraph's distribution is its parent's plus a pass over what it said.

**Contains is *says everything its subtree says*.** In a BVH, containment is the invariant that makes pruning sound. Here it holds by the telling, not by a fitting pass: every word is told under every scope on its path, so `count(chapter, says, w) ≥ count(paragraph, says, w)` by construction, the parent's table being the sum of its children's — the 2.8 KB a word of docs/133.

**The ray is a few words and their pairs.** In `casts.rs`, six consecutive words and the five ordered pairs between them — `conds.extend(w.windows(2).map(|p| format!("pair:{} {}", p[0], p[1])))`, then `cast_under(&counts, &[], &conds, f64::MIN, 2)`. Eleven conditions per stretch; the pairs matter, since a paragraph saying `his` and saying `witty` has the vocabulary of `his witty` without the stretch. Here the analogy is loosest: a ray has order and extent, a list of conditions neither.

**The hit test is D4's movement in nats.** A renderer's test is a predicate; the cast's is a number, and it is §2's.

```rust
let said = conds.iter()
    .filter(|(c, _)| counts.count(Some(scope), SAYS, c) >= 1.0 && local(c)).count();
let graded: f64 = conds.iter().filter_map(|(c, i)| i.map(|i|
    ((counts.count(Some(scope), SAYS, c) + base[i]) / (total + 1.0) / base[i]).ln())).sum();
```

`said` carries D3 through `local`, which drops a condition said by more than half the siblings: `is` is said by every paragraph of *Emma* and draws nowhere. `graded` is the log of the scope's smoothed share over the base's, summed over the asked conditions. Two consequences have no analogue in geometry: a miss is quantitative — a scope can miss by 3 nats or by 40 — and a negative hit can still be entered, since the ranking is `said` first, with the attraction only breaking ties.

**The pruning is the beam.** The cast prunes by a fixed width, `next.truncate(beam)`, with `beam = 2`, and `casts.rs` passes `min = f64::MIN`, so nothing is pruned by the threshold: the attractions in §5 are of scopes entered on the strength of their counts, not of scopes that passed a bar. Here the structure gives up a guarantee the renderer keeps — a beam of two never sees the paragraph that would have scored best under a dropped chapter.

| BVH | tree of conditions | code |
|---|---|---|
| volume: a box enclosing the subtree | a scope's smoothed distribution over what it says | `Counts::dist` |
| contains: the geometry lies inside | a scope says everything its subtree says, by the telling | `read_tree` |
| the ray: an origin and a direction | six words and their five pairs, eleven conditions | `casts.rs::casts` |
| hit test: does the ray meet the box | D4: `ln(p_scope / p_base)` summed over the conditions, in nats | `ScopeTree::attraction` |
| pruning: skip the subtree on a miss | the beam: the best two children, the rest dropped | `next.truncate(beam)` |

*Table 2. The correspondence, with the code that holds each part. Four more of the sixteen trees hold the rest of the apparatus: the strand's cut tree is the builder, as SAH splits a scene; the sixty-four levels and the sphere's degrees are levels of detail; the reading path is the traversal's node stack.*

A BVH is judged by how much of a scene a ray can skip, a property of how tightly its volumes fit; the question the analogy hands to a tree of conditions is not *is the cast accurate* but *how tight are the volumes*.

## 4. The bounding volumes: how much of *Emma* holds each word

For each of the story's 58 distinct words, `casts.rs` counts the held-out paragraphs that say it, and the chapters. The denominators are 2,314 paragraphs — *Emma* with the story's own first chapter, the second being what was held out — and 56 chapter-level scopes.

| word | par | ch | word | par | ch | word | par | ch |
|---|---|---|---|---|---|---|---|---|
| it | 1163 | 56 | his | 560 | 55 | witty | 2 | 2 |
| was | 928 | 56 | ego | 0 | 0 | senses | 4 | 4 |
| on | 507 | 56 | andn | 0 | 0 | somehow | 5 | 4 |
| a | 1320 | 55 | happened | 27 | 19 | evertihng | 0 | 0 |
| sunday | 2 | 2 | to | 1630 | 56 | had | 726 | 56 |

| word | par | ch | word | par | ch | word | par | ch |
|---|---|---|---|---|---|---|---|---|
| afternoon | 5 | 5 | be | 1039 | 56 | always | 204 | 52 |
| when | 296 | 52 | in | 1073 | 56 | worked | 2 | 1 |
| finally | 0 | 0 | great | 232 | 54 | for | 778 | 55 |
| downed | 0 | 0 | position | 0 | 0 | best | 82 | 41 |
| our | 79 | 34 | and | 1524 | 56 | outcomes | 0 | 0 |

| word | par | ch | word | par | ch | word | par | ch |
|---|---|---|---|---|---|---|---|---|
| gentleman | 23 | 16 | process | 2 | 1 | moved | 7 | 7 |
| that | 995 | 56 | launching | 0 | 0 | heart | 61 | 36 |
| he | 775 | 56 | business | 53 | 31 | strong | 41 | 28 |
| has | 195 | 43 | with | 772 | 56 | loveful | 0 | 0 |
| exceeded | 0 | 0 | american | 0 | 0 | machine | 0 | 0 |

| word | par | ch | word | par | ch | word | par | ch |
|---|---|---|---|---|---|---|---|---|
| the | 1486 | 56 | counter | 2 | 2 | insulated | 0 | 0 |
| threshold | 0 | 0 | parts | 2 | 2 | action | 6 | 6 |
| of | 1446 | 56 | moment | 105 | 37 | love | 101 | 42 |
| holding | 1 | 1 | suddenly | 7 | 4 |  |  |  |
| onto | 0 | 0 | occured | 0 | 0 |  |  |  |

*Table 3, in parts. The bounding volumes: how many of the 2,314 held-out paragraphs (par) and of the 56 chapter scopes (ch) say each distinct word of the story, in the story's own order.*

**Volumes that are the whole scene.** `to` is said in 1,630 paragraphs of 2,314, `and` in 1,524, `the` in 1,486, `of` in 1,446, `a` in 1,320, `in` in 1,073, `be` in 1,039, `that` in 995. Sixteen of the 58 words are said in more than a fifth of all paragraphs, thirteen in every one of the 56 chapter scopes and nineteen in 52 or more. A volume enclosing two thirds of the scene prunes nothing: a ray whose only condition were `to` would hit every chapter, and the traversal would be a full scan in disguise. This is why D3 exists and why it is not enough: `local` drops `the` and `to` from `said` at the chapter level, but they still enter the graded sum.

**Volumes that are tight.** `holding` is in 1 paragraph in 1 chapter; `sunday`, `witty`, `worked`, `counter`, `parts` and `process` in 2 each; `senses` in 4, `afternoon` and `somehow` in 5, `action` in 6, `suddenly` and `moved` in 7. A condition said by two paragraphs of 2,314 cuts the space by a factor of a thousand in one test, and `holding` posts 11.50 nats in D2's reading, the largest post of any word of the story — but a six-word window of prose rarely holds one, and when it does the other five outweigh it.

**Volumes that are empty.** Sixteen words are said in no paragraph and no chapter: `finally`, `downed`, `exceeded`, `threshold`, `onto`, `ego`, `andn`, `position`, `launching`, `american`, `occured`, `evertihng`, `outcomes`, `loveful`, `machine`, `insulated`. Such a condition has no index among the kind's values, so `filter_map` drops it from the graded sum: the cast is blind to the words that make the story new.

## 5. The twenty-seven casts

Twenty-seven stretches of six words, taken three apart so each overlaps its neighbour by three, were cast into the language with the story's own chapter held out. Each asks which paragraph of everything the language holds says *this* most, beyond what its chapter already said.

| # | stretch | leaf | said | own | attraction |
|---|---|---|---|---|---|
| 1 | it was on a sunday afternoon | emma/ch04/p8 | 27 | 6 | 0.055 |
| 2 | a sunday afternoon when it finally | emma/ch04/p8 | 20 | 4 | −0.138 |
| 3 | when it finally downed on our | emma/ch52/p13 | 15 | 3 | −3.770 |
| 4 | downed on our gentleman that he | emma/ch55/p9 | 16 | 4 | −25.498 |
| 5 | gentleman that he has exceeded the | emma/ch38/p52 | 18 | 5 | −5.319 |

| # | stretch | leaf | said | own | attraction |
|---|---|---|---|---|---|
| 6 | has exceeded the threshold of holding | emma/ch24/p39 | 8 | 1 | −10.396 |
| 7 | threshold of holding onto his ego | emma/ch24/p5 | 8 | 2 | 6.526 |
| 8 | onto his ego andn happened to | emma/ch49/p19 | 11 | 3 | 8.763 |
| 9 | andn happened to be in a | emma/ch19/p31 | 22 | 6 | −6.208 |
| 10 | be in a great position and | emma/ch51/p32 | 18 | 4 | −8.167 |

| # | stretch | leaf | said | own | attraction |
|---|---|---|---|---|---|
| 11 | great position and he was in | **story/ch1/p1** | 16 | 3 | −40.963 |
| 12 | he was in the process of | emma/ch23/p4 | 23 | 4 | −14.145 |
| 13 | the process of launching his business | emma/ch23/p4 | 20 | 3 | −10.270 |
| 14 | launching his business with his american | emma/ch08/p58 | 20 | 5 | −12.990 |
| 15 | with his american counter parts in | emma/ch27/p24 | 16 | 4 | −14.494 |

| # | stretch | leaf | said | own | attraction |
|---|---|---|---|---|---|
| 16 | counter parts in that moment it | emma/ch26/p97 | 19 | 4 | −28.868 |
| 17 | that moment it suddenly occured to | **story/ch1/p1** | 16 | 3 | −10.973 |
| 18 | suddenly occured to his witty senses | emma/ch54/p24 | 13 | 2 | −37.133 |
| 19 | his witty senses that somehow evertihng | emma/ch49/p19 | 13 | 3 | −24.658 |
| 20 | that somehow evertihng had always worked | emma/ch55/p2 | 14 | 3 | −19.890 |

| # | stretch | leaf | said | own | attraction |
|---|---|---|---|---|---|
| 21 | had always worked for the best | emma/ch53/p36 | 22 | 5 | −30.721 |
| 22 | for the best outcomes and that | emma/ch55/p2 | 16 | 3 | −20.541 |
| 23 | outcomes and that moved his heart | emma/ch54/p20 | 18 | 4 | −3.127 |
| 24 | moved his heart to be a | emma/ch54/p20 | 22 | 5 | −15.264 |
| 25 | to be a strong and loveful | emma/ch47/p33 | 20 | 5 | −15.591 |

| # | stretch | leaf | said | own | attraction |
|---|---|---|---|---|---|
| 26 | strong and loveful machine that insulated | emma/ch07/p5 | 10 | 3 | 8.562 |
| 27 | machine that insulated his action that | emma/ch48/p5 | 9 | 2 | 3.628 |

*Table 4, in parts. All 27 casts: `leaf` is the last scope of the winning path, `said` the asked conditions said anywhere along it, `own` those said by the leaf alone, `attraction` the graded sum in nats along the path. Two leaves are the story's own first chapter, in bold.*

| leaf | the six heaviest words it says | leaf | the six heaviest words it says |
|---|---|---|---|
| ch04/p8 | and 4, mrs 4, goddard 3, had 3, miss 3, a 2 | ch48/p5 | and 19, the 19, had 18, to 17, of 15, her 9 |
| ch07/p5 | a 9, i 6, and 5, is 4, it 4, the 4 | ch49/p19 | i 12, was 9, and 7, to 7, a 5, for 5 |
| ch08/p58 | and 17, to 13, had 12, mr 12, elton 8, he 8 | ch51/p32 | of 7, to 7, had 6, and 5, he 5, it 5 |

| leaf | the six heaviest words it says | leaf | the six heaviest words it says |
|---|---|---|---|
| ch19/p31 | i 18, to 18, and 11, so 10, her 8, it 8 | ch52/p13 | you 8, and 3, i 3, not 3, a 2, have 2 |
| ch23/p4 | the 25, to 13, and 12, of 12, a 10, she 9 | ch53/p36 | the 12, and 7, in 6, she 6, to 6, was 6 |
| ch24/p5 | the 32, of 20, and 15, he 12, was 11, a 9 | ch54/p20 | and 16, to 16, the 9, he 8, him 6, in 6 |

| leaf | the six heaviest words it says | leaf | the six heaviest words it says |
|---|---|---|---|
| ch24/p39 | and 6, her 4, to 4, we 4, a 3, have 3 | ch54/p24 | i 4, be 3, to 3, her 2, how 2, me 2 |
| ch26/p97 | and 2, bates 2, miss 2, that 2, this 2, are 1 | ch55/p2 | the 10, and 8, her 5, to 5, had 4, that 4 |
| ch27/p24 | the 9, and 8, a 7, at 4, she 4, to 4 | ch55/p9 | he 5, the 5, to 5, not 4, that 4, could 3 |

| leaf | the six heaviest words it says | leaf | the six heaviest words it says |
|---|---|---|---|
| ch38/p52 | for 3, i 3, to 3, and 2, chosen 2, elton 2 | story/ch1/p1 | to 4, he 2, she 2, that 2, the 2, was 2 |
| ch47/p33 | her 11, and 10, the 10, had 8, of 8, to 8 | | |

*Table 5, in parts. The 21 distinct winning leaves of Table 4, and the six words each says most.*

**The search works as a search.** Twenty-seven rays descended four levels and came out at 21 distinct paragraphs in 17 chapters of *Emma* plus the story's own first chapter, with between 8 and 27 of the eleven conditions said along a winning path and between 1 and 6 by the winning paragraph alone.

**Two stretches were drawn into the story's own earlier chapter.** Stretches 11 and 17 both reached `story/ch1/p1`, in the tree because only the story's *second* chapter was held out, and the smallest leaf either ray met. This is the one place in the 27 where the cast found the right *text*, and it found it for the wrong reason: stretch 11's attraction there is −40.963 nats, the worst of the 27. A small table smoothed toward a large base gives every condition a share far below its own, so the log ratios are large and negative, and the ranking by `said` put the scope first anyway.

**The attraction is usually negative, often strongly.** Of the 27, 22 are negative and 5 positive, and the positive ones — stretches 1, 7, 8, 26 and 27 — land on four of the longest leaves in Table 5, where `ch24/p5` says `the` 32 times and `ch48/p5` says `and` and `the` 19 each. The pattern is arithmetical, not semantic: a long paragraph has a large `total`, so its smoothed share of a common word stays above the base's, while a short paragraph's shares are dragged toward the base by the `+1` weight. The attraction reports paragraph length as much as fit, which is why it is reported rather than used.

**The leaves that win are full of function words.** Of the 162 entries in Table 5, counting repeats across the 27 casts, twelve are not function words, pronouns or auxiliaries: `mrs`, `goddard`, `miss` (`ch04/p8`, twice over), `chosen`, `elton` (`ch38/p52`), `mr`, `elton` (`ch08/p58`), `bates`, `miss` (`ch26/p97`).

### 5.1 The limit, stated plainly

The cast is drawn by what a paragraph **says**, and most of what these stretches say is `it was a his that`. A cast on function words finds paragraphs full of function words. Nothing in these 27 results is evidence that the story was placed where its meaning belongs, and the one result that looks like such evidence is not: stretch 1 reaching the paragraph of *Mrs Goddard* was decided by `and`, `had` and `a`, while `sunday` and `afternoon` — the two words of that stretch that could have carried any sense, said in 2 and 5 paragraphs of 2,314 — were outweighed.

This is not a matter of tuning. A six-word window of English is mostly made of words whose volumes enclose most of the scene, and the words that make this story new have no volume at all, so a test that sums over the asked conditions is dominated by the conditions there are most of. One could weight the sum by inverse document frequency or ask only for the rare conditions — each a knob beside the base, which this project's rule (docs/124, 137–138; paper 26 §9) says must be measured before it is believed. What is reported here is the base.

## 6. The forest proper

![The forest](fig/f155_forest.png)

*Figure 2. The forest: `scope:text` over two texts — *Emma*'s chapters on the left, this story on the right — with the first cast's six words drawn down into `emma/ch04/p8`. The figure's own label counts the two texts; `ScopeTree::roots` returns one, their parent. A node holds a distribution, not a box; the test is D4's movement in nats.*

`ScopeTree::roots` returns the scopes that were never a child, and on this run it returns one: `scope:text`. That is the measured value for tree 16, `roots: 1`, and it is one because both texts were read from the same directory, so the forest has a floor. Beneath it stand `scope:text/emma`, with 55 chapters and almost all of the 2,315 paragraphs, and `scope:text/story`, with two chapters and two paragraphs.

It is nonetheless a forest in the renderer's sense: a top-level acceleration structure is a BVH over several scenes, each with its own BVH, and the scenes know nothing of each other. The roots are never pruned — with an empty `under` the floor for the first level is `f64::MIN` and every root is scored, while from the second level down `min` applies — so a story is cast into every text at once and the one that says it most holds it, which is what happened to stretches 11 and 17. The forest gives no judgement between its scenes: the roots are scored by the same attraction, whose bias points toward *Emma*, where 25 of 27 casts went.

## 7. The sixteen trees

![The sixteen trees](fig/f155_trees.png)

*Figure 3. The sixteen trees and where a BVH sits among them. Number 1 is this paper's; 3 is the builder; 4 and 13 are levels of detail; 5 is the traversal's stack; 16 is the forest. Paper 29 takes the table at length.*

The scope tree is one of sixteen tree-shaped structures the system holds; the full table is in `trees.rs` and in paper 29. Five hold a renderer's apparatus between them: **1** the scope tree is the volumes and the traversal, **3** the strand's cut tree the builder, **4** the sixty-four levels and **13** the sphere's degrees the levels of detail, **5** the reading path the node stack, **16** the forest the top-level structure. None was written for the analogy: each is code with tests, and each gave a number on this story.

## 8. What the story goes on to

Two passages in this work are generated; they are the only generated text in it. The first continues the story from its own end, by the language with the story told, from its last twelve tokens, for forty words:

```
a strong and loveful machine that insulated his action that love. it was not
to be a very good- will." i am sure it was a source of high entertainment to
you, to be insensible to mirth. i am the readier to suspect, because,
```

The second is the story's first six tokens read by *Emma*'s language alone, with the story held out, for forty words:

```
it was on a sunday afternoon, and the, and, as to the purpose of taking her
opinion as to be done, than to go to mrs. goddard to whom he could apply for
information of her relations or friends. could
```

Both are `Knowing::go` (docs/149's path form). The first shows the cast landing where it should and the reading then falling off a cliff: its opening tokens are the story's own tail, so the cast is *heard*; then the story's words run out, and from the stray close-quote onward the text is Austen's register with no subject. The second shows the same without the story: `it was on a sunday afternoon` is stretch 1 of Table 4, drawn to `emma/ch04/p8`, and the text goes to *mrs. goddard*.

## 9. What this shows, and what it does not

**A told text enters the structures already there, with no registration**: nothing was configured and no schema extended, and the misspellings and the coinage are in the lexicon as words. **The volumes the tree fits over English prose are mostly not tight**: of the story's 58 words, sixteen are said in more than a fifth of the 2,314 paragraphs and thirteen in every chapter scope, while sixteen are said in none at all, so the cast cannot see them.

**It does not show that the story was placed where its meaning is, and it does not show understanding.** Twenty-two of 27 attractions are negative; the worst, −40.963 nats, is one of the two casts that found the right text; 150 of the 162 heaviest words of the winning leaves are function words or pronouns; and the one cast that looks semantic was decided by `and`, `had` and `a`. The structure can say where a sentence belongs, what company each word keeps and what it would go on to, but not what the story is about. Wittgenstein's remark that meaning is use (PI §43) is the hypothesis this apparatus tests, not a result it reports.

## Reproducing this paper

```
cargo run --release --bin forest -- data/text --story text/story/ch2
cargo run --release --bin strand -- data/text/story/ch2.txt \
    --prior data/text/emma --out data/forest/strand
cargo test -p phiplm a_word_is_drawn_to_the_scope        # the cast's own test
python3 docs/papers/figs155.py forest bvh trees          # figures 1–3
```

The first command writes `data/forest/forest.json`, from which every number here is taken; `docs/papers/forest/FOREST_FACTS.md` is the same numbers as tables, from the run of 2026-10-12.

## References

**The structure**, cited as an analogy, not an implementation. Clark, J. H. (1976). *Hierarchical geometric models for visible surface algorithms.* Comm. ACM 19(10). MacDonald, J. D., Booth, K. S. (1990). *Heuristics for ray tracing using space subdivision.* Visual Computer — the surface-area heuristic, of which the strand's cut tree is the information-theoretic analogue. Wald, I., Boulos, S., Shirley, P. (2007). *Ray tracing deformable scenes using dynamic BVHs.* ACM TOG — top-level structures over several scenes.

**The counts.** Witten, I. H., Bell, T. C. (1991). *The zero-frequency problem.* IEEE Trans. Inf. Theory — the smoothing of `Counts::dist` (D4; docs/126, 131). Spärck Jones, K. (1972). *A statistical interpretation of term specificity.* J. Documentation — the weighting §5.1 names as an untested knob. Austen, J. (1815). *Emma.* The story of §1 is the user's own, told once and quoted unaltered. Wittgenstein, L. (1953). *Philosophical Investigations*: §43 (meaning as use), §122 (surveyability), paraphrased; D1–D20 are papers 19–23.

**Phient's own record.** Papers 25 (Phasor Language Models), 26 (the world model), 27 and 29 onward; docs 124 (the knob rule), 126 (conditions at levels, where the BVH analogy is first written), 127–128 (the story as a tree of knowing), 131–138 (breadth, the store, held-out *Emma*), 140–141 (phikit), 149 (the levels as a path), 152–154 (the readers), 155 (the short form of this paper).
