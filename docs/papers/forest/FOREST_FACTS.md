# Measured facts — the forest reading of one story (bin/forest, docs/155)

Everything below was measured by `phiano/src/bin/forest` on 2026-10-12 and is in `data/forest/forest.json`. Do not invent numbers; quote these.

## The story (data/text/story/ch2.txt)

```
IT WAS ON A SUNDAY AFTERNOON WHEN IT FINALLY DOWNED ON OUR GENTLEMAN THAT HE HAS EXCEEDED THE THRESHOLD OF HOLDING ONTO HIS EGO ANDN HAPPENED TO BE IN A GREAT POSITION AND HE WAS IN THE PROCESS OF LAUNCHING HIS BUSINESS WITH HIS AMERICAN COUNTER PARTS IN THAT MOMENT IT SUDDENLY OCCURED TO HIS WITTY SENSES THAT SOMEHOW EVERTIHNG HAD ALWAYS WORKED FOR THE BEST OUTCOMES AND THAT MOVED HIS HEART TO BE A STRONG AND LOVEFUL MACHINE THAT INSULATED HIS ACTION THAT LOVE.
```

- 86 tokens, told as the single paragraph `scope:text/story/ch2/p1`.
- The language it was told into: Jane Austen's *Emma*, 2315 paragraphs in all (2314 without this story), 2,969,690 rows counted, 69,896 words in the tree.
- Tokens: it was on a sunday afternoon when it finally downed on our gentleman that he has exceeded the threshold of holding onto his ego andn happened to be in a great position and he was in the process of launching his business with his american counter parts in that moment it suddenly occured to his witty senses that somehow evertihng had always worked for the best outcomes and that moved his heart to be a strong and loveful machine that insulated his action that love .

## What the paragraph says beyond its parent (D4, nats)

| word | nats |
|---|---|
| his | 3.85 |
| that | 2.13 |
| a | 1.93 |
| and | 1.06 |
| in | 1.06 |
| it | 1.06 |
| action | 0.64 |
| afternoon | 0.64 |
| always | 0.64 |
| american | 0.64 |
| andn | 0.64 |
| best | 0.64 |

## The sixteen trees (trees.rs; each measured where a count is defined)

### 1 · the scope tree
- node: a scope: a text, a chapter, a paragraph — a word too, since a kind and a condition are words
- child: told by a `within` row; a scope says everything its subtree says
- ray: a few words and their pairs, cast from the roots down, entering only children that say them more than the whole tree does
- code: `phiplm::relate::tree (ScopeTree::cast_under, Cast)`
- as a BVH: the one the BVH analogy is written for: the volume is the scope's distribution over what it says, the hit test is D4's movement in nats, the pruning is the beam
- measured on this story: casts: 27
### 2 · the text tree
- node: a directory, a `.txt` file, a blank-line paragraph
- child: the file system's own nesting; a paragraph is a leaf
- ray: reading a text builds it; nothing is queried, it is the tree the scope tree is told from
- code: `phiplm::prose::read_tree / for_each_paragraph`
- as a BVH: the scene graph the volumes are fitted to — the authored hierarchy before any query
- measured on this story: own scope: scope:text/story/ch2/p1; paragraphs: 2315
### 3 · the strand's cut tree
- node: a span of tokens, `start..end`
- child: the division that gains the most nats against the parent's distribution, taken first
- ray: none: the tree is built once, then read at sixty-four resolutions
- code: `phiplm::prose::strand::cut (Cut::divide, Span)`
- as a BVH: the BVH *builder*: a top-down split that minimises a cost, the way SAH splits a scene
- measured on this story: tokens: 86
### 4 · the sixty-four levels
- node: a unit of one level, `AAA` (the word) to `TTT` (the whole story)
- child: the cut tree pruned to `m(k)` units, the counts spaced geometrically
- ray: a word asked at a level reads the unit holding it
- code: `phiplm::prose::strand (Strand::units, level, unit_of)`
- as a BVH: the BVH's levels of detail: one tree read at the resolution the query can afford
- measured on this story: levels: 64
### 5 · the reading path
- node: a unit on the path from the word up to the whole story
- child: the unit's parent in the cut tree
- ray: the word being read; its ancestors are mixed, weights counted for it
- code: `phiplm::prose::strand::path (Path, Scopes::path)`
- as a BVH: the node stack a ray traversal keeps — here the whole stack is read, not just the leaf
### 6 · the tree of knowing's levels
- node: a condition: a scope, the last word, the last pair, or a word under a scope
- child: finer than its parent; the path's scopes, then the word, then the pair
- ray: the words said so far; what the levels give is the next word's distribution
- code: `phiplm::relate::levels, relate::path (Weights::Path)`
- as a BVH: the traversal's result turned into an answer: every node on the path contributes, by a counted weight
- measured on this story: posts: 11
### 7 · the condition tree (X ⇒ Y under c)
- node: a word, and a condition which is itself a word
- child: a condition told under another condition — the recursion the paradigm names
- ray: a word plus the condition it is read under
- code: `phiecs::condition::Under, World::calls_for_under`
- as a BVH: nested volumes with no geometry: the enclosure is `told under`, not `contains`
- measured on this story: under: 83
### 8 · the quest's walk
- node: a word asked of the oracle
- child: a word the answer named, at depth + 1
- ray: the frontier: the quest walks outward from the seed
- code: `phiecs::quest (Quest::next_word, push, DEFAULT_DEPTH)`
- as a BVH: the build queue of a BVH built lazily: nodes exist once something asks for them
### 9 · the code tree
- node: a crate, a module, a function, a token of its body
- child: the compiler's own nesting, read as `within` rows
- ray: a function's words, cast to find where a new function belongs
- code: `phiplm::code`
- as a BVH: the same scope tree over a different text; the grammar fits the volumes exactly
### 10 · the kin tree
- node: a word
- child: not nesting but neighbourhood: the words with the closest left and right company
- ray: a word never told after the last one; its kin answer in its place
- code: `phiplm::relate::kin (Counts::kin, kin_base)`
- as a BVH: the fallback when a ray misses every volume: ask what lies near instead
- measured on this story: never heard: 16
### 11 · the phasor's channels
- node: a word
- child: 64 relation channels, each an angle and a count
- ray: two words compared on a channel (agreement) or over all of them (resonance)
- code: `phiecs::phasor::Phasor, phical::CHANNELS`
- as a BVH: the node's *bounds*, if a BVH stored orientation instead of extent: a word has no position, only angles
- measured on this story: edges: 563; words: 58
### 12 · the helix
- node: a directory of the facet's tree, coded as a codon
- child: the channel's place on the helix: 64 codons, three bases of four
- ray: `ls`, `grep`, `cat` — the file system is the query language
- code: `phiano::helix`
- as a BVH: a BVH written to disk: the path names the node, so a lookup is a path walk
### 13 · the sphere's degrees
- node: a degree `l` of the spherical harmonics
- child: its `2l+1` orders, tied to one weight
- ray: a word read at a lower degree is the word at a coarser resolution
- code: `phifno::sphere, phical::Tie::{Sphere, Anisotropic}`
- as a BVH: mip levels: one object at many resolutions, the coarse level a bound on the fine one
### 14 · the operator's tape
- node: a value on the autograd tape
- child: the operation that made it, with its inputs
- ray: the backward pass walks it from the loss
- code: `phinet::tensor (Tape, Op, backward)`
- as a BVH: not a BVH but the same discipline: a node knows its children, and the walk visits each once
### 15 · the Fourier modes
- node: a mode of the operator's spectral weight
- child: the modes kept below the cutoff, the rest zeroed by the mask
- ray: a function passed through: low modes carry it, high modes must earn their place
- code: `phical::operator (Fno, modes, mask_modes)`
- as a BVH: the coarse bound again, in frequency: the low modes are the volume the detail sits inside
### 16 · the forest
- node: a whole told text with its own root
- child: the roots standing beside each other under no parent
- ray: a story cast into every root at once; the one that says it most holds it
- code: `phiplm::relate::tree (ScopeTree::roots), this binary`
- as a BVH: the top-level acceleration structure over several scenes — a BVH of BVHs, which is what this story in Emma is
- measured on this story: emma paragraphs: 2314; roots: 1

## Bounding volumes: how much of Emma holds each word

| word | Emma paragraphs that say it | chapters |
|---|---|---|
| it | 1163 | 56 |
| was | 928 | 56 |
| on | 507 | 56 |
| a | 1320 | 55 |
| sunday | 2 | 2 |
| afternoon | 5 | 5 |
| when | 296 | 52 |
| finally | 0 | 0 |
| downed | 0 | 0 |
| our | 79 | 34 |
| gentleman | 23 | 16 |
| that | 995 | 56 |
| he | 775 | 56 |
| has | 195 | 43 |
| exceeded | 0 | 0 |
| the | 1486 | 56 |
| threshold | 0 | 0 |
| of | 1446 | 56 |
| holding | 1 | 1 |
| onto | 0 | 0 |
| his | 560 | 55 |
| ego | 0 | 0 |
| andn | 0 | 0 |
| happened | 27 | 19 |
| to | 1630 | 56 |
| be | 1039 | 56 |
| in | 1073 | 56 |
| great | 232 | 54 |
| position | 0 | 0 |
| and | 1524 | 56 |
| process | 2 | 1 |
| launching | 0 | 0 |
| business | 53 | 31 |
| with | 772 | 56 |
| american | 0 | 0 |
| counter | 2 | 2 |
| parts | 2 | 2 |
| moment | 105 | 37 |
| suddenly | 7 | 4 |
| occured | 0 | 0 |
| witty | 2 | 2 |
| senses | 4 | 4 |
| somehow | 5 | 4 |
| evertihng | 0 | 0 |
| had | 726 | 56 |
| always | 204 | 52 |
| worked | 2 | 1 |
| for | 778 | 55 |
| best | 82 | 41 |
| outcomes | 0 | 0 |
| moved | 7 | 7 |
| heart | 61 | 36 |
| strong | 41 | 28 |
| loveful | 0 | 0 |
| machine | 0 | 0 |
| insulated | 0 | 0 |
| action | 6 | 6 |
| love | 101 | 42 |

## Casts: 27 stretches of six words, three apart, cast into Emma alone

- **"it was on a sunday afternoon"** → `scope:text/emma/ch04/scope:text/emma/ch04/p8` · 27 conditions said along the path, 6 by the leaf alone, attraction 0.055. That leaf says: and (4), mrs (4), goddard (3), had (3), miss (3), a (2)
- **"a sunday afternoon when it finally"** → `scope:text/emma/ch04/scope:text/emma/ch04/p8` · 20 conditions said along the path, 4 by the leaf alone, attraction -0.138. That leaf says: and (4), mrs (4), goddard (3), had (3), miss (3), a (2)
- **"when it finally downed on our"** → `scope:text/emma/ch52/scope:text/emma/ch52/p13` · 15 conditions said along the path, 3 by the leaf alone, attraction -3.770. That leaf says: you (8), and (3), i (3), not (3), a (2), have (2)
- **"downed on our gentleman that he"** → `scope:text/emma/ch55/scope:text/emma/ch55/p9` · 16 conditions said along the path, 4 by the leaf alone, attraction -25.498. That leaf says: he (5), the (5), to (5), not (4), that (4), could (3)
- **"gentleman that he has exceeded the"** → `scope:text/emma/ch38/scope:text/emma/ch38/p52` · 18 conditions said along the path, 5 by the leaf alone, attraction -5.319. That leaf says: for (3), i (3), to (3), and (2), chosen (2), elton (2)
- **"has exceeded the threshold of holding"** → `scope:text/emma/ch24/scope:text/emma/ch24/p39` · 8 conditions said along the path, 1 by the leaf alone, attraction -10.396. That leaf says: and (6), her (4), to (4), we (4), a (3), have (3)
- **"threshold of holding onto his ego"** → `scope:text/emma/ch24/scope:text/emma/ch24/p5` · 8 conditions said along the path, 2 by the leaf alone, attraction 6.526. That leaf says: the (32), of (20), and (15), he (12), was (11), a (9)
- **"onto his ego andn happened to"** → `scope:text/emma/ch49/scope:text/emma/ch49/p19` · 11 conditions said along the path, 3 by the leaf alone, attraction 8.763. That leaf says: i (12), was (9), and (7), to (7), a (5), for (5)
- **"andn happened to be in a"** → `scope:text/emma/ch19/scope:text/emma/ch19/p31` · 22 conditions said along the path, 6 by the leaf alone, attraction -6.208. That leaf says: i (18), to (18), and (11), so (10), her (8), it (8)
- **"be in a great position and"** → `scope:text/emma/ch51/scope:text/emma/ch51/p32` · 18 conditions said along the path, 4 by the leaf alone, attraction -8.167. That leaf says: of (7), to (7), had (6), and (5), he (5), it (5)
- **"great position and he was in"** → `scope:text/story/ch1/scope:text/story/ch1/p1` · 16 conditions said along the path, 3 by the leaf alone, attraction -40.963. That leaf says: to (4), he (2), she (2), that (2), the (2), was (2)
- **"he was in the process of"** → `scope:text/emma/ch23/scope:text/emma/ch23/p4` · 23 conditions said along the path, 4 by the leaf alone, attraction -14.145. That leaf says: the (25), to (13), and (12), of (12), a (10), she (9)
- **"the process of launching his business"** → `scope:text/emma/ch23/scope:text/emma/ch23/p4` · 20 conditions said along the path, 3 by the leaf alone, attraction -10.270. That leaf says: the (25), to (13), and (12), of (12), a (10), she (9)
- **"launching his business with his american"** → `scope:text/emma/ch08/scope:text/emma/ch08/p58` · 20 conditions said along the path, 5 by the leaf alone, attraction -12.990. That leaf says: and (17), to (13), had (12), mr (12), elton (8), he (8)
- **"with his american counter parts in"** → `scope:text/emma/ch27/scope:text/emma/ch27/p24` · 16 conditions said along the path, 4 by the leaf alone, attraction -14.494. That leaf says: the (9), and (8), a (7), at (4), she (4), to (4)
- **"counter parts in that moment it"** → `scope:text/emma/ch26/scope:text/emma/ch26/p97` · 19 conditions said along the path, 4 by the leaf alone, attraction -28.868. That leaf says: and (2), bates (2), miss (2), that (2), this (2), are (1)
- **"that moment it suddenly occured to"** → `scope:text/story/ch1/scope:text/story/ch1/p1` · 16 conditions said along the path, 3 by the leaf alone, attraction -10.973. That leaf says: to (4), he (2), she (2), that (2), the (2), was (2)
- **"suddenly occured to his witty senses"** → `scope:text/emma/ch54/scope:text/emma/ch54/p24` · 13 conditions said along the path, 2 by the leaf alone, attraction -37.133. That leaf says: i (4), be (3), to (3), her (2), how (2), me (2)
- **"his witty senses that somehow evertihng"** → `scope:text/emma/ch49/scope:text/emma/ch49/p19` · 13 conditions said along the path, 3 by the leaf alone, attraction -24.658. That leaf says: i (12), was (9), and (7), to (7), a (5), for (5)
- **"that somehow evertihng had always worked"** → `scope:text/emma/ch55/scope:text/emma/ch55/p2` · 14 conditions said along the path, 3 by the leaf alone, attraction -19.890. That leaf says: the (10), and (8), her (5), to (5), had (4), that (4)
- **"had always worked for the best"** → `scope:text/emma/ch53/scope:text/emma/ch53/p36` · 22 conditions said along the path, 5 by the leaf alone, attraction -30.721. That leaf says: the (12), and (7), in (6), she (6), to (6), was (6)
- **"for the best outcomes and that"** → `scope:text/emma/ch55/scope:text/emma/ch55/p2` · 16 conditions said along the path, 3 by the leaf alone, attraction -20.541. That leaf says: the (10), and (8), her (5), to (5), had (4), that (4)
- **"outcomes and that moved his heart"** → `scope:text/emma/ch54/scope:text/emma/ch54/p20` · 18 conditions said along the path, 4 by the leaf alone, attraction -3.127. That leaf says: and (16), to (16), the (9), he (8), him (6), in (6)
- **"moved his heart to be a"** → `scope:text/emma/ch54/scope:text/emma/ch54/p20` · 22 conditions said along the path, 5 by the leaf alone, attraction -15.264. That leaf says: and (16), to (16), the (9), he (8), him (6), in (6)
- **"to be a strong and loveful"** → `scope:text/emma/ch47/scope:text/emma/ch47/p33` · 20 conditions said along the path, 5 by the leaf alone, attraction -15.591. That leaf says: her (11), and (10), the (10), had (8), of (8), to (8)
- **"strong and loveful machine that insulated"** → `scope:text/emma/ch07/scope:text/emma/ch07/p5` · 10 conditions said along the path, 3 by the leaf alone, attraction 8.562. That leaf says: a (9), i (6), and (5), is (4), it (4), the (4)
- **"machine that insulated his action that"** → `scope:text/emma/ch48/scope:text/emma/ch48/p5` · 9 conditions said along the path, 2 by the leaf alone, attraction 3.628. That leaf says: and (19), the (19), had (18), to (17), of (15), her (9)

## Every word of the story, as the forest holds it

| word | Emma says it | post (nats) | next in Emma | next here | kin |
|---|---|---|---|---|---|
| it | 2530 | 1.54 | was, is, ., , | finally, suddenly, was | there, he, she, this |
| was | 2403 | 1.50 | not, a, the, very | in, on | is, seems, does, did |
| on | 693 | 1.72 | the, her, ,, his | a, our | in, by, contemplating, during |
| a | 3130 | 2.63 | very, little, great, few | great, strong, sunday | the, so, discussed, some |
| sunday | 2 | 3.56 | ,, any | afternoon | tradesman, knife, seminary, receipt |
| afternoon | 6 | 3.46 | -, and, it, next | when | sale, lime, militia, elegancies |
| when | 364 | 1.82 | he, she, i, the | it | that, if, while, though |
| finally | 0 | 0.00 |  | downed |  |
| downed | 0 | 0.00 |  | on |  |
| our | 99 | 4.89 | good, lives, little, _ | gentleman | your, their, his, bending |
| gentleman | 27 | 2.13 | -, in, !, . | that | crowd, personage, push, bluntness |
| that | 1802 | 1.28 | she, he, i, it | he, insulated, love | when, though, because, which |
| he | 1813 | 2.21 | had, was, is, could | has, was | she, there, it, who |
| has | 252 | 2.69 | been, not, a, he | exceeded | had, does, did, was |
| exceeded | 0 | 0.00 |  | the |  |
| the | 5206 | 2.40 | same, very, world, other | best, process, threshold | a, their, some, his |
| threshold | 0 | 0.00 |  | of |  |
| of | 4294 | 1.50 | the, her, his, a | holding, launching | in, by, on, from |
| holding | 1 | 11.50 | cheap | onto | eligibly, clerk, insulted, prone |
| onto | 0 | 0.00 |  | his |  |
| his | 1150 | 2.97 | own, father, wife, being | action, american, business | your, their, her, its |
| ego | 0 | 0.00 |  | andn |  |
| andn | 0 | 0.00 |  | happened |  |
| happened | 28 | 1.93 | ., to, ;, that | to | ceased, calculated, preconceived, known |
| to | 5248 | 1.81 | be, the, her, have | be, his | in, may, for, by |
| be | 1977 | 1.82 | a, the, so, very | a, in | spend, make, avoid, write |
| in | 2189 | 1.85 | the, a, her, his | a, that, the | by, on, contemplating, during |
| great | 265 | 4.22 | deal, pleasure, a, many | position | vast, good, donkey, minute |
| position | 0 | 0.00 |  | and |  |
| and | 4898 | 0.94 | the, i, she, , | he, loveful, that | but, though, till, that |
| process | 2 | 2.91 | -, of | of | sale, result, shadow, rapidity |
| launching | 0 | 0.00 |  | his |  |
| business | 55 | 1.67 | ., ,, to, was | with | purpose, plan, subject, comfort |
| with | 1219 | 1.70 | the, a, her, all | his | by, in, for, from |
| american | 0 | 0.00 |  | counter |  |
| counter | 2 | 2.80 | , | parts | dialogues, medley, departed, fourth |
| parts | 2 | 2.62 | ,, of | in | suddenness, bottom, dignity, garden |
| moment | 111 | 1.53 | ,, ., -, ; | it | letter, fortnight, morning, speech |
| suddenly | 7 | 5.12 | ;, had, happened, it | occured | believed, liked, intends, keenly |
| occured | 0 | 0.00 |  | to |  |
| witty | 2 | 7.17 | things, word | senses | stranger, sharp, wonderful, little |
| senses | 4 | 2.56 | ,, and | that | brother, uncle, merits, son |
| somehow | 5 | 5.48 | or, there | evertihng | friday, bake, negligence, win |
| evertihng | 0 | 0.00 |  | had |  |
| had | 1628 | 2.18 | been, not, a, no | always | has, could, did, have |
| always | 238 | 2.64 | the, been, at, be | worked | really, never, quite, called |
| worked | 2 | 7.17 | hard, such | for | accompanied, reached, received, makes |
| for | 1347 | 1.48 | the, her, a, his | the | by, with, in, on |
| best | 85 | 3.12 | of, ., ,, to | outcomes | sound, mantelpiece, rest, recollection |
| outcomes | 0 | 0.00 |  | and |  |
| moved | 7 | 5.39 | away, a, about, back | his | driven, remonstrated, disappointments, received |
| heart | 66 | 1.68 | ., ,, and, was | to | voice, father, mind, friend |
| strong | 44 | 3.61 | ., a, and, as | and | formal, formidable, good, bad |
| loveful | 0 | 0.00 |  | machine |  |
| machine | 0 | 0.00 |  | that |  |
| insulated | 0 | 0.00 |  | his |  |
| action | 6 | 4.36 | ,, ;, abroad, had | that | january, highbury, london, fact |
| love | 118 | 1.75 | with, ,, ., ; | . | charity, housekeeping, infancy, blossom |

## The 16 words Emma never heard, and what fills the slot

| word | left context | the word after | best from the left alone | best between both sides |
|---|---|---|---|---|
| finally | …afternoon when it | downed | was (-1.6), is (-1.9), came (-2.6) | comes (-6.9), burst (-7.0), stopped (-7.3) |
| downed | …when it finally | on | to (-3.6), the (-3.7), and (-3.7) | ⟨end⟩ (-5.1), and (-9.2), go (-9.9) |
| exceeded | …that he has | the | been (-2.0), not (-2.5), a (-3.3) | been (-5.1), outstepped (-5.7), ⟨end⟩ (-6.2) |
| threshold | …has exceeded the | of | very (-4.1), same (-4.2), world (-4.5) | rest (-5.6), idea (-5.8), sort (-6.0) |
| onto | …threshold of holding | his | cheap (-1.1), to (-4.0), the (-4.1) | cheap (-2.9), ⟨end⟩ (-5.5), of (-7.3) |
| ego | …holding onto his | andn | own (-3.0), father (-3.8), wife (-4.0) | ⟨end⟩ (-6.3), alarms (-8.5), wife's (-8.5) |
| andn | …onto his ego | happened | to (-3.6), the (-3.7), and (-3.7) | ⟨end⟩ (-5.1), had (-10.2), has (-10.2) |
| position | …in a great | and | deal (-1.0), many (-2.8), regard (-2.9) | regard (-5.5), deal (-6.2), dreamer (-6.2) |
| launching | …the process of | his | a (-1.3), the (-2.7), her (-3.5) | ⟨end⟩ (-6.3), of (-8.1), all (-8.9) |
| american | …business with his | counter | own (-3.1), father (-3.5), wife (-3.6) | ⟨end⟩ (-6.2), tray (-6.2), muttering (-6.6) |
| occured | …moment it suddenly | to | happened (-1.3), it (-3.2), had (-3.2) | happened (-2.7), perceived (-5.3), opened (-5.8) |
| evertihng | …senses that somehow | had | or (-0.6), there (-1.9), to (-4.9) | there (-5.5), or (-6.4), ⟨end⟩ (-6.4) |
| outcomes | …for the best | and | of (-2.0), to (-3.6), friend (-3.8) | excuses (-5.8), size (-5.8), ⟨end⟩ (-5.8) |
| loveful | …a strong and | machine | steady (-2.9), unaffected (-2.9), concise (-2.9) | concise (-4.7), unaffected (-5.2), steady (-6.1) |
| machine | …strong and loveful | that | to (-3.6), the (-3.7), and (-3.7) | ⟨end⟩ (-5.1), and (-7.8), all (-7.9) |
| insulated | …loveful machine that | his | she (-2.4), he (-2.5), i (-2.8) | ⟨end⟩ (-6.9), of (-7.9), in (-8.1) |

## The story told to a phiecs world

- 58 words (entities), 563 edges, 83 relations told under a condition (`rising`, `resolving`).
- Kinds used: `before`, `after` (each pair), `near` (every pair within a stretch of six).

The most resonant pairs (the cosine over every channel both have moved):

| a | b | resonance |
|---|---|---|
| the | of | 0.973 |
| holding | launching | 0.952 |
| moved | love | 0.952 |
| senses | action | 0.948 |
| gentleman | to | 0.947 |
| of | witty | 0.943 |
| it | gentleman | 0.941 |
| his | witty | 0.937 |
| and | suddenly | 0.931 |
| afternoon | loveful | 0.927 |
| the | witty | 0.920 |
| parts | moved | 0.919 |

Some words' channels (angle in radians, amplitude ln(1+evidence)):

| word | channel | theta | amplitude | evidence |
|---|---|---|---|---|
| it | near | 5.056 | 3.434 | 30 |
| it | before | 1.200 | 1.792 | 5 |
| it | after | 0.375 | 1.792 | 5 |
| was | near | 5.477 | 3.045 | 20 |
| was | before | 3.513 | 1.609 | 4 |
| was | after | 2.769 | 1.609 | 4 |
| on | near | 5.147 | 3.045 | 20 |
| on | before | 1.792 | 1.609 | 4 |
| on | after | 4.243 | 1.609 | 4 |
| a | near | 4.871 | 3.434 | 30 |
| a | before | 0.875 | 1.946 | 6 |
| a | after | 2.492 | 1.946 | 6 |

Relations under a condition, for the first few words:

- `it` under `rising` calls for was (1.0), finally (1.0)
- `it` under `resolving` calls for suddenly (1.0)
- `was` under `rising` calls for on (1.0), in (1.0)
- `on` under `rising` calls for a (1.0), our (1.0)
- `a` under `rising` calls for sunday (1.0), great (1.0)
- `a` under `resolving` calls for strong (1.0)
- `sunday` under `rising` calls for afternoon (1.0)
- `afternoon` under `rising` calls for when (1.0)

## The trained readers on this story (operators trained on Emma alone)

- **Joint FNO, 32 epochs**: of 84 positions, 6 read the next word first; 6 read the word *before* first, by the conjugate filter, which was never trained.
- **mirror, 128 epochs**: of 84 positions, 6 read the next word first; 4 read the word *before* first, by the conjugate filter, which was never trained.

A few positions in detail:


**Joint FNO, 32 epochs**

| after | truth | forward rank | its top 3 | backward rank |
|---|---|---|---|---|
| it | was | 0 | was, would, is | 1 |
| was | on | 85 | quite, not, so | 5942 |
| on | a | 6 | his, the, her | 1000 |
| a | sunday | 602 | great, little, few | 1 |
| sunday | afternoon | 1555 | ., ,, her | 1331 |
| afternoon | when | 43 | ., of, , | 2502 |
| when | it | 3 | she, he, they | 245 |
| it | finally | None | was, would, is | None |
| finally | downed | None |  | None |
| downed | on | None |  | None |

**mirror, 128 epochs**

| after | truth | forward rank | its top 3 | backward rank |
|---|---|---|---|---|
| it | was | 2 | is, would, was | 1 |
| was | on | 81 | a, the, so | 3768 |
| on | a | 9 | the, mr, this | 13 |
| a | sunday | 383 | most, little, man | 6 |
| sunday | afternoon | 284 | ;, on, any | 1233 |
| afternoon | when | 23 | ;, ?, ! | 407 |
| when | it | 24 | he, miss, i | 195 |
| it | finally | None | is, would, was | None |
| finally | downed | None |  | None |
| downed | on | None |  | None |

## The story gone on with (generated — the only generated text here)

From its own end, inside its own chapter:

```
a strong and loveful machine that insulated his action that love. it was not to be a very good- will." i am sure it was a source of high entertainment to you, to be insensible to mirth. i am the readier to suspect, because,
```

Its opening, read by Emma's language alone (the story held out):

```
it was on a sunday afternoon, and the, and, as to the purpose of taking her opinion as to be done, than to go to mrs. goddard to whom he could apply for information of her relations or friends. could
```
