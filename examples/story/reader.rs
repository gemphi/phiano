//! The operator's reading of what was said, as the base the levels move
//! from (docs/137, docs/151). For each offset the operator was trained on —
//! the last word under `next`, the one before it under `next2`, … — its
//! cosines are softened at the temperature that gives the told rows the
//! most likelihood. The offsets are then read as one mixture, with weights
//! counted on told positions (`relate::mixture_weights`, the path rule of
//! docs/149). The prior — what the counts read with no condition — is a
//! component of the same mixture (docs/152): where no offset knows the
//! word, frequency still does. Everything is fitted on what was told, never
//! on what is held out. The operator saw those rows in training, so the
//! fit trusts it more than unseen text would; `--dev` fits on paragraphs
//! the operator never saw (docs/152). `fit_with` and `read_with` take any
//! reading of the words before — the phase-conjugate mirror's (docs/153)
//! as well as the operator's.

use super::soften;
use phiano::prose::Base;
use phiano::relate::{kind_token, mixture_weights, RelationModel};
use phiecs::quest::Row;
use std::cell::RefCell;
use std::rc::Rc;

const GRID: [f64; 6] = [0.01, 0.02, 0.05, 0.1, 0.2, 0.5];

pub struct Reader {
    /// The counts' prior over the values, the mixture's first component.
    pub prior: Vec<f64>,
    /// What each offset is read under (`next`, `next2`, or `H²`…), for the note.
    pub names: Vec<String>,
    pub temps: Vec<f64>,
    /// The prior's weight, then each offset's.
    pub weights: Vec<f64>,
}

/// A reading of the word `d` places back (`d = 0`: the last word) as scores
/// over the values, or `None` when it cannot be read.
pub type Reading<'a> = dyn FnMut(&str, usize) -> Option<Vec<f64>> + 'a;

/// The kinds an operator was trained on, in offset order.
fn kinds(m: &RelationModel) -> Vec<String> {
    (1..=8).map(|d| if d == 1 { "next".to_string() } else { format!("next{d}") }).take_while(|k| m.index.contains_key(&kind_token(k))).collect()
}

impl Reader {
    /// As `fit_with`, for the offsets an operator was trained on.
    pub fn fit(m: &mut RelationModel, told: &[Row], values: &[String], prior: &[f64], n: usize) -> Reader {
        let names = kinds(m);
        let ks = names.clone();
        Reader::fit_with(&mut |w: &str, d: usize| m.scores(w, &ks[d], values), names, told, values, prior, n)
    }

    /// As `read_with`, for an operator.
    pub fn read(&self, m: &mut RelationModel, said: &[String], values: &[String]) -> Option<Vec<f64>> {
        let ks = kinds(m);
        self.read_with(&mut |w: &str, d: usize| m.scores(w, ks.get(d)?, values), said)
    }

    /// Fitted on `n` positions sampled from `told`, the `next` rows in
    /// paragraph order (a row's words before it are the rows that run into
    /// it), for one offset per name.
    pub fn fit_with(read: &mut Reading, names: Vec<String>, told: &[Row], values: &[String], prior: &[f64], n: usize) -> Reader {
        let at: std::collections::HashMap<&str, usize> = values.iter().enumerate().map(|(i, v)| (v.as_str(), i)).collect();
        let back = |j: usize, d: usize| (d <= j && (1..=d).all(|e| told[j - e].other == told[j - e + 1].about)).then(|| told[j - d].about.as_str());
        let step = (told.len() / n.max(1)).max(1);
        // Each sampled position: the truth's index and every offset's scores (None where the paragraph has no word that far back).
        let sample: Vec<(usize, Vec<Option<Vec<f64>>>)> = (0..told.len())
            .step_by(step)
            .take(n)
            .filter_map(|j| Some((*at.get(told[j].other.as_str())?, (0..names.len()).map(|d| read(back(j, d)?, d)).collect())))
            .collect();
        let temps: Vec<f64> = (0..names.len())
            .map(|d| {
                let score = |t: f64| sample.iter().filter_map(|(i, c)| Some(soften(c[d].as_ref()?, t)[*i].max(1e-12).ln())).sum::<f64>();
                GRID.iter().copied().fold((0.1, f64::MIN), |a, t| if score(t) > a.1 { (t, score(t)) } else { a }).0
            })
            .collect();
        let rows: Vec<(f64, Vec<f64>)> = sample
            .iter()
            .filter(|(_, c)| c.iter().all(Option::is_some))
            .map(|(i, c)| (1.0, std::iter::once(prior[*i]).chain(c.iter().zip(&temps).map(|(c, t)| c.as_ref().map_or(0.0, |c| soften(c, *t)[*i]))).collect()))
            .collect();
        let weights = mixture_weights(&rows, names.len() + 1);
        Reader { prior: prior.to_vec(), names, temps, weights }
    }

    /// The mixture for what was `said`, its weights renormalised over the
    /// prior and the offsets that could be read.
    pub fn read_with(&self, read: &mut Reading, said: &[String]) -> Option<Vec<f64>> {
        let mut d: Vec<f64> = self.prior.iter().map(|p| self.weights[0] * p).collect();
        let mut z = self.weights[0];
        for (o, ((t, w), word)) in self.temps.iter().zip(&self.weights[1..]).zip(said.iter().rev()).enumerate() {
            if let Some(c) = read(word, o) {
                d.iter_mut().zip(soften(&c, *t)).for_each(|(d, p)| *d += w * p);
                z += w;
            }
        }
        Some(d.into_iter().map(|p| p / z).collect())
    }

    /// One line on what was fitted.
    pub fn note(&self) -> String {
        let parts: Vec<String> = self.names.iter().zip(&self.temps).zip(&self.weights[1..]).map(|((k, t), w)| format!("{k} T = {t}, weight {w:.3}")).collect();
        format!("prior weight {:.3} · {}", self.weights[0], parts.join(" · "))
    }

    /// `Knowing::base` from the operator.
    pub fn base(self, m: Rc<RefCell<RelationModel>>) -> Base {
        Box::new(move |said: &[String], values: &[String]| self.read(&mut m.borrow_mut(), said, values))
    }
}
