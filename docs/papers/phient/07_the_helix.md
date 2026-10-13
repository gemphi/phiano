# The helix
*Phient papers, 7 of 16 · 2026-04-06 · The helix: the lexicon as a DNA-coded directory tree — 64 codons, two strands, a position and a colour for every code*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

The helix is a file format for a Phiano lexicon. `bin/helix write` takes a facet saved as a `.chroma` and writes it as a directory tree. Every word becomes a text file of 64 lines, one line per relation channel, and every channel is named by a codon: three bases over `A`, `C`, `G`, `T`. Each line carries the word's source byte and target byte on that channel, which are the two strands, then the evidence count on that channel, then PhiTon's reading of the source angle as a wavelength, a spectral band and a colour. One table, `codons.tsv`, gives each codon its channel number, a hexagram glyph, a spherical-harmonic degree and order, a point on a B-DNA helix, and the facet's own name for the relation. `bin/helix read` turns the tree back into a facet.

The module is `phiano::helix`, in four files: `mod.rs`, `code.rs`, `write.rs` and `read.rs`. The binary is `src/bin/helix/main.rs`. The PhiTon half lives in `src/phiton`.

What was measured is a write and a round trip. The persona lexicon, 18,496 words with 24 marker relations, writes to 75 MB. docs/84 gives the write time as 0.6 s. PHIENT_FACTS.md gives 1.9 s. The later document is PHIENT_FACTS.md, and the strand line it describes carries the PhiTon columns that docs/84's layout does not show, so this paper takes 1.9 s. The round trip is byte-exact on the source side, the target side, the evidence and the relation table. On the synthetic quest's lexicon, the word `w0` has evidence on exactly six codons: its five relations and the first offset channel.

What was not measured is any claim about language. The helix trains nothing, predicts nothing and changes no score. Whether a codon's place on the helix, its degree on the sphere or its hexagram carries meaning about the relation on that channel is not measured. The paper says so wherever it matters.

The paper also reports what reading the code turned up. The hexagram glyph printed for a codon is Unicode's hexagram in position `k`, which follows the King Wen order and does not draw `k`'s six bits as the doc comment says. The amplitude in a strand header is printed to six decimals and is outside the round-trip test. The reader drops any marker relation whose name begins with `ch` or `offset`. None of these has touched a measured number yet. All three are fixable in a few lines.

The rest of the paper follows the arc of the series. Sections 3 and 6 to 10 read the code. Section 4 holds the numbers. Sections 11 to 15 place the helix in the family and say what is left.

## 2. The claim

The claim is small and exact. Four things in the project come in sixty-fours. A `SpectralPhasor` has 64 relation channels. The real spherical harmonics of degree below 8 number 64, since `Σ_{l<8}(2l+1) = 64`. The I Ching has 64 hexagrams, which `phinet::hexagram` already uses as gates on a spectrum's six octaves. DNA has 64 codons, three bases of four. Because the counts agree, one index `k` in `0..64` can carry all four names at once, and one table can print them side by side. The helix is that table plus a directory tree laid out on it.

The module's doc comment says the correspondence is exact in number and, in its words, "this module argues", in kind. DNA stores no point: a gene is codons, and a codon's meaning is fixed by where it sits. Phical stores no point: a word is relations, and a relation's meaning is fixed by what it relates to. This paper keeps the two halves of that sentence apart. The count is a fact about the code. The kind is an analogy. docs/84 draws the line itself: the helix is a file format whose 64-fold code lines up with three others by count, and it makes no claim that language is DNA.

The second claim is practical. In a `.chroma`, a word's relations are eight `u64` words of source angles, eight of target angles and thirty-two of evidence counts, in binary. Nobody reads that without the program. The helix writes the same bits as text, one file per word, sharded by the word's first two characters, so that `ls`, `cat`, `grep` and `awk` become the query language.

The third claim is that the tree loses nothing the lexicon needs. `read` rebuilds every phasor's source side, target side and evidence, and rebuilds the relation table from `codons.tsv`. A unit test checks that they come back equal. N-grams are outside the helix and stay in the `.chroma`.

The fourth claim is the one the user asked for: every code is a position in space. `helix_position(k)` puts codon `k` at angle `2πk / 10.5` and height `0.34k` nanometres on a helix of radius 1. The claim is true of the tree by construction. It is a statement about where the file puts a code. Whether that place says anything about the relation stored there is a separate question, and it was not measured.

The fifth claim came one doc later. docs/85 asks that the engine stop reporting bare numbers where the family already has a unit. The helix is the first shipped reader of PhiTon's phase-to-colour conversion, so a codon's source angle is printed as a wavelength and a named band beside its byte.

## 3. The mechanism

`code.rs` holds the code. `BASES` is `['A', 'C', 'G', 'T']`, in the order that makes a codon the base-four digits of its channel. `CODONS` is 64. `PITCH_PER_TURN` is 10.5 codons per turn and `RISE` is 0.34 nm per codon, the B-DNA values. A private `RADIUS` is 1.0 nm. `codon(k)` reduces `k` modulo 64 and returns `BASES[k/16]`, `BASES[(k/4)%4]`, `BASES[k%4]`, so `k = 16a + 4b + c`. `codon_index(s)` inverts it, accepts lower case, and returns `None` for a string that is not three bases. `hexagram_glyph(k)` returns the character at `U+4DC0 + k`. `helix_position(k)` returns `[cos θ, sin θ, 0.34k]` with `θ = 2πk / 10.5`. The struct `Codon` gathers `k`, `codon`, `hexagram`, `degree`, `order` and `position`. `Codon::of(k)` fills it, taking degree and order from `phinet::sphere::degree_of` and `order_of`, and `Codon::table()` returns all 64.

`write.rs` holds `write(facet, dir)`, which returns the number of strands written. It creates `strands/` and `codons/`. It writes `codons.tsv`, one row per `Codon`, with `facet.relation_name(k)` as the last column. It writes `README.md` from `readme(facet)`. Then it sorts the lexicon's words, so the tree is reproducible, and writes one strand per word to `strands/<shard>/<file>.dna`, where `shard_of` takes the first two alphanumeric characters, lower-cased, and `file_name` keeps ASCII letters, digits and `-` and escapes the rest as `%` and hex. A strand's first line is `# <word>`, then `amplitude`, `band` and `count`. Each of the next 64 lines is the codon, `theta_byte(k)`, the target byte, `evidence_count(k)`, and the `PhitonColor` from `PhitonSpectrum::phase_to_color(theta(k), band_n)`: wavelength to one decimal, band name, hex colour. While it writes, `write` collects every word with evidence on each channel. At the end it writes `codons/<CODON>/words.tsv`, heaviest evidence first, ties broken by the word, capped at `CODON_WORDS`, which is 200.

`read.rs` holds `read(dir)`. It starts from `Facet::new()`. It parses `codons.tsv`, keeps rows with `k ≥ 8` whose relation name does not begin with `ch` or `offset`, and passes them to `RelationVocab::from_entries`. It walks the shards and files in sorted order. For each file, `strand(text)` reads the header fields, then each line's first four columns, and packs them: the source byte into `packed[k/8]` at bit `8·(k%8)`, the target byte into `tgt[k/8]` the same way, the evidence into `evid[k/2]` at bit `32·(k%2)`. `SpectralPhasor::from_packed_sides` builds the phasor. The PhiTon columns are written for people and ignored on read.

`bin/helix` is forty lines. `helix write <facet.chroma> <dir>` loads with `Storage::load`, times `helix::write`, and prints the strand count, 64 codons, the marker count and the seconds. `helix read <dir> <facet.chroma>` calls `helix::read` and saves with `Storage::save`. Anything else prints the usage line and exits with code 2.

## 4. What was measured and the numbers

The measurements are all about the file, and each is listed with its source.

| what | number | source |
|---|---|---|
| persona lexicon, words | 18,496 | docs/84, PHIENT_FACTS.md |
| persona lexicon, marker relations | 24 | docs/84 |
| tree size on disk | 75 MB | docs/84, PHIENT_FACTS.md |
| write time | 0.6 s | docs/84 |
| write time | 1.9 s | PHIENT_FACTS.md |
| round trip | byte-exact on source side, target side, evidence and relation table | docs/84 (tested), PHIENT_FACTS.md |
| quest word `w0`, active codons | `AGA`, `AGC`, `AGG`, `AGT`, `ATA` and offset+1 | docs/84 |
| `love`, codon `AAA`, PhiTon reading | 418.5 nm, indigo, `#5900F8` | docs/85 |
| B-DNA geometry used | 10.5 codons per turn, 0.34 nm rise | `code.rs` |
| codon directory length | at most 200 words | `write.rs` |

The two write times disagree. docs/84 describes a strand line of four fields, codon, source byte, target byte and evidence, and reports 0.6 s. `write.rs` and PHIENT_FACTS.md describe a strand line of seven fields, with wavelength, band and hex colour added, and PHIENT_FACTS.md reports 1.9 s. docs/85 records that the PhiTon columns were added after docs/84. The later figure is 1.9 s and this paper uses it. The cause of the difference is plausibly the added colour work and the longer lines, but the two versions were not timed side by side, so the cause is not measured. Both documents give 75 MB.

The round trip has two levels of evidence. The unit test `a_helix_round_trips_the_lexicon` in `read.rs` builds a facet from three toy sentences, "the cat sat on the mat", "a dog eats a donut" and "cat of mat", runs `Trainer::couple_relations` three times over them, writes, reads back, and asserts equality of `packed()`, `packed_target()`, `packed_evidence()`, `count` and the relation table's entries, and that `codons/AAA/words.tsv` exists. PHIENT_FACTS.md states the byte-exact round trip beside the 18,496-word figure. The test does not compare `amplitude` or `band_n`. Section 13 returns to the amplitude.

The quest result is a reading of an existing lexicon. The quest's oracle has five relation kinds, is-a, part-of, near, used-for and unlike. docs/84 reports that `w0`'s active codons are exactly its five relations plus offset+1, which is the quest read as a gene. No MRR was computed through the helix.

Nothing here measures meaning. No experiment asks whether words near each other on a codon's list are related, whether codons close on the helix carry related relations, or whether the hexagram of a codon predicts anything. docs/74 had already measured hexagrams as predictors elsewhere and refuted them, 0.9% against chance 1.6%. The helix inherits that result as a caution.

## 5. The one idea restated

A lexicon in Phiano is a set of words, and a word is 64 relations, each a byte of angle on each of two sides with a count. The helix gives each of the 64 relations a three-letter name and a place, and writes each word as the list of its 64 relations under those names. That is the whole idea.

It follows from the paradigm with nothing added. Phical stores no point, so a word has no coordinates of its own. It has 64 channels, and on each channel it has an angle it presents when it is the word doing the relating, an angle it presents when it is the word related to, and how often that channel has seen it. A file with one line per channel and those three numbers per line is the word, complete, with nothing summarised or projected. The format is a transcription of the phasor. It is a text file a person can open.

The codon names are one choice of label among many. They were picked because they are short, they are three characters wide in a column, they sort, and they carry the base-four digits of the channel, so `AGG` is channel 10 to anyone who knows the rule. The spherical degree and order, the hexagram and the helix point are printed beside the codon because the same index gives them for free. They are four readings of one integer. The table in `codons.tsv` is where a reader can see all four and the facet's relation name at once.

The two strands carry the paradigm's direction. X ⇒ Y has a side for X and a side for Y. Since docs/69 every phasor keeps a source angle and a target angle per channel, and `couple_relations` moves the earlier word's source toward the later word's target. The helix writes these as columns two and three of each line. The word's DNA is double-stranded because the relation is directed.

The colour is the unit. A byte of angle means nothing to a reader. `PhitonSpectrum::phase_to_color` maps the same angle to a wavelength between 380 and 750 nm, names the band it falls in, and gives an RGB colour. docs/85 calls this the instruction not to recreate `f64`: where the family has a unit, use it. So each codon line reads as a spectral line, and a strand of 64 codons reads as a spectrum.

The position is the last reading and the thinnest. `helix_position` puts channel `k` on a B-DNA helix. The only claim is that the tree now has a geometry in which every code has a place. The idea asks nothing more of it, and nothing more was measured.

## 6. The code table

`codons.tsv` has a header and 64 rows. The header is `k`, `codon`, `hexagram`, `degree`, `order`, `x`, `y`, `z`, `relation`. Here are eight rows. Every column but the last is computed by the source's own functions, as arithmetic, with nothing measured. The relation column for channels 8 to 12 is the quest's, from docs/84.

| k | codon | hexagram | degree | order | x | y | z | relation (quest) |
|---|---|---|---|---|---|---|---|---|
| 0 | AAA | ䷀ | 0 | 0 | 1.0000 | 0.0000 | 0.0000 | offset+1 |
| 8 | AGA | ䷈ | 2 | 2 | 0.0747 | -0.9972 | 2.7200 | is-a |
| 9 | AGC | ䷉ | 3 | -3 | 0.6235 | -0.7818 | 3.0600 | near |
| 10 | AGG | ䷊ | 3 | -2 | 0.9556 | -0.2948 | 3.4000 | unlike |
| 11 | AGT | ䷋ | 3 | -1 | 0.9556 | 0.2948 | 3.7400 | part-of |
| 12 | ATA | ䷌ | 3 | 0 | 0.6235 | 0.7818 | 4.0800 | used-for |
| 21 | CCC | ䷕ | 4 | 1 | 1.0000 | -0.0000 | 7.1400 | |
| 63 | TTT | ䷿ | 7 | 7 | 1.0000 | -0.0000 | 21.4200 | |

Three rules produce the rows. The codon is `k` in base four. The degree is `⌊√k⌋` and the order is `k − (l² + l)`, which is the sphere's `k = l² + l + m` from docs/84. The point is `2πk / 10.5` round the axis and `0.34k` up it. The test `the_helix_turns_every_ten_and_a_half` checks that codon 21 sits directly above codon 0, two full turns up. By the same rule codon 63, `TTT`, is six turns up and also above `AAA`.

The relation column is the facet's. `RelationVocab::name(k)` returns `offset+1` to `offset+8` for the eight offset channels, 0 to 7, which run `AAA` to `ACT`. From channel 8, `AGA`, it returns the marker word that owns the channel, or `ch<k>` if none does. On the persona lexicon, with 24 marker relations, the markers fill channels 8 to 31, and docs/84 names channel 10, `AGG`, as `of`. On the quest's lexicon the same `AGG` is unlike. A codon is a place. The relation in it depends on which lexicon is being read.

The table is also how `read` recovers the relation table. It takes every row from channel 8 upward whose name is a marker, and hands the pairs of name and channel to `RelationVocab::from_entries`. The code table is therefore both the legend and part of the data.

## 7. The two strands

A DNA double helix has two strands. In the helix file, the two strands are a word's two sides on each channel. Column two of a codon line is `theta_byte(k)`, the source byte: the angle the word presents on channel `k` when it is the word doing the relating. Column three is the target byte: the angle the word presents when it is the word related to. Both are bytes, one of 256 quanta of 2π.

The split has its own history. docs/69 found that with one phase per word per channel, "a then b" and "b then a" were the same constraint, and attraction on every adjacent pair drove the frequent words to one angle. On the Rust Book, offset+1 dispersion among the top 100 words was 0.003. The fix gave `SpectralPhasor` a `tgt: [u64; 8]` beside `packed`. `couple_relations` moves the earlier word's source side toward the later word's target side. `on(a, b, k)` is `cos(src_a[k] − tgt_b[k])` and is directed. docs/84 says phiano has kept the sides apart "since docs/72". The split was made in docs/69. docs/72 tested giving rare target sides three times the step, found it did not help, and kept the sides separate.

`write` does not read the target byte directly. It calls `theta_target_raw(k)`, which turns the stored byte into an angle, and `target_byte` divides by 2π, multiplies by 256 and rounds. The angle was a byte times 2π over 256, so the rounding returns the stored byte and the round trip is exact.

The evidence column is the count. `evidence_count(k)` reads 32 bits from `evid[k/2]`. A test in the phasor module sets channel 63 to 4,000,000,000 and reads it back, so a codon line can carry any count the trainer can reach. `read` puts the value back in the same 32 bits.

The analogy has a limit here and it should be said. In DNA the second strand is the complement of the first: `A` pairs with `T` and `C` with `G`, and one strand determines the other. In the helix the two strands are independent. The target byte is learned separately from the source byte and the one says nothing about the other. The helix could print a codon's complement, and it does not.

A worked line from the quest makes the directions concrete. `w0` is-a something means `w0` is the word doing the relating on the is-a channel, `AGA`. So `w0`'s source byte on `AGA` moved toward the target byte of the word it is-a. Evidence counted on that channel. Reading `strands/w0/w0.dna` with `awk -F'\t' '$4>0'` prints that line among the six, with both bytes and the count.

## 8. The strand file

A strand is a word's 64 relations as text. The first line is a header. For a word `w` it reads `# w`, a tab, `amplitude` and the value to six decimals, a tab, `band` and the phasor's `band_n`, a tab, `count` and the word's count. Then come 64 lines, in channel order, `AAA` first and `TTT` last, each with seven tab-separated fields:

```text
codon  src  tgt  evidence  λnm  band  hex
```

The file lives at `strands/<aa>/<word>.dna`, where `<aa>` is `shard_of(word)`: the first two alphanumeric characters, lower-cased, or `_` if there are fewer. `cat` goes to `strands/ca/cat.dna`. The shard keeps any one directory from holding the whole lexicon, which for the persona lexicon is 18,496 files. The file name is `file_name(word)`: ASCII letters, digits and `-` stay, and every other character becomes `%` and its code point in hex. The word itself is in the header, and `read` takes it from there.

The last three columns are PhiTon's. `PhitonSpectrum::phase_to_color(theta(k), band_n)` calls `Phiton::phase_to_wavelength`. That function takes the angle's fraction of a full turn, places it between `VISIBLE_MIN_NM` 380 and `VISIBLE_MAX_NM` 750, adds a fine-structure shift of `Phical::quantum(band_n) × α × 10`, and clamps the result to the visible range. `SpectralBand::from_wavelength` names the band, and `PhitonColor` carries the RGB value that `hex()` prints as `#RRGGBB`. The comment in `phase_to_wavelength` records why the quantum clamp is there: without it the shipped manifold's counts of 1.8 million pushed every word to 750 nm.

docs/85 gives the one worked value: `love`'s `AAA` line reads 418.5 nm, indigo, `#5900F8`, where before it was the byte 26. That is the first time `phase_to_color` was read by anything that ships. docs/85 counts seven dead conversions in `phiton` before that day. The helix is now one of its two consumers, with the sector palette.

What the colour means is modest. It is a monotone relabelling of the source angle, shifted a little by the word's convergence counter. Two words with the same source byte and the same quantum get the same colour on that codon. The colour says nothing the byte did not. It says it in the family's units.

The header's `band` field is the raw counter `band_n`. docs/85 says it should print the quantum beside it, since the quantum is what the colour uses. That is not done. The header also names this field `band` while column six names a spectral band, so the one word means two things in one file.

A short query shows the format working. `awk -F'\t' '$4>0' strands/ca/cat.dna` prints only the codons on which `cat` has evidence. Adding `{print $1, $5, $6}` prints them as codon, wavelength and band, the word's active spectrum.

## 9. The codon directories and the shell

The strands answer "what does this word relate to". The codon directories answer the reverse: "who carries this relation". For each channel `k`, `write` keeps every word with evidence above zero, sorts by evidence descending and then by word, and writes the first 200 to `codons/<CODON>/words.tsv`. The file's first line is `# <CODON> · <glyph> · <relation>`, then a header `word evidence`, then one word per line.

So `cat codons/AGG/words.tsv` on the persona lexicon lists the 200 words with the most evidence on channel 10, the relation `of`. The same command on the quest's helix lists the words with the most unlike evidence. The directory is a ranked index, and it is capped. A word with evidence on `AGG` below the 200th is still in its strand and absent from the list.

For the full set, docs/84 gives the query:

```text
grep -l $'^AGG\t.*\t[1-9]' strands/*/*.dna | wc -l
```

`grep -l` names every strand with a line that starts with `AGG` and has a field starting with a nonzero digit later on the line. `wc -l` counts them. The pattern is loose. The final `[1-9]` can match the target byte, the evidence or the wavelength, so as written it can count a word whose evidence on `AGG` is zero. A tight version tests field four, for example `awk -F'\t' '$1=="AGG" && $4>0 {print FILENAME}'`. The count on the persona lexicon was not reported.

The three queries docs/84 prints are the ones the user asked for:

```text
cat codons/AGG/words.tsv                              # who carries `of`
awk -F'\t' '$4>0' strands/ca/cat.dna                   # a word's active codons
grep -l $'^AGG\t.*\t[1-9]' strands/*/*.dna | wc -l     # every word with `of` evidence
```

`README.md` prints the same three with the codon `GAT` in place of `AGG`. The README's `awk` has no `-F'\t'`, which works because the default splits on any whitespace and no strand field contains a space.

The shell is the whole query language. There is no index server, no binary to call, and no format to learn beyond tabs and three-letter codons. A person can `ls strands/lo` to see words beginning `lo`, `diff` two strands to see where two words differ channel by channel, or `sort -k4 -n` a strand to rank its relations by evidence. The tree is plain files, so version control, `rsync` and a text editor all apply. That was the point of choosing a directory tree over another binary.

The cost is size. At 75 MB for 18,496 words, a strand is a few kilobytes against the phasor's 64 bytes of source, 64 of target and 256 of evidence. The tree is a view to read and query, and the `.chroma` remains the working format.

## 10. Reading back, and the quest read as a gene

`read(dir)` is the other half. It assumes nothing but the tree. It reads `codons.tsv` first and rebuilds the relation table from the named rows, channel 8 upward. Then it lists `strands/`, sorts the shards, lists and sorts each shard's files, and parses each file with `strand(text)`. The parser takes the header's first field as the word and finds `amplitude `, `band ` and `count ` by prefix. For each following line with at least four fields it calls `codon_index` on the first. An unknown codon makes the whole strand return `None`, and that word is skipped. It parses the source, target and evidence as `u64` and ORs each into its packed array at its channel's offset. `SpectralPhasor::from_packed_sides(packed, tgt, evid, amplitude, band_n, count)` builds the phasor and the word goes into `facet.lexicon`.

Each line carries its codon, so line order within a strand does not matter.

The test is in `read.rs`. It learns relations from three toy sentences, initialises every word, trains three passes of `couple_relations` at `LEARNING_RATE`, writes to a temporary directory, reads back, and compares. The lexicon sizes match. For every word, `packed()`, `packed_target()` and `packed_evidence()` are equal, and so is `count`. The relation entries are equal. `codons/AAA/words.tsv` exists. Then it removes the directory.

The quest is where the helix reads as intended. The phiecs oracle has 256 words in families of 8 and five relation kinds. docs/83's `bin/quest_ingest` turns every row into the three-token document `about kind other`, so `RelationVocab::learn` reads each kind word as a marker and gives it a channel with no mapping written anywhere. On the quest's helix those five are channels 8 to 12, and docs/84 reports `w0`'s active codons as exactly these:

| codon | channel | relation |
|---|---|---|
| `AAA` | 0 | offset+1 |
| `AGA` | 8 | is-a |
| `AGC` | 9 | near |
| `AGG` | 10 | unlike |
| `AGT` | 11 | part-of |
| `ATA` | 12 | used-for |

Five relations the oracle gave `w0`, and the adjacency channel offset+1, which the three-token documents also feed. The other 58 codons have no evidence. docs/84 calls this the quest read as a gene, and the phrase is fair in one sense: a person can open one file and see the whole of what the quest taught one word, by name, with no program.

It is also a check. If `w0` had evidence on a sixth marker codon, the ingest would have leaked a relation across channels. It did not. PHIENT_FACTS.md reports the quest's scores elsewhere, attraction-only with dithered rounding at 0.594 MRR against chance 0.021, and wrong channel at 0.015. The helix shows the same separation one word at a time.

## 11. What it depends on and what depends on it

In the PML family, phinet is the parent engine. The helix uses one piece of it directly. `Codon::of` calls `phinet::sphere::degree_of` and `phinet::sphere::order_of`, which give channel `k` its degree `⌊√k⌋` and order `k − (l² + l)`. These are the same indices the sphere tie uses. `phinet::Tie::Sphere` shares one complex weight per degree across 64 = 8² channels, and at 1,192 parameters it reached 12.8× valid and 13.1× test on Austen. The anisotropic tie, one weight per `(l, |m|)`, reached 16.0× and 15.6× at 4,776 parameters. A helix row's degree and order columns are the channel's place in that operator. The helix trains nothing, and it only prints them.

The helix does not call `phinet::hexagram`. Its glyph comes from `hexagram_glyph` in `code.rs`, and its doc comment says it gives the same reading `phinet::hexagram` gives a spectrum's octaves. Section 13 shows that it does not. docs/86 filed the quest's 256 words by `phinet::hexagram::hexagram_of` and found them in 61 of 64 hexagrams. That is a hexagram of a word's function. The helix's hexagram is a label on a channel, a different object.

phiton is the child the helix woke up. docs/85 lists phiton as a module inside phiano with seven dead conversions, and records that the helix now reads it through `PhitonSpectrum::phase_to_color`, `Phiton::phase_to_wavelength`, `SpectralBand::from_wavelength` and `PhitonColor`. The planned move of phiton to its own crate names the helix and the sector palette as its two consumers.

phical enters through `Phical::quantum(band_n)`, the clamp on the convergence counter that sets the fine-structure shift. The geometry crate `phi-cal` does not enter.

phinum is not used. phiqlm is planned, and nothing of it is built, so it is not used. phifno 1.0.0 shares the sphere's indexing and does not call the helix.

Inside phiano the helix depends on `Facet`, its `lexicon` and `relations`, on `SpectralPhasor` and its accessors and constructor, on `RelationVocab`, and in the binary on `Storage::load` and `Storage::save` for the `.chroma`. It is listed in docs/85 as new in phiano 0.2.0.

What depends on the helix today is the binary `bin/helix` and the round-trip test. No other code reads a helix tree. phiecs talks to phiano through TSV, and docs/84 proposes that a quest write strands directly. That is not built. phiace and its UI do not read the helix. The helix sits at the edge of the graph: it reads four parts of the family and nothing reads it. By docs/82's rule that a thing nothing depends on is not in the system, the helix is a view the user reads. It is part of the system through the person at the shell, and through no code.

## 12. The next experiment

docs/84 names two next steps for the helix: the helix as the quest's own output format, with a quest writing strands directly, and placing a word's phasors on the sphere by relation kind in place of channel index. The first is engineering. The second is the first experiment that could give the helix's geometry a meaning, and it is the one proposed here.

Today a relation's channel is set by `RelationVocab::learn`: docs/83 records that is-a is channel 8 because it is the word most often found standing between two others. On the quest, is-a sits at channel 8, degree 2, and near, unlike, part-of and used-for sit at channels 9 to 12, all degree 3. Degree is scale on the sphere, and the anisotropic tie learns one weight per `(l, |m|)`. If channels were assigned by relation kind, for example with the two hierarchical kinds, is-a and part-of, at one degree and the associative kinds, near, used-for and unlike, at another, the sphere tie would share weights within those groups by construction. The helix would then print a degree that says which kind of relation lives there.

The experiment trains the quest with the shipped attraction-only, dithered-rounding trainer and the anisotropic sphere tie under two channel assignments, and scores both with the same MRR, hit@1 and hit@5 against the oracle's 1,280 rows and 293 target words. The treatment assigns channels by kind as above. The control is the current assignment by `RelationVocab::learn`, run with the same seeds, epochs and tie. A second control shuffles the five kinds over the same five channels, to separate an effect of grouping from an effect of which channels are used. The held-out 20% two-hop scores, where is-a reached 0.095 and used-for 0.093, are the numbers to watch, since that is where a shared weight could carry a relation across a hop.

The pass condition is a difference between the treatment and both controls larger than the spread across seeds. If there is none, the helix's degree column stays a label and the paper says so. The quest's byte deadband, in which a nudge of `0.06·sin d` falls under half a quantum once `sin d < 0.2`, limits every quest number, so the comparison must keep dithering on in all arms.

The engineering step can run alongside it. A quest that writes strands directly should be scored after `read`, with the same run saved as a `.chroma` as the control. The expected result is identical scores to the last digit, and anything else is a bug in the round trip. It also forces the amplitude question in section 13 into a test.

Neither run has been done. Both use code that exists, with no new model.

## 13. What is wrong or missing

The hexagram column is wrong against its own comment. `hexagram_glyph(k)` returns `U+4DC0 + k`. The Unicode hexagram block follows the King Wen sequence, so channel 0 gets ䷀, the Creative, all lines unbroken, and channel 63 gets ䷿, Before Completion, whose lines alternate. If the glyph drew `k`'s six bits, channel 0 and channel 63 would be opposites with all six lines the same, and ䷿ is neither. The comment says the glyph gives `k`'s six bits as lines, bottom first, the same reading as `phinet::hexagram`. It gives the King Wen hexagram numbered `k + 1`. The test asserts ䷀ and ䷿, so it fixes the behaviour without checking the claim. Either the comment or the function must change.

The amplitude does not round-trip exactly. The header prints it with `{:.6}`, and `read` parses that string. The round-trip test compares source, target, evidence and count, and neither amplitude nor `band_n`. So "byte-exact" is true of every byte, every count and the relation table, and the amplitude is correct to six decimals. Whether any score moves with that rounding is not measured. The fix is to print it with the shortest round-tripping format and add it to the test.

The reader drops some marker names. `read` skips any row whose relation begins with `ch` or `offset`, to exclude the placeholder `ch<k>` and the offset names. A marker word that begins with `ch`, such as a word like `child`, would be written to `codons.tsv` and silently lost on read, and its channel would come back unnamed. Matching `ch` followed only by digits would close it.

`file_name` escapes a character as `%` and `{:02x}` of its code point, which is two hex digits only below 256. A character above that gives three or more, so two different words can share a file name, for example one character at U+2014 and a space followed by `14`. One strand would overwrite the other. The persona lexicon's words were not checked for this.

The header field `band` is the raw counter. docs/85 asks that the quantum be printed beside it, and that is not done. The same word `band` names a spectral band in column six.

The two strands are independent where DNA's are complementary.

The helix position means nothing yet. It is correct B-DNA geometry applied to a channel index. The degree and order are correct sphere indices applied to the same index. No experiment connects either to language. docs/74's measurement of hexagrams as predictors, 0.9% against chance 1.6%, is a reason for care.

The two documents disagree on the write time, 0.6 s against 1.9 s, and docs/84 attributes the two sides to docs/72 when they were introduced in docs/69.

## 14. How it was found

The helix came at the end of a line of work on what a word is in Phiano, and every step is in a numbered doc.

docs/64 promoted a lexicon and reported offset-channel dispersion of 0.85 to 0.93, calling the channels healthy. docs/69 found that by frequency tier they were not: on the Rust Book the top 100 words had offset+1 dispersion 0.003. The cause was one phase per word per channel. The fix gave every `SpectralPhasor` a target side, `tgt: [u64; 8]`, beside its source side, and moved the file format to v5. That is the origin of the helix's two strands. docs/72 tested boosting rare target sides, found that more steps on a noisy side made it noisier, and kept the sides apart with the boost set to 1.0.

docs/74 brought in the I Ching as `phinet::hexagram`: six lines gating six octaves of a 64-sample spectrum. It measured whether the operator's predicted hexagram matched the next word's and refuted it, 0.9% against chance 1.6%. The hexagram stayed as an operation and a label. docs/83 built the bridge from a phiecs quest back to a facet, `bin/quest_ingest`, where each relation kind became a marker channel because the kind word stands between the pair, and found the byte deadband that limits every quest score.

docs/84 is where the helix was built. Its first half is the sphere. Reading NVIDIA's spherical operator showed that the 64 channels are also 64 real harmonics of degree below 8, which gave `phinet::sphere`, `Tie::Sphere` and phifno. The note's opening counts four sixty-fours: channels, harmonics, hexagrams and codons. Its second half is the helix. It writes the tree layout with a four-field strand line, reports the persona lexicon at 0.6 s and 75 MB, and reads the quest's `w0` as a gene.

docs/85 took an inventory of every `phi*` crate and set out PML: phinet the parent, phical, phinum, phiqlm, phifno and phiton its children. Under the heading "Do not recreate f64" it recorded that the helix's strand line now carries each codon's `PhitonColor`, with `love`'s `AAA` at 418.5 nm, indigo, `#5900F8`. It named the helix as the first shipped reader of `phase_to_color`, and asked for the quantum beside the header's `band`. It lists the helix as new in phiano 0.2.0.

docs/86 filed the quest's words by hexagram, 61 of 64 occupied, and found the table flat. PHIENT_FACTS.md then gathered the helix into one line, with the write time at 1.9 s.

The order of discovery matters for reading the helix. The two strands came from a measured defect. The 64-fold count came from the sphere. The colour came from an instruction about units. The DNA picture came last and tied them together as a file.

## 15. What remains

Three defects in the code need fixing before the helix is relied on. The hexagram glyph should either draw `k`'s six bits, as its comment says, or the comment should say it is the King Wen hexagram numbered `k + 1`. The amplitude should be printed so that it parses back to the same `f64`, and the round-trip test should compare amplitude and `band_n`. The reader's filter should match the placeholder names exactly, so that a marker beginning with `ch` survives. Each is a few lines and a test.

Three smaller items follow. `file_name` should escape with a fixed width, or a terminator, so that no two words share a file. docs/84's `grep` query should test the evidence field directly. The strand header should print `Phical::quantum(band_n)` beside the counter, as docs/85 asks, and one of the two fields called `band` should be renamed.

The experiment in section 12 remains: quest channels assigned by relation kind against the ingest order and a shuffle, under the anisotropic sphere tie, with dithering on. Until it runs, the degree and order columns are indices. The helix's xyz column has no experiment proposed for it at all, and none is obvious. It may stay a picture.

The quest writing strands directly remains, with the `.chroma` save as its control and identical scores as its pass. So does any use of a strand as editable input: a person changes a line and reads the effect. Nobody has tried that.

The helix at larger scale remains untested. The persona lexicon is 18,496 words and 75 MB. The size per word is set by 64 lines of seven fields, so a lexicon ten times larger is a tree ten times larger. The write time was measured once for each version of the format, and query times were not measured.

The helix does not touch the Phi-4 run, which has not happened. PHIENT_FACTS.md records that the quest has run on the synthetic oracle only. When a real teacher's relations come in, the helix is the obvious place to read them one word at a time, and whether they look like `w0`'s six clean codons is the first thing to look at.

The claim that stays, in plain terms: a Phiano lexicon can be written as text a person reads with `cat`, where every relation has a three-letter name, a channel, a degree, a hexagram, a point and a colour. It comes back with every byte and every count. What those names mean beyond the channel index is the open question, and the helix is where it can be seen.

## 16. Sources

These are the local files read for this paper. Every number above is from one of them or is arithmetic on a constant or function defined in them, marked as such in section 6.

The eight files named for this paper are PHIENT_FACTS.md, docs/84, the four files of `src/helix`, the helix binary and phiton's `mod.rs`, read with the brief. The rest were opened to confirm a function's name or behaviour, or a number that a named file cites: the phasor accessors, the relation names, the quantum clamp, the band names, the sphere indices, and the docs that the history in section 14 cites by number. docs/64 is cited only through docs/69's account of it. No source file was modified.

| path | what it gave |
|---|---|
| `/home/claude/phiano/docs/papers/phient/BRIEF.md` | the form of the paper |
| `/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md` | the paradigm paragraph; the family; the helix line (18,496 words, 1.9 s, 75 MB, byte-exact); sphere, quest and deadband numbers |
| `/home/claude/phiano/docs/84_sphere_and_helix.md` | section 2, the helix: tree layout, codon rules, 0.6 s and 75 MB, shell queries, `w0` as a gene, what the helix is not; section 1 for the sphere numbers; section 3 for what remains |
| `/home/claude/phiano/src/helix/mod.rs` | the module doc, the tree layout with seven-field strands, the exports |
| `/home/claude/phiano/src/helix/code.rs` | `BASES`, `CODONS`, `PITCH_PER_TURN`, `RISE`, `RADIUS`, `codon`, `codon_index`, `hexagram_glyph`, `helix_position`, `Codon`, tests |
| `/home/claude/phiano/src/helix/write.rs` | `write`, `CODON_WORDS`, `target_byte`, `shard_of`, `file_name`, `readme` |
| `/home/claude/phiano/src/helix/read.rs` | `read`, `strand`, the round-trip test |
| `/home/claude/phiano/src/bin/helix/main.rs` | the `write` and `read` commands |
| `/home/claude/phiano/src/phiton/mod.rs` | `Phiton::phase_to_wavelength`, the visible range, the quantum clamp |
| `/home/claude/phiano/src/phiton/spectrum.rs` | `PhitonSpectrum::phase_to_color` |
| `/home/claude/phiano/src/phiton/types.rs` | `PhitonColor::name`, `PhitonColor::hex` |
| `/home/claude/phiano/src/relational/relation_vocab.rs` | `RelationVocab::name`, `markers` |
| `/home/claude/phiano/src/phical/phical.rs` | `Phical::quantum` |
| `/home/claude/phiano/src/phasor/spectral_phasor.rs` | `theta_target_raw`, `evidence_count` |
| `/home/claude/phiano/src/phasor/spectral_phasor_nudge.rs` | `theta_byte` |
| `/home/claude/phiano/src/phasor/spectral_phasor_set_evidence_count.rs` | `theta` |
| `/home/claude/phiano/src/phasor/tests_width_tests.rs` | the 4,000,000,000 evidence test |
| `/home/claude/phiano/src/facet/facet_relational_fit.rs` | `Facet::relation_name` |
| `/home/claude/phinet/src/sphere/basis.rs` | `degree_of`, `order_of` |
| `/home/claude/phiano/docs/85_PML_STRUCTURE.md` | PML structure; the helix as phiton's reader; `love`'s `AAA` |
| `/home/claude/phiano/docs/69_two_sides.md` | the source and target split, dispersion 0.003 |
| `/home/claude/phiano/docs/72_two_more_negatives.md` | the rare target boost, kept off |
| `/home/claude/phiano/docs/74_pair_key_lovelace_hexagrams_austen.md` | `phinet::hexagram`, prediction 0.9% against chance 1.6% |
| `/home/claude/phiano/docs/83_quest_ingest.md` (opening) | `bin/quest_ingest`, how kinds become marker channels |
| `/home/claude/phiano/src/phiton/bands.rs` | the sixteen band names |
| `/home/claude/phiano/docs/86_liskov_three_operators.md` | the quest filed by hexagram, 61 of 64 |
