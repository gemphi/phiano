#[allow(unused_imports)]
use super::*;

/// Function words. A gloss opens with them constantly and none is ever a genus.
///
/// The first version of this extractor filtered adjectives but not these, and
/// then picked the candidate with the highest definitional in-degree on the
/// theory that genus terms are general. Frequency-as-generality is exactly
/// backwards once function words are in the pool: *the* has the highest
/// in-degree in any dictionary, so every entry came back with
/// `genus(dog, the)`. The role-versus-bag comparison still ran, and still
/// showed a gap — on garbage input, which makes the number worthless.
pub(super) fn is_function_word(w: &str) -> bool {
    matches!(
        w,
        "a" | "an" | "the" | "of" | "or" | "and" | "to" | "in" | "is" | "are" | "as" | "that"
            | "which" | "with" | "for" | "by" | "on" | "at" | "from" | "it" | "its" | "any"
            | "one" | "used" | "being" | "having" | "also" | "was" | "were" | "be" | "been"
            | "this" | "these" | "those" | "not" | "but" | "so" | "than" | "then" | "into"
            | "upon" | "over" | "under" | "out" | "up" | "down" | "off" | "who" | "whom"
            | "what" | "when" | "where" | "while" | "all" | "some" | "each" | "every"
            | "more" | "most" | "less" | "least" | "very" | "much" | "may" | "can" | "will"
            | "would" | "should" | "must" | "has" | "have" | "had" | "does" | "did" | "do"
    )
}

/// Adjectives and quantifiers that open a gloss without being its genus.
pub(super) fn is_modifier(w: &str) -> bool {
    matches!(
        w,
        "small" | "large" | "great" | "little" | "young" | "old" | "common" | "certain"
            | "various" | "many" | "several" | "other" | "same" | "such" | "first" | "last"
            | "high" | "low" | "long" | "short" | "new" | "good" | "bad" | "hard" | "soft"
            | "domestic" | "wild" | "male" | "female" | "single" | "double" | "whole"
    )
}

/// Extracts constitutive rules from a Webster's gloss.
///
/// The genus is the load-bearing one, and Webster's makes it recoverable
/// because its entries are genus-differentia: *cat: **an animal** of various
/// species*, *car: a small **vehicle** moved on wheels*. The kind comes first,
/// after the article and any adjectives.
///
/// So the rule is simply the **first surviving content word**: drop function
/// words, drop modifiers, take what is left. Frequency is deliberately not used
/// to rank candidates — see [`is_function_word`] for what that cost.
///
/// `in_degree` is still consulted, but only as a *ceiling*: a candidate used in
/// more than a fifth of all glosses is a function word this list has missed,
/// and taking it as a genus would place half the dictionary under one node.
pub(super) fn extract(
    gloss: &str,
    in_degree: &HashMap<String, usize>,
    total_glosses: usize,
    head: &str,
) -> Vec<(Role, String)> {
    let toks: Vec<String> = Tokenizer::tokenize(gloss);
    let mut out = Vec::new();
    let ceiling = (total_glosses / 5).max(1);

    // ---- genus: first content word ----
    let genus = toks
        .iter()
        .take(6)
        .find(|w| {
            w.len() > 2
                && !is_function_word(w)
                && !is_modifier(w)
                && *w != head
                && in_degree.get(*w).copied().unwrap_or(0) < ceiling
        })
        .cloned();
    if let Some(g) = genus {
        out.push((Role::Genus, g));
    }

    // ---- function: "used to X", "used for X", "enables X", "serves to X" ----
    for (i, w) in toks.iter().enumerate() {
        let trigger = matches!(w.as_str(), "used" | "serves" | "enables" | "employed");
        if !trigger {
            continue;
        }
        if let Some(f) = toks
            .iter()
            .skip(i + 1)
            .take(4)
            .find(|x| x.len() > 3 && !is_modifier(x) && !is_function_word(x))
        {
            out.push((Role::Function, f.clone()));
            break;
        }
    }

    // ---- form: "made of X", "consisting of X", "composed of X" ----
    for (i, w) in toks.iter().enumerate() {
        if !matches!(w.as_str(), "made" | "consisting" | "composed" | "formed") {
            continue;
        }
        if let Some(f) = toks
            .iter()
            .skip(i + 1)
            .take(4)
            .find(|x| x.len() > 2 && !is_modifier(x) && !is_function_word(x))
        {
            out.push((Role::Form, f.clone()));
            break;
        }
    }

    out
}

/// Bag control: the same fillers, superposed with no role rotation.
pub(super) fn bag_identity(facet: &Facet, fillers: &[String]) -> Option<SpectralPhasor> {
    let mut acc = vec![(0.0f64, 0.0f64); PHASE_CHANNELS];
    let mut used = 0usize;
    for f in fillers {
        let p = match facet.lexicon.get(f) {
            Some(p) => p,
            None => continue,
        };
        used += 1;
        for (k, a) in acc.iter_mut().enumerate() {
            let t = p.theta(k);
            a.0 += t.cos();
            a.1 += t.sin();
        }
    }
    if used == 0 {
        return None;
    }
    let mut out = SpectralPhasor::seeded("bag", 1.0, 1);
    for (k, (x, y)) in acc.iter().enumerate() {
        if x.hypot(*y) > 1e-12 {
            out.set_theta(k, y.atan2(*x));
        }
    }
    out.sync_phase();
    Some(out)
}

pub(super) fn nearest(facet: &Facet, q: &SpectralPhasor, exclude: &[&str], k: usize) -> Vec<String> {
    let mut scored: Vec<(&str, f64)> = facet
        .lexicon
        .iter()
        .filter(|(w, _)| !exclude.contains(&w.as_str()))
        .map(|(w, p)| (w.as_str(), q.resonance(p)))
        .collect();
    scored.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.0.cmp(b.0))
    });
    scored.into_iter().take(k).map(|(w, _)| w.to_string()).collect()
}
