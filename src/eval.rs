use crate::config;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use crate::wave::Wave;
use std::fmt;
use std::f64::consts::PI;

/// Verdict - qualitative assessment of a text's semantic quality.
///
/// Derived from the coherence, novelty, and resonance scores.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Input was empty - no tokens to evaluate.
    Empty,
    /// Mostly unknown vocabulary - sounds like noise.
    Noise,
    /// Words don't resonate but the input is novel - possibly creative.
    DissonantNovel,
    /// Words do not resonate together at all.
    Incoherent,
    /// Coherent and well-grounded in existing knowledge.
    CoherentGrounded,
    /// Coherent and introduces novel ideas - insightful.
    CoherentNovel,
    /// Moderately coherent with some novel elements.
    ModerateNovel,
    /// Coherent but familiar - no new information.
    CoherentFamiliar,
    /// Weakly coherent - marginal meaning.
    WeaklyCoherent,
}

impl Verdict {
    /// Determines a verdict from the three eval scores.
    pub fn from_scores(coherence: f64, novelty: f64, resonance: f64) -> Self {
        if resonance < 0.3 {
            return Self::Noise;
        }
        if coherence < 0.2 {
            return if novelty > 0.7 { Self::DissonantNovel } else { Self::Incoherent };
        }
        if coherence > 0.7 && novelty < 0.3 {
            return Self::CoherentGrounded;
        }
        if coherence > 0.7 && novelty > 0.6 {
            return Self::CoherentNovel;
        }
        if coherence > 0.5 {
            return if novelty > 0.5 { Self::ModerateNovel } else { Self::CoherentFamiliar };
        }
        Self::WeaklyCoherent
    }
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty input"),
            Self::Noise => write!(f, "Noise-like, mostly unknown vocabulary"),
            Self::DissonantNovel => write!(f, "Dissonant but novel - may be creative or nonsensical"),
            Self::Incoherent => write!(f, "Incoherent - words do not resonate together"),
            Self::CoherentGrounded => write!(f, "Coherent and well-grounded"),
            Self::CoherentNovel => write!(f, "Coherent and novel - insightful"),
            Self::ModerateNovel => write!(f, "Moderately coherent with novel elements"),
            Self::CoherentFamiliar => write!(f, "Coherent and familiar"),
            Self::WeaklyCoherent => write!(f, "Weakly coherent - marginal meaning"),
        }
    }
}

/// Eval - the result of evaluating a text against the facet.
///
/// Contains quantitative scores (coherence, novelty, resonance, overall)
/// and a qualitative verdict.
#[derive(Debug, Clone)]
pub struct Eval {
    /// How well the words resonate together (0.0 - 1.0).
    pub coherence: f64,
    /// How different the input is from the facet centroid (0.0 - 1.0).
    pub novelty: f64,
    /// Fraction of input tokens known by the facet (0.0 - 1.0).
    pub resonance: f64,
    /// Weighted combination of all scores (0.0 - 1.0).
    pub overall: f64,
    /// Qualitative assessment of the text.
    pub verdict: Verdict,
}

impl fmt::Display for Eval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  Coherence: {:.2}  Novelty: {:.2}  Resonance: {:.2}  Overall: {:.2}\n  Verdict: {}",
            self.coherence, self.novelty, self.resonance, self.overall, self.verdict,
        )
    }
}

/// Evaluator - scores text against the facet's semantic space.
///
/// Produces an `Eval` result with coherence, novelty, and resonance scores.
pub struct Evaluator;

impl Evaluator {
    /// Creates a new evaluator.
    pub fn new() -> Self {
        Self
    }

    /// Evaluates a text string against the facet.
    ///
    /// - **Resonance**: fraction of input tokens the facet knows
    /// - **Coherence**: how well the known words' wave superposition aligns
    /// - **Novelty**: distance of the input wave from the facet centroid
    /// - **Overall**: weighted combination (coherence 45%, resonance 40%, novelty 15%)
    pub fn eval(&self, facet: &Facet, text: &str) -> Eval {
        let tokens = Tokenizer::tokenize(text);
        if tokens.is_empty() {
            return Eval {
                coherence: 0.0,
                novelty: 0.0,
                resonance: 0.0,
                overall: 0.0,
                verdict: Verdict::Empty,
            };
        }

        let known = tokens.iter().filter(|t| facet.contains_word(t)).count();
        let resonance = known as f64 / tokens.len() as f64;

        let wave = Wave::sentence(facet, &tokens);
        let centroid = facet.centroid();

        let coherence = if known == 0 {
            0.0
        } else if known == 1 {
            // Single word: measure alignment with facet centroid direction
            let centroid_dir = centroid.arg();
            let word_phase = wave.arg();
            let alignment = (centroid_dir - word_phase).cos().max(0.0);
            alignment * 0.5 + 0.25
        } else {
            // Multiple words: Kuramoto order parameter
            (wave.norm() / known as f64).clamp(0.0, 1.0)
        };

        let novelty = if facet.vocabulary_size() > 0 && wave.norm() > 1e-10 {
            let centroid_arg = centroid.arg();
            let wave_arg = wave.arg();
            let angular_dist = ((centroid_arg - wave_arg).abs()).min(PI);
            let normalized = angular_dist / PI;
            (1.0 - (-(normalized * config::NOVELTY_SCALE * 5.0).min(20.0)).exp()).clamp(0.0, 1.0)
        } else {
            1.0
        };

        let overall = config::eval_overall(coherence, novelty, resonance);
        let verdict = Verdict::from_scores(coherence, novelty, resonance);

        Eval {
            coherence,
            novelty,
            resonance,
            overall,
            verdict,
        }
    }

}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
