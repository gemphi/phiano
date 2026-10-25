# PhiTon: light as the unit system
*Phient papers, 14 of 16 · 2026-07-13 · PhiTon: light as the unit system — a phase as a wavelength, a band and a colour, and where a bare f64 should give way to it*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

PhiTon is the family's unit layer for light. It lives in `phiano/src/phiton`, a module of `mod.rs` and four files, and it does one thing that matters. It reads an angle on the relation circle as a wavelength of visible light, names the spectral band that wavelength falls in, and gives it an RGB colour. The types are `Phiton`, `LightQuantum`, `PhitonColor`, `SpectralBand`, `PhitonSpectrum` and `Chiton`. The path that carries the reading is `Phiton::phase_to_wavelength`, then `SpectralBand::from_wavelength`, then `PhitonColor::new`, joined in `PhitonSpectrum::phase_to_color`. The module has no tests of its own.

The paper asks two questions. What exactly does PhiTon do when it turns a phase into light? And where in the family should a bare `f64` give way to PhiTon's units? The second is the user's instruction as docs/85 records it, under the heading "Do not recreate f64". docs/85 answers it three ways. The helix's strand line now prints each codon's wavelength, band and hex colour beside its bytes, and that is done. phifno's spectrum by degree could be named in PhiTon's bands, and that is not done. The helix header should print the quantum beside the raw counter, and that is not done. docs/85 also says where the unit does not belong: inside the tape.

What was measured is small. docs/85 gives one worked reading, `love`'s codon `AAA` at 418.5 nm, indigo, `#5900F8`, where before it was the byte 26. docs/70 records the fix that made such readings sane. The convergence counter `band_n` reaches 1,845,381 on the shipped manifold, and before `Phical::quantum` clamped it to 13, `phase_to_wavelength` pushed every such word to 750 nm. docs/82 counts seven dead items in `phiton`. PHIENT_FACTS.md lists the helix at 18,496 words in 1.9 s and 75 MB, with a PhiTon wavelength, band and hex per codon. No language score passes through PhiTon. Nothing about light has been measured as a predictor of anything.

Reading the code turned up defects, and the paper reports them. `SpectralBand::from_wavelength(750.0)` returns violet. The sixteen bands are called proportional and are not: fourteen are 25.6 or 25.7 nm wide, red is 10.3 and crimson is 1.0. `band_colors(64)` returns violet twice, yellow twice, and never red or crimson. `spectral_distance` says it accounts for wrap-around and does not. The code holds two fine-structure shifts that differ by a factor of about six. The oscillator names a band by index and disagrees with the wavelength reading on 178 of 256 byte angles.

The conclusion is narrow. PhiTon is a good unit for reading and a poor one for arithmetic. Where the engine reports an angle to a person, it should report nanometres and a band name. Where the engine computes, the circle is the unit, and a wavelength cuts it open.

## 2. The claim

The claim written in the code is large. The module comment in `mod.rs` says light "is governed by quantum oscillations of the fine-structure constant α ≈ 1/137", and that Phiton and Chiton together "form a 2-in-1 architecture for light's wave-particle duality, providing the physical basis for linguistic word mapping and intelligence". `chiton.rs` says the two counter-rotating aspects create standing waves "that make linguistic word mapping and intelligence plausible". `gemgum.rs`, the binding layer that calls PhiTon, says "words follow the physics of light".

This paper does not carry that claim. None of it was measured. No score in PHIENT_FACTS.md passes through a wavelength. The paper quotes the claim so the reader knows it is in the source, and sets it aside.

The claim this paper makes is the one docs/85 makes, and it is about units. A channel of a Phiano word holds an angle, stored as a byte, one of 256 quanta of 2π. A byte is a bare number and says nothing to a person reading a file. PhiTon gives the same angle three readings that do say something: a wavelength in nanometres between 380 and 750, one of sixteen named bands, and a colour in `#RRGGBB`. Inside one quantum the map is monotone, so two angles in one order give two wavelengths in the same order. The reading loses nothing a person needs and adds a name.

The second part of the claim is where the unit belongs. docs/85 takes the user's instruction literally: "where the engine reports a bare number that the family already has a unit for, use the unit." Its rule for what not to do is as plain: "replace `f64` inside the tape. Autograd is arithmetic on complex numbers; a unit type there would be a cost with no reading." This paper adds one reason from the code. The circle closes and the spectrum does not. Byte 0 and byte 255 are neighbours on the circle. Their wavelengths at quantum 0 are 380.0 nm and 748.6 nm, by arithmetic on `phase_to_wavelength`. Arithmetic on wavelengths treats those neighbours as the two ends of the spectrum.

The third part is placement. docs/85 names phiton a child of phinet, one of the things that "reads the 64 channels its own way and needs no words". It proposes lifting `phiano/src/phiton` into its own crate that depends only on phinet's `c64` and phical's `quantum`. That move is not done.

So the claim comes to three sentences. A phase can be printed as light. It should be printed as light wherever a person reads it. It should stay a phase wherever the engine computes with it.

## 3. The mechanism

`mod.rs` declares the unit struct `Phiton` and its constants: `C` is 299,792,458 m/s, `H` is 6.626 070 15e-34 J·s, `VISIBLE_MIN_NM` is 380.0 and `VISIBLE_MAX_NM` is 750.0. `phase_to_wavelength(phase, band_n)` takes `phase.rem_euclid(TWO_PI) / TWO_PI`, a fraction of a turn. It places that fraction linearly between 380 and 750, adds a shift of `Phical::quantum(band_n) × ALPHA × 10` nanometres, and clamps the sum to the visible range. `ALPHA` is `1.0 / 137.0` in `config`. `wavelength_to_phase` clamps and inverts the linear part only. `frequency_thz` is ν = c/λ in terahertz. `energy_ev` is E = hc/λ in electronvolts, with the elementary charge written into the function. `sector_color(sector, sector_count, band_n)` turns a sector index into a phase and then into a `PhitonColor`.

`types.rs` holds two structs. `PhitonColor` has `wavelength_nm`, `band` and `rgb`. `PhitonColor::new` computes `rgb` with the private `wavelength_to_rgb`, a piecewise ramp over six intervals from 380 to 750 nm, times an intensity factor that falls off below 420 nm and above 700 nm. `name()` returns the band's name and `hex()` formats the triple. `LightQuantum` has `phase`, `amplitude`, `band_n` and `wavelength_nm`. `from_phase` fills the wavelength. It offers `frequency_thz`, `energy_ev`, `to_color` and `band_position`.

`bands.rs` holds `SpectralBand`, with `min_nm`, `max_nm` and a `&'static str` name, and the constant table `BANDS` of sixteen: violet, indigo, blue, azure, cyan, teal, emerald, green, lime, yellow, gold, amber, orange, scarlet, red, crimson. `from_wavelength` clamps and returns the first band with `min_nm ≤ λ < max_nm`, falling back to `BANDS[0]`. `index_of` does the same and returns the index. `at_index` takes an index modulo 16. `center_nm`, `is_warm`, `is_cool` and `is_green` are tests on the band edges.

`spectrum.rs` holds `PhitonSpectrum`: `sector_to_color`, `phase_to_color`, `band_colors`, `spectral_distance`, `coupling` and `antipodal_color`. `phase_to_color` is the function the helix calls.

`chiton.rs` holds `Chiton`, with `angular_frequency`, `period`, `spin_clockwise` and `wavelength_nm`. `from_quantum` sets ω to 2π times the frequency in THz, T to its inverse, and `spin_clockwise` to false. `resonance` multiplies a spin factor, a wavelength factor `1 / (1 + φ·|Δλ|/100)` and a frequency factor `1 / (1 + |Δω|·α)`. `standing_wave_amplitude` is 2·A·cos(kx)·cos(ωt). `to_quantum` goes back through `wavelength_to_phase`.

The shift comes from phical. `Phical::quantum(band_n)` in `src/phical/phical.rs` is `band_n.min(BAND_N_EFFECTIVE_MAX)`. `BAND_N_EFFECTIVE_MAX` is 13, which the config comment derives as 2π/64 ÷ α = 13.45, "one sector" of fine-structure shift. Every PhiTon wavelength passes through that clamp. PhiTon imports nothing from the phinet crate today.

## 4. What was measured and the numbers

Every number in this table is quoted from a source. None is a result about language.

| what | number | source |
|---|---|---|
| `love`, codon `AAA`, stored byte | 26 | docs/85 |
| `love`, codon `AAA`, PhiTon reading | 418.5 nm, indigo, `#5900F8` | docs/85 |
| `band_n` on the shipped manifold | 1,845,381 | docs/70 |
| `band_n`, as the code comments put it | 1.8 million | `mod.rs`, `phical.rs` |
| quantum clamp | 13 | `BAND_N_EFFECTIVE_MAX`, docs/70 |
| dead items in `phiton` | 7 | docs/82 |
| helix: words, write time, size | 18,496, 1.9 s, 75 MB | PHIENT_FACTS.md |
| helix write time before the PhiTon columns | 0.6 s | docs/84 |
| tests in `src/phiton` | 0 | the source files |

The reading of `love` is the only PhiTon value any doc reports for a real word, and it checks against the code. Byte 26 is 26/256 of a turn. The linear part gives 417.578 nm. The shift at quantum 13 is 13 × (1/137) × 10 = 0.949 nm. The sum is 418.527, printed to one decimal as 418.5. Indigo runs from 405.6 to 431.2. `wavelength_to_rgb` at that wavelength gives red 0.358, green 0 and blue 1.0, times a factor of 0.974, which rounds to 89, 0 and 248, or `#5900F8`. At quantum 11 the line would print 418.4, and at quantum 0 it would print 417.6 and `#5B00F4`. So docs/85's value is consistent with a quantum of 12 or 13. `love`'s counter was not reported. All of this is arithmetic on the source's constants. It confirms the print. It is not a finding.

The counter figures agree up to rounding. docs/70 gives the exact 1,845,381. The comments round it to 1.8 million. The test `counters_do_not_walk_the_wheel` in `src/phical/tests.rs` builds `ColorSpaceTimePoint::from_phase` at 13 and at 1,800,000 and asserts the same band number, the same effective phase, the same oscillation energy, and zero colour-space-time distance. It is the only test in the tree that exercises a PhiTon formula, and it does so through phical.

The helix figures come from two documents. docs/84 gives 0.6 s for a four-field strand line. PHIENT_FACTS.md gives 1.9 s for the seven-field line that carries PhiTon's wavelength, band and hex. Paper 7 of this series took the later figure. The two versions were not timed side by side, so the cost of the PhiTon columns is not measured.

Not measured: any score through a wavelength, a band or a colour; any timing of `phiton` alone; any test of whether a person reads a strand faster in nanometres than in bytes.

## 5. The one idea restated

A Phiano word stores an angle on each channel. PhiTon says the angle can be read as a colour. That is the whole idea.

The idea sits inside the paradigm with nothing added. Phical stores no point, so a word has no coordinates of its own. It has 64 relations, and each relation is an angle on a closed circle with a count. PhiTon does not change what is stored. It relabels each angle. A strand of 64 codons, printed by the helix, becomes 64 spectral lines, and the codons on which a word has evidence become its active spectrum. `awk -F'\t' '$4>0 {print $1, $5, $6}' strands/ca/cat.dna` prints `cat`'s active codons as codon, wavelength and band.

The relabelling adds no information. Two words with the same byte and the same quantum get the same wavelength, the same band and the same colour. A person gains a name. The engine gains nothing, and the paper does not claim it does.

The map has one cost, and it is geometric. The circle is closed and the visible spectrum is a segment. `phase_to_wavelength` cuts the circle at angle 0 and lays it out from 380 to 750 nm. Byte 0 sits at the violet end. Byte 255 sits at the red end, one quantum away on the circle and 368.6 nm away on the spectrum. The oscillator module keeps a closed reading beside it: `hue(t)` returns degrees in [0, 360), which wrap. PhiTon chose the spectrum over the hue wheel. The spectrum gives physical units. The hue wheel keeps the topology. Nothing in the code chooses between them on evidence.

The fine-structure shift is the only thing PhiTon reads besides the angle. It reads the quantum, which is the convergence counter clamped to 13, and moves the wavelength up by at most 0.949 nm. One byte of angle is 370/256 = 1.445 nm of spectrum. So the shift moves a printed wavelength by at most about two thirds of a byte. It changes the band only when a byte sits just below a band edge.

The unit, finally, lives in a field name. `PhitonColor::wavelength_nm` and `LightQuantum::wavelength_nm` are `f64`. There is no `Nanometres` type, no `Terahertz` type, no `Electronvolts` type. PhiTon is a set of named conversions over `f64`. In practice, then, "give way to PhiTon" means this: print the wavelength, print the band name, print the colour, and keep the arithmetic on the angle. A type that carries the unit through the compiler is not built.

## 6. From phase to wavelength, worked

The table runs six bytes of one channel through `phase_to_wavelength` at quantum 0 and at quantum 13, then through `SpectralBand::from_wavelength`, `wavelength_to_rgb`, `frequency_thz` and `energy_ev`. Every value is arithmetic on the source's constants. None is measured.

| byte | quantum | λ nm | band | hex | ν THz | E eV |
|---|---|---|---|---|---|---|
| 0 | 0 | 380.000 | violet | `#4C004C` | 788.9 | 3.263 |
| 0 | 13 | 380.949 | violet | `#4F0051` | 787.0 | 3.255 |
| 26 | 0 | 417.578 | indigo | `#5B00F4` | 717.9 | 2.969 |
| 26 | 13 | 418.527 | indigo | `#5900F8` | 716.3 | 2.962 |
| 64 | 0 | 472.500 | azure | `#00A6FF` | 634.5 | 2.624 |
| 128 | 0 | 565.000 | green | `#C8FF00` | 530.6 | 2.194 |
| 192 | 0 | 657.500 | gold | `#FF0000` | 456.0 | 1.886 |
| 255 | 0 | 748.555 | red | `#520000` | 400.5 | 1.656 |
| 255 | 13 | 749.504 | crimson | `#4E0000` | 400.0 | 1.654 |

The rows show three things. The linear part is exact: byte `b` gives 380 + 370·b/256, so a quarter turn is 472.5 nm and a half turn 565.0. The shift is uniform, 0.949 nm at quantum 13, and changes the band only for byte 255, which crosses 749.0 from red into crimson. Frequency and energy run the other way, since ν = c/λ.

The helix calls `phase_to_color(p.theta(k), p.band_n)` for each of 64 codons and prints `{:.1}` of the wavelength, then `name()`, then `hex()`. So `love`'s `AAA` line ends in `418.5`, `indigo`, `#5900F8`, which is what docs/85 quotes. The target byte gets no colour. Only the source angle is read as light.

The frequency and energy are PhiTon's other units, and nothing in the helix prints them. phical's `oscillation_energy` computes `(n + 1) × α × H × ν`, while its doc comment writes `E = n·α·h·ν`. No shipped output prints an energy. The eV column above is a unit with no reader.

A person can read a word's spectrum from the shell today. `sort -t$'\t' -k5 -n strands/lo/love.dna` orders `love`'s codons by wavelength. `cut -f6 strands/lo/love.dna | sort | uniq -c` counts its codons per band. Neither was run for this paper.

Going back is less exact. `wavelength_to_phase(418.527)` returns the angle of byte 26 plus the shift. `Chiton::to_quantum` then calls `from_phase`, which adds the shift again, and lands at 419.476 nm. Each cycle adds one shift. `to_quantum` is one of docs/82's dead items, so nothing has hit it yet.

## 7. Sixteen bands and a colour

`SpectralBand::BANDS` is the table a band name comes from. Its doc comment says the sixteen bands are "distributed proportionally across [380, 750] nm". Sixteen equal bands over 370 nm would each be 23.125 nm wide. The table is built differently. The first fourteen bands are 25.6 or 25.7 nm wide and run from 380.0 to 738.7. The last two share what is left: red runs from 738.7 to 749.0, 10.3 nm, and crimson from 749.0 to 750.0, 1.0 nm.

Counting bytes makes the effect concrete. At quantum 0, each of the first fourteen bands receives 17 or 18 of the 256 byte angles. Red receives 7. Crimson receives none. At quantum 13, crimson receives one byte, 255. Crimson is reachable from a stored byte only by byte 255, and only when the quantum is 7 or more, since the shift must lift 748.555 past 749.0. Of sixteen named bands, one is almost never printed.

The upper edge has a harder defect. `from_wavelength` clamps to [380, 750] and looks for a band with `min_nm ≤ λ < max_nm`. Crimson's `max_nm` is 750.0, so a wavelength of exactly 750.0 matches no band, and `unwrap_or(BANDS[0])` returns violet. `phase_to_wavelength` clamps to 750.0, so every input past the top lands on exactly that value. From a byte this cannot happen, since byte 255 at quantum 13 reaches 749.504. From a continuous phase it can. At quantum 13, any angle past 0.99744 of a turn is clamped to 750 and named violet. `index_of` returns 0 for the same input. The reddest light is named for the bluest band.

The colours carry their own mismatch. `wavelength_to_rgb` is the usual piecewise ramp, and its hues sit where a person expects them: pure red from 645 nm, yellow near 580. The band names do not follow it. The band called yellow, 610.6 to 636.2 nm, renders from `#FF8400` to `#FF2A00`, orange to red-orange. The band called gold renders `#FF0000` at its middle. Lime renders `#FFDF00`, a yellow. Green at 565 nm renders `#C8FF00`, yellow-green. Teal at 518.75 nm renders `#20FF00`, a green. The names follow a paint box and the colour function follows the spectrum. So a strand line can read `yellow` and `#FF2A00` side by side.

The warm, cool and green tests agree with the table's edges and are correct.

For a reader the lesson is modest. The band name is a coarse label on the angle, sixteen bins of unequal width. The hex is a better colour than the name. The wavelength is the unit, and it is exact.

## 8. The fine-structure shift, twice

A fine-structure shift appears twice in the code, in two units, with two sizes.

The first is PhiTon's. `phase_to_wavelength` adds `quantum × ALPHA × 10` nanometres after the linear map. At the clamp of 13 that is 0.949 nm. The factor 10 has no comment.

The second is phical's. `Phical::effective_phase(phase, band_n)` adds `quantum × ALPHA` radians to the angle before any map. At 13 that is 0.0949 rad. Read through the linear map, 0.0949 rad is 0.0949 / 2π × 370 = 5.588 nm. The config comment on `BAND_N_EFFECTIVE_MAX` says the clamp keeps this shift "inside one sector", and 2π/64 ÷ α = 13.45 is exactly that: thirteen quanta of α fit in one sector of 64.

The two shifts differ by a factor of 370 / (2π × 10), about 5.9. Which one a word gets depends on the caller. The helix calls `phase_to_color(theta(k), band_n)`, which takes the raw angle and the PhiTon shift. `Gemgum::phasor_to_color` calls `Phical::effective_phase(phasor.phase, band_n)` and then `phase_to_color(eff, 0)`, which takes the phical shift and no PhiTon shift. `ColorSpaceTimePoint::from_phase` does the same. `LightQuantum::from_phase` takes the raw angle and the PhiTon shift. So one angle at quantum 13 has two colours in the codebase. For byte 26, the helix path gives 418.527 nm, `#5900F8`. The gemgum path gives 423.166 nm, `#4800FF`. Both say indigo. Both are arithmetic on the same constants.

A third reading ignores wavelength altogether. `Oscillator::color(t)` in `src/oscillator/mod.rs` takes the angle's fraction of a turn, multiplies by 16, floors, and calls `SpectralBand::at_index`. That treats the sixteen bands as equal slices of the circle, which the table in section 7 says they are not. Across all 256 byte angles at quantum 0, the index path and the wavelength path name the same band for 78 bytes and different bands for 178. At half a turn, byte 128, the wavelength path says green and the index path says lime. At a quarter turn it is azure against cyan. At three quarters it is gold against orange.

The history explains why the shift is clamped at all. docs/70 found that the trainer increments `band_n` on every converged channel-0 step, so on the shipped manifold it reaches 1,845,381. `SpectralPhasor::effective_phase` already clamped it to 13. Phical and PhiTon did not, and `phase_to_wavelength` pushed every such word to 750 nm. After section 7, that means every converged word was also named violet. `Phical::quantum` now sits under every Phical and PhiTon formula, and the comment in `phase_to_wavelength` records the reason.

So the one number PhiTon adds to the angle is a counter's clamp, in one of two sizes, depending on the path. For a strand, the size is 0.949 nm, under one byte.

## 9. Chiton, quanta and distances

`Chiton`, `coupling`, `spectral_distance`, `antipodal_color` and `band_position` are the parts of PhiTon that compute. Each has a gap between its comment and its code.

`Chiton` is called the wave with "opposite spin" to the particle. `from_quantum` sets `spin_clockwise` to false for every chiton it makes, and `LightQuantum` has no spin field. `resonance` gives a spin factor of 1.0 when two chitons' spins differ and 0.5 when they match. Every chiton built by `from_quantum` has the same spin, so every resonance in the code carries the 0.5. At equal wavelengths it is 0.5 exactly. At one byte apart, 1.445 nm, it is 0.439. At 10 nm it is 0.244, at 100 nm 0.026. The opposite spins that the module comment says create standing waves are never constructed. `standing_wave_amplitude` and `wave_number` are dead, per docs/82.

`PhitonSpectrum::coupling(a, b)` is `α / (1 + φ·|Δλ|/100)`. It runs from 0.0073 at equal wavelengths down to 0.0010 at the two ends of the spectrum. `Gemgum::word_coupling` multiplies phical's interference by `resonance` and by `1 + coupling`. The coupling therefore moves a word pair's product by less than one percent. Whether it moves any ranking was not measured.

`spectral_distance(phase_a, band_a, phase_b, band_b)` says it "accounts for circular wrap-around". It returns `|λa − λb|`. Byte 0 and byte 255, neighbours on the circle, are 368.6 nm apart at quantum 0. Two bytes half a turn apart are 185 nm apart. So the distance ranks a neighbour across the cut as farther than the antipode. `color_space_time_distance` in phical and `Gemgum::semantic_distance` combine the same wavelength difference with a frequency difference, weighted by φ, and inherit the same cut.

`antipodal_color(phase, band_n)` adds π and reads the colour. Its comment gives "red ↔ green, blue ↔ orange" as examples. On the band table, byte 0, violet, maps to green. Byte 26, indigo, maps to lime. Byte 240, scarlet, maps to emerald. The half-turn is exact on the circle. On the spectrum it is 185 nm. The named pairs are loose.

`LightQuantum::band_position` computes the wavelength's position inside its band, from 0 to 1, multiplies by 1/φ, and takes the remainder modulo 1. A value in [0, 1) times 0.618 is already in [0, 0.618), so the remainder does nothing. The comment promises [0, 1) and a golden-ratio distribution. The function returns a linear position scaled by 0.618. `Gemgum::band_position` repeats it.

`PhitonColor` and `SpectralBand` derive `Serialize` and not `Deserialize`, since a band's name is a `&'static str`. A colour can be written and not read back.

No doc reports a number from any of these functions. Every defect here comes from doing arithmetic on wavelengths.

## 10. Where a bare f64 should give way

docs/85's rule is short: where the engine reports a bare number the family has a unit for, use the unit. This section takes each place the sources name or the code shows, and says whether PhiTon belongs there and whether it is done.

The helix strand line is done. Each of 64 codons prints its source angle's wavelength, band and hex beside its bytes. docs/85 calls this the first time `phase_to_color` was read by anything that ships. Its earlier callers, `Gemgum::phasor_to_color` and `Gemgum::spectral_identity`, are reached from `Facet::word_color` and `Facet::spectral_identity`, both marked `#[allow(dead_code)]`. The seven items docs/82 lists as dead are a different seven: `center_nm`, `standing_wave_amplitude`, `to_quantum`, `wave_number`, `antipodal_color`, `band_colors` and `spectral_distance`. The helix calls none of them. So docs/85's table phrase "7 dead conversions before today; now read by the helix" is true of the module and false of the seven. They are still dead.

The helix header is not done. It prints `band` and the raw counter `band_n`, which can read 1,845,381. docs/85 asks for `Phical::quantum(band_n)` beside it, since the quantum is what the colour uses. The same file also uses `band` for the spectral band in column six. One word names two things.

phifno's spectrum by degree is not done. docs/85 describes `Word::energy()` as a rotation-invariant spectrum by degree and suggests naming it in PhiTon's bands, 16 bands for 8 degrees. It declines, in its own words: "it would be a mapping without a measurement." This paper agrees. A degree is a scale on the sphere, and a wavelength is a relabelled angle. A band per degree would put a colour on a quantity that is not an angle.

The sector palette is done, with defects. `SectorPalette::color` delegates to `Gemgum::sector_color_name`, which calls `Phiton::sector_color` at quantum 0. It is read in `compose`, `persona`, `server` and `command/persona/chat.rs`. With `SECTOR_RESOLUTION` at 64, the palette prints fifteen of the sixteen names. Red appears once, and crimson never. `band_colors(64)` returns violet twice and yellow twice.

The oscillator should give way. `Oscillator::color` names a band by index, and in 178 of 256 cases it names a different band from the wavelength reading. Calling `PhitonSpectrum::phase_to_color` would make the family print one name per angle.

The distances should not give way. `spectral_distance`, `color_space_time_distance` and `semantic_distance` do arithmetic on wavelengths and inherit the cut at angle 0. The shipped relation score, `on(a, b, k) = cos(src_a[k] − tgt_b[k])`, lives on the circle and has no cut. Inside the tape, docs/85 already says no.

The types are not built. A `Nanometres` newtype, a band that deserialises, one shift in one unit: none exists. The rule "give way" is today a printing rule.

## 11. What it depends on and what depends on it

In the PML family, phinet is the parent engine: `Tensor [b,w,s]` complex, `Tape` autograd, unitary FFT, `Fno`, joint training, HRR, 64 hexagram gates and the sphere. docs/85 places phiton as its child and gives the rule that makes it one: a child of phinet "reads the 64 channels its own way and needs no words". PhiTon reads a channel's angle as light. It needs no `Facet`. Today, though, it imports nothing from the phinet crate. It uses `config::{ALPHA, PHI, TWO_PI}` from phiano and `Phical::quantum` from phiano's phical module. The planned crate would depend on phinet's `c64` and phical's `quantum`. That lift is move 1 of docs/85's five, ranked first because it risks nothing. It is not done.

phical is the sibling PhiTon leans on hardest, and there are two of it. The module `phiano/src/phical` supplies `quantum`, the clamp under every wavelength, and it calls PhiTon in return: `phase_to_frequency_thz`, `color_space_time_distance`, `oscillation_energy`, `ColorSpaceTimePoint::band` and `color`, and, in `operators.rs` and `topology`, `LightQuantum`, `Chiton` and `PhitonSpectrum::coupling`. The crate `phi-cal` in `phiace/crates/phical`, the Riemann sphere per channel, does not touch PhiTon. docs/85 names the collision and proposes one phical. That is not done.

phinum does not use PhiTon. The doc comment on `gemgum::SpectralIdentity` calls it "the phinum model like GPT", and that phrase is the only link. phiqlm is planned, and nothing of it is built. phifno 1.0.0 does not use PhiTon. docs/85's suggestion that it should, bands per degree, is declined for want of a measurement.

Inside phiano, the helix is PhiTon's one reader that ships a file. `SectorPalette` is its reader for printed sector names, in compose, personas, the server and chat. `SectorPalette::phiton_color` is called from `persona/fingerprint/traits.rs`. `Gemgum` binds PhiTon to `SpectralPhasor`, and `Facet::word_color`, `semantic_distance` and `spectral_identity` expose it, with two of the three marked dead. `SpectralPhasor::to_light_quantum` is on docs/82's dead list. The oscillator reads `SpectralBand::BANDS` directly.

phiecs does not read PhiTon. It talks to phiano through TSV, and the quest's scores never pass through a wavelength. phiace, the UI, does not read PhiTon. That is the gap most likely to be closed first. The Blender-parity UI draws, and a colour computed from a word's angle is what a view of a lexicon would want.

So PhiTon today is a leaf of phiano with one heavy sibling. The helix and the sector palette print it. Phical computes with it. The distances and energies built on it have no measured consumer. By docs/82's rule that a thing nothing depends on is not in the system, the printing half of PhiTon is in the system, and much of the computing half is not.

## 12. The next experiment

The paper's claim that wavelengths are for reading and angles are for arithmetic is argued from the code. It has not been measured. The quest gives a clean place to measure it, with code that exists and no new model.

The setting is PHIENT_FACTS.md's synthetic oracle in phiecs: 256 words in families of 8, five relation kinds (is-a, part-of, near, used-for, unlike), 1,280 rows, 293 target words, chance MRR 0.021. The lexicon is the one trained by attraction-only coupling with dithered rounding, which reached 0.594 MRR, hit@1 34.8% and hit@5 97.4%. Nothing is retrained. Only the scoring function changes.

The control is the shipped circular score. For each row, rank all 293 candidate targets by `cos(src_about[k] − tgt_candidate[k])` on the kind's channel `k`. This must reproduce 0.594. If it does not, the harness is wrong and nothing else is read.

The treatment is PhiTon's distance. Rank the same candidates by `|λ(src_about[k]) − λ(tgt_candidate[k])|`, smallest first, with both wavelengths from `phase_to_wavelength` at each word's own `band_n`. This is `spectral_distance` as shipped. A second treatment is the gemgum reading, `Gemgum::semantic_distance`, restricted to channel `k`, which adds the frequency term.

The prediction follows from section 9. Where a row's source angle and its true target's angle sit on the same side of angle 0, the linear distance and the circular distance order candidates in nearly the same way. Where they straddle angle 0, the true target is near on the circle and far on the spectrum, and the treatment should rank it low. So the experiment also reports the fraction of rows whose true pair straddles the cut, and MRR on straddling and non-straddling rows separately. The prediction is a drop in MRR confined to straddling rows, roughly in proportion to their share.

A third arm isolates the fine-structure shift. Rank by wavelength distance with every quantum forced to 0. If the shift carries anything, the arms at quantum 0 and at each word's own quantum differ. With a maximum shift of 0.949 nm against a byte of 1.445 nm, the expected difference is nil.

The pass condition for the paper's rule is the predicted drop on straddling rows, larger than the spread across three dithering seeds. If PhiTon's distance matches the circle, the rule is too strong, and the distances can stay. The byte deadband limits every quest score, so dithering stays on in every arm.

The run is not done. It needs one scoring loop and three distance functions that exist.

## 13. What is wrong or missing

`SpectralBand::from_wavelength(750.0)` returns violet, and `index_of(750.0)` returns 0. Crimson's upper bound is exclusive, and 750.0 is exactly where `phase_to_wavelength` clamps. Before docs/70's clamp, every converged word landed there. Today a continuous phase past 0.99744 of a turn at quantum 13 still does. The fix is an inclusive last band, or a clamp to just under 750.

The bands are not proportional, as the doc comment claims. Fourteen are 25.6 or 25.7 nm wide, red is 10.3 and crimson is 1.0. From stored bytes crimson is reached by byte 255 alone, at a quantum of 7 or more. Either the table should be sixteen equal bands of 23.125 nm, or the comment should say what it is.

The band names do not match the colours printed beside them. The band called yellow renders orange to red-orange, lime renders yellow, teal renders green. A strand line can read `yellow` and `#FF2A00`.

`band_colors(sector_count)` claims to return "all 16 band colors". At 64 sectors it returns violet twice and yellow twice, and no red or crimson. It samples sectors, and sectors are equal slices where bands are not.

The fine-structure shift exists in two units. PhiTon adds `quantum × α × 10` nanometres. Phical adds `quantum × α` radians, worth 5.588 nm at the clamp. The helix and gemgum therefore give one angle two wavelengths. For byte 26 at quantum 13 they are 418.527 nm and 423.166 nm.

The oscillator names bands by equal index and disagrees with the wavelength path on 178 of 256 byte angles.

`spectral_distance` says it handles wrap-around. It does not, and neither do `color_space_time_distance` and `Gemgum::semantic_distance`. Byte 0 and byte 255 are 368.6 nm apart.

`Chiton::from_quantum` always sets `spin_clockwise` to false, so `resonance` always applies 0.5 and the opposite spins the module describes never occur. `Chiton::to_quantum` adds the shift a second time, so the phiton-chiton-phiton cycle drifts by 0.949 nm per turn at quantum 13.

`band_position` returns [0, 0.618) where it promises [0, 1). `oscillation_energy` computes `(n + 1)·α·h·ν` where its comment says `n·α·h·ν`. `ALPHA` is exactly `1.0 / 137.0` where the module comment says `α ≈ 1/137`, so the approximation is fixed in the constant. `PhitonColor` and `SpectralBand` cannot be deserialised.

`src/phiton` has no tests. Seven items remain dead, per docs/82, and the helix revived none of them.

The unit lives in field names. There is no type for nanometres, terahertz or electronvolts, so the compiler cannot stop a wavelength being added to an angle.

No result in PHIENT_FACTS.md depends on PhiTon. Nothing has been measured about light and language. The module comments' claims about intelligence stand without a single number.

## 14. How it was found

No numbered doc records when PhiTon was written. The code says what it replaced. `Phiton::sector_color` and `Gemgum::phasor_to_color` each carry a comment saying they replace "the hard-coded color array in `SectorPalette`", and `bands.rs` says the sixteen bands replace it. So PhiTon began as a palette: a way to name a sector's colour from physics where a list of names had been. The fine-structure shift, the chiton and the couplings grew around that palette. None of them was written up with a measurement.

docs/70 is the first numbered doc that touches PhiTon's arithmetic. Its section 3 has the title "`band_n` is a counter; the quantum number is its clamp". It found that the trainer increments `band_n` on every converged channel-0 step, so on the shipped manifold it reaches 1,845,381. `SpectralPhasor::effective_phase` already clamped the counter to 13. Phical and PhiTon did not, and `Phiton::phase_to_wavelength` pushed every such word to 750 nm. docs/70 put `Phical::quantum` under every Phical and PhiTon formula and added the test that a count of 13 and a count of 1.8 million are the same point. Its change table lists `src/phiton/mod.rs` among the files touched.

docs/82 took an inventory of what nothing references. It found 112 public items with no reference anywhere, and listed `phiton` with 7, "wavelength/colour conversions". The inventory file names them: `center_nm`, `standing_wave_amplitude`, `to_quantum`, `wave_number`, `antipodal_color`, `band_colors` and `spectral_distance`. It also lists `to_light_quantum` among the phasor module's dead items.

docs/84 built the helix with a four-field strand line, codon, source byte, target byte and evidence, and reported 0.6 s and 75 MB for 18,496 words. PhiTon is not in it.

docs/85 took the inventory of every `phi*` crate on the day it was written, set out PML, and placed phiton as a child of phinet. Under "Do not recreate f64" it recorded the helix's new PhiTon columns and `love`'s `AAA` at 418.5 nm, indigo, `#5900F8`. It proposed bands per degree for phifno and declined it. It asked for the quantum in the helix header. It said not to put units in the tape. It ranked the lift of phiton into a crate first among five moves.

PHIENT_FACTS.md then gathered the helix into one line, with "PhiTon wavelength/band/hex per codon" and 1.9 s. Paper 7 of this series read the helix and its PhiTon columns, and reported the header's two meanings of `band`.

This paper read the five source files of the module, the helix writer and docs/85's section 3, and followed each PhiTon call to its callers. The defects in section 13 were found by running the source's formulas on the 256 byte angles, as arithmetic, and comparing what each function returns with what its comment says.

## 15. What remains

The fixes come first. Each is a few lines and a test. The last band should include 750.0, so that the reddest light is not named violet. The band table should either become sixteen equal bands or carry a comment that says what it is. `band_colors` should sample band centres. `spectral_distance` should either take the shorter way round or say in its comment that it does not. `band_position` should drop the no-op remainder or do what its comment says. `oscillation_energy` and its comment should agree. `Chiton::from_quantum` should either set a spin or the module comment should stop describing opposite spins. `Chiton::to_quantum` should subtract the shift it adds. A first test file for `phiton` can pin `love`'s 418.5 nm, the byte table in section 6 and the 750.0 case.

One shift should remain, in one unit. The choice is between phical's radians, which the clamp's derivation of 13.45 assumes, and PhiTon's nanometres, which the helix prints. Once one is chosen, the helix and gemgum will print one wavelength per angle. The oscillator should call `phase_to_color` so the family has one band name per angle.

The printing rule has two open items from docs/85. The helix header should print `Phical::quantum(band_n)` beside the counter, and one of the two fields called `band` should be renamed. phifno's bands per degree stay declined until a measurement asks for them.

The lift remains. docs/85's move 1 takes `phiano/src/phiton` into its own crate under phinet, depending on phinet's `c64` and phical's `quantum`, with phiano re-exporting it. The helix and the sector palette come with it as consumers, and the seven dead items come with it to be counted again. A `Nanometres` type and a band that deserialises would fit best in that crate, since a crate boundary is where a unit type pays for itself.

The experiment in section 12 remains: the quest ranked by the circle, by PhiTon's wavelength distance, and by the wavelength distance at quantum 0, with straddling rows reported apart. It is the one run that could turn this paper's rule into a measured result.

phiace remains the reader nobody has built: a lexicon view that colours each codon by `PhitonColor`.

The claim that stays, in plain terms: a Phiano angle can be printed as a wavelength, a band and a colour, and on `love`'s `AAA` it prints 418.5 nm, indigo, `#5900F8`. That reading is exact and useful to a person. The arithmetic built on wavelengths cuts the circle, carries two shifts, and has never been measured. Until it is, the angle remains the engine's unit and light is the unit it prints.

## 16. Sources

These are the local files read for this paper. Every number above is quoted from one of them or is arithmetic on a constant or function defined in them, and the arithmetic is marked where it appears. No source file was modified.

The eight files named for this paper are the five files of `src/phiton`, `src/helix/write.rs`, section 3 of docs/85 and PHIENT_FACTS.md. The rest were opened to follow a PhiTon call to its callers, to confirm a constant, or to read the history in section 14.

The worked values in sections 4, 6, 7, 8 and 9 were produced by transcribing `phase_to_wavelength`, `from_wavelength`, `wavelength_to_rgb`, `frequency_thz`, `energy_ev`, `effective_phase`, `resonance` and `coupling` line for line and running them over the 256 byte angles at quanta 0 to 13. That is arithmetic on the source, done once for this paper and kept nowhere in the tree. It reproduced docs/85's `love` reading exactly, which is the only check against a printed value available. No Rust was compiled or run for this paper, and no bin was called.

| path | what it gave |
|---|---|
| `/home/claude/phiano/docs/papers/phient/BRIEF.md` | the form of the paper |
| `/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md` | the paradigm paragraph; the family; the helix line (18,496 words, 1.9 s, 75 MB, PhiTon per codon); the quest numbers for section 12 |
| `/home/claude/phiano/src/phiton/mod.rs` | `Phiton`, `C`, `H`, the visible range, `phase_to_wavelength`, `wavelength_to_phase`, `frequency_thz`, `energy_ev`, `sector_color`, the module's claims |
| `/home/claude/phiano/src/phiton/types.rs` | `PhitonColor`, `LightQuantum`, `band_position`, `wavelength_to_rgb` |
| `/home/claude/phiano/src/phiton/bands.rs` | `SpectralBand`, the sixteen `BANDS`, `from_wavelength`, `index_of`, `at_index`, the warm, cool and green tests |
| `/home/claude/phiano/src/phiton/chiton.rs` | `Chiton`, `from_quantum`, `resonance`, `standing_wave_amplitude`, `to_quantum` |
| `/home/claude/phiano/src/phiton/spectrum.rs` | `PhitonSpectrum`, `phase_to_color`, `band_colors`, `spectral_distance`, `coupling`, `antipodal_color` |
| `/home/claude/phiano/src/helix/write.rs` | the strand line and its PhiTon columns |
| `/home/claude/phiano/docs/85_PML_STRUCTURE.md` | section 3, "Do not recreate f64"; sections 1, 2 and 4 for phiton's place and the lift |
| `/home/claude/phiano/docs/70_issues_closed.md` | section 3, the counter and the clamp, 1,845,381, 750 nm |
| `/home/claude/phiano/docs/82_missing_and_unused.md` | 112 dead items, 7 in `phiton` |
| `/home/claude/phiano/docs/82_unused_inventory.txt` | the names of the seven |
| `/home/claude/phiano/docs/84_sphere_and_helix.md` | the helix at 0.6 s and 75 MB |
| `/home/claude/phiano/docs/papers/phient/07_the_helix.md` | the helix as paper 7 read it |
| `/home/claude/phiano/src/phical/phical.rs` | `Phical::quantum`, `effective_phase`, `color_space_time_distance`, `oscillation_energy`, `ColorSpaceTimePoint` |
| `/home/claude/phiano/src/phical/tests.rs` | `counters_do_not_walk_the_wheel` |
| `/home/claude/phiano/src/config/constants/types.rs` | `ALPHA`, `PHI`, `TWO_PI`, `SECTOR_RESOLUTION` |
| `/home/claude/phiano/src/config/constants/channels_per_update.rs` | `BAND_N_EFFECTIVE_MAX` and its derivation |
| `/home/claude/phiano/src/gemgum.rs` | `phasor_to_color`, `sector_color_name`, `semantic_distance`, `band_position`, `word_coupling`, `SpectralIdentity` |
| `/home/claude/phiano/src/compose/mod.rs` | `SectorPalette` |
| `/home/claude/phiano/src/facet/spectral.rs` | `word_color`, `spectral_identity` |
| `/home/claude/phiano/src/oscillator/mod.rs` | `Oscillator::hue`, `Oscillator::color` by index |
| `/home/claude/phiano/src/wave/wave_ray_cast.rs` | `Wave::sector_count` |
