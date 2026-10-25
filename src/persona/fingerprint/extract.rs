#[allow(unused_imports)]
use super::*;

impl Fingerprint {
    pub(super) fn is_stop_word(word: &str) -> bool {
        crate::config::PhiConfig::is_stop_word(word)
    }

    /// The mean target-side function of the evidenced lexicon: the bias every
    /// word shares. Sectors are read on the *centred* function (word minus
    /// this mean), because the raw centroids of a whole lexicon bunch into a
    /// few sectors (45–49 on Austen, docs/74) and a histogram over them
    /// cannot tell authors apart. Centring is what a channel-balanced sector
    /// means: what this word does that the average word does not.
    pub fn lexicon_mean(facet: &Facet) -> Vec<crate::wave::c64> {
        let mut sum = vec![crate::wave::c64::new(0.0, 0.0); crate::config::PHASE_CHANNELS];
        let mut n = 0usize;
        for p in facet.lexicon.values() {
            if p.has_evidence() {
                for (a, b) in sum.iter_mut().zip(crate::phinet::space::encode_side(p, true)) {
                    *a += b;
                }
                n += 1;
            }
        }
        if n > 0 {
            for z in sum.iter_mut() {
                *z /= n as f64;
            }
        }
        sum
    }

    /// The sector a word occupies on the relation circle: the Q64 sector of
    /// the centroid of its evidence-weighted *target-side* function — the
    /// side a writer's choices put it under — centred on the lexicon mean,
    /// when the lexicon carries relation evidence; the legacy single phase
    /// otherwise. The single phase (channel 0, source side) collapsed for
    /// frequent words (docs/69) and every persona read the same four sectors
    /// from it; the uncentred centroid bunched into sectors 45–49 (docs/74).
    pub fn relational_sector(facet: &Facet, word: &str, mean: &[crate::wave::c64]) -> Option<u16> {
        let p = facet.lexicon.get(word)?;
        if !p.has_evidence() {
            return Wave::word_sector(facet, word);
        }
        let f: Vec<crate::wave::c64> = crate::phinet::space::encode_side(p, true).iter().zip(mean).map(|(a, m)| a - m).collect();
        Some(phistd::hexagram::hexagram_of(&f) as u16)
    }

    /// Extracts a fingerprint from text examples using the facet.
    pub fn extract(facet: &Facet, examples: &[String]) -> Self {
        let n = Wave::sector_count() as usize;
        let mut histogram = vec![0.0f64; n];
        let mut total_length = 0usize;
        let mean = Self::lexicon_mean(facet);

        for example in examples {
            let tokens = Tokenizer::tokenize(example);
            total_length += tokens.len();

            let content_tokens: Vec<String> = tokens
                .iter()
                .filter(|t| !Self::is_stop_word(t))
                .cloned()
                .collect();

            let sentence_tokens = if content_tokens.is_empty() { &tokens } else { &content_tokens };
            // The sentence's sector: the centroid of its words' target-side
            // functions when evidenced, the legacy wave otherwise.
            let evidenced = sentence_tokens.iter().filter_map(|t| facet.lexicon.get(t)).any(|p| p.has_evidence());
            let sector = if evidenced {
                let mut sum = vec![crate::wave::c64::new(0.0, 0.0); crate::config::PHASE_CHANNELS];
                for t in sentence_tokens {
                    if let Some(p) = facet.lexicon.get(t) {
                        if p.has_evidence() {
                            for ((a, b), m) in sum.iter_mut().zip(crate::phinet::space::encode_side(p, true)).zip(&mean) {
                                *a += b - m;
                            }
                        }
                    }
                }
                phistd::hexagram::hexagram_of(&sum) as usize
            } else {
                Wave::wave_sector(Wave::sentence(facet, sentence_tokens)) as usize
            };
            if sector < n {
                histogram[sector] += 1.0;
            }

            for token in &content_tokens {
                if let Some(word_sector) = Self::relational_sector(facet, token, &mean) {
                    let s = word_sector as usize;
                    if s < n {
                        let amp = facet.get_phasor(token).map(|p| p.amplitude).unwrap_or(1.0);
                        let weight = crate::config::FINGERPRINT_WORD_WEIGHT / (1.0 + amp);
                        histogram[s] += weight;
                    }
                }
            }
        }

        let total: f64 = histogram.iter().sum();
        if total > 0.0 {
            for h in &mut histogram { *h /= total; }
        }

        let mut dominant: Vec<(u16, f64)> = histogram
            .iter()
            .enumerate()
            .map(|(i, &w)| (i as u16, w))
            .collect();
        dominant.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        dominant.retain(|(_, w)| *w > 0.0);

        let entropy: f64 = histogram
            .iter()
            .filter(|&&h| h > 0.0)
            .map(|&h| -h * h.ln())
            .sum();

        let avg_length = if examples.is_empty() {
            0.0
        } else {
            total_length as f64 / examples.len() as f64
        };

        Fingerprint {
            sample_count: examples.len(),
            sector_histogram: histogram,
            dominant, avg_length,
            diversity: entropy,
        }
    }

    /// Computes cosine similarity between two fingerprints.
    pub fn similarity(&self, other: &Fingerprint) -> f64 {
        let n = self.sector_histogram.len().min(other.sector_histogram.len());
        if n == 0 { return 0.0; }

        let mut dot = 0.0f64;
        let mut norm_a = 0.0f64;
        let mut norm_b = 0.0f64;

        for i in 0..n {
            let a = self.sector_histogram[i];
            let b = other.sector_histogram[i];
            dot += a * b;
            norm_a += a * a;
            norm_b += b * b;
        }

        if norm_a == 0.0 || norm_b == 0.0 { return 0.0; }
        dot / (norm_a.sqrt() * norm_b.sqrt())
    }

    /// Overlap with another fingerprint after diffusing both over the circle
    /// for time `t` (Laplace–Beltrami heat flow, exact in Fourier modes):
    /// the Bhattacharyya coefficient in [0, 1]. At `t = 0` two personas that
    /// share no sector overlap at 0; as `t` grows, personas in *nearby*
    /// sectors overlap before distant ones do. The `t` that maximises
    /// attribution confidence on short texts is the persona's reach.
    pub fn overlap_after(&self, other: &Fingerprint, t: f64) -> f64 {
        let n = self.sector_histogram.len().min(other.sector_histogram.len());
        crate::physics::exact::heat::overlap_after(&self.sector_histogram[..n], &other.sector_histogram[..n], t)
    }

    /// The Q64 name of this fingerprint: the sector of its circular centroid.
    pub fn q64(&self) -> u8 {
        let n = self.sector_histogram.len().max(1) as f64;
        let (mut x, mut y) = (0.0f64, 0.0f64);
        for (i, w) in self.sector_histogram.iter().enumerate() {
            let th = (i as f64 + 0.5) / n * std::f64::consts::TAU;
            x += w * th.cos();
            y += w * th.sin();
        }
        crate::physics::exact::q64::quantise(y.atan2(x))
    }
}
