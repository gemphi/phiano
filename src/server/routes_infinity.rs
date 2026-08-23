/// Infinity resonance system types and route handlers.

use super::SharedModel;
use super::types::*;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct WordPhasorDetail {
    pub word: String,
    pub phase: f64,
    pub amplitude: f64,
    pub effective_phase: f64,
    pub sector: u16,
}

#[derive(Serialize)]
pub struct ComplexDetail {
    pub re: f64,
    pub im: f64,
    pub amp: f64,
    pub phase: f64,
}

#[derive(Serialize)]
pub struct VariationDetail {
    pub sector: u16,
    pub color: String,
    pub text: String,
    pub resonance: f64,
    pub wave: ComplexDetail,
    pub words: Vec<WordPhasorDetail>,
}

#[derive(Serialize)]
pub struct InfinityResponse {
    pub variations: Vec<VariationDetail>,
    pub prompt_wave: ComplexDetail,
}

#[derive(Serialize)]
pub struct InfinityTrainResponse {
    pub success: bool,
    pub message: String,
    pub tokens: usize,
    pub vocabulary: usize,
    pub shifts: Vec<WordShiftDetail>,
}

#[derive(Serialize)]
pub struct WordShiftDetail {
    pub word: String,
    pub phase_before: f64,
    pub phase_after: f64,
    pub shift: f64,
}

pub async fn infinity_visualize(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<InfinityResponse>, StatusCode> {
    let facet = {
        let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        guard.facet.clone()
    };
    let prompt = &req.text;

    let prompt_wave = crate::wave::Wave::text(&facet, prompt);
    let prompt_wave_detail = ComplexDetail {
        re: prompt_wave.re,
        im: prompt_wave.im,
        amp: prompt_wave.norm(),
        phase: prompt_wave.arg(),
    };

    let flows = crate::compose::flow::RiverFlow::generate_variations(&facet, prompt, 8);
    let mut variations = Vec::with_capacity(flows.len());

    for flow in flows {
        let sector = flow.source_sector;
        let color = crate::compose::sector_color(sector);
        let text = flow.text;
        let wave = crate::wave::Wave::text(&facet, &text);
        let theta = (sector as f64) * 2.0 * std::f64::consts::PI / (crate::wave::sectors() as f64);
        let diff = theta - wave.arg();
        let denom = 1.0 - 0.95 * diff.cos();
        let resonance = wave.norm() / denom;

        let wave_detail = ComplexDetail {
            re: wave.re, im: wave.im, amp: wave.norm(), phase: wave.arg(),
        };

        let tokens = crate::tokenizer::Tokenizer::tokenize(&text);
        let mut words = Vec::new();
        for token in tokens {
            if let Some(p) = facet.get_phasor(&token) {
                let effective_phase = p.phase + (p.band_n as f64 * crate::config::ALPHA);
                let word_sector = crate::wave::Wave::sector_of(effective_phase);
                words.push(WordPhasorDetail {
                    word: token, phase: p.phase, amplitude: p.amplitude,
                    effective_phase, sector: word_sector,
                });
            }
        }
        variations.push(VariationDetail { sector, color, text, resonance, wave: wave_detail, words });
    }

    Ok(Json(InfinityResponse { variations, prompt_wave: prompt_wave_detail }))
}

pub async fn infinity_train(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<InfinityTrainResponse>, StatusCode> {
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;
    let text = req.text.clone();
    let tokens = crate::tokenizer::Tokenizer::tokenize(&text);

    let mut before_phases = std::collections::HashMap::new();
    for token in &tokens {
        if let Some(p) = model.facet.get_phasor(token) {
            before_phases.insert(token.clone(), p.phase);
        }
    }

    let tokens_count = model.trainer.train_online(&mut model.facet, &text);

    let mut shifts = Vec::new();
    for token in &tokens {
        if let Some(p) = model.facet.get_phasor(token) {
            let phase_before = before_phases.get(token).cloned().unwrap_or(0.0);
            let phase_after = p.phase;
            let mut diff = phase_after - phase_before;
            while diff > std::f64::consts::PI { diff -= 2.0 * std::f64::consts::PI; }
            while diff < -std::f64::consts::PI { diff += 2.0 * std::f64::consts::PI; }
            shifts.push(WordShiftDetail { word: token.clone(), phase_before, phase_after, shift: diff });
        }
    }
    shifts.sort_by_key(|s| s.word.clone());
    shifts.dedup_by(|a, b| a.word == b.word);

    Ok(Json(InfinityTrainResponse {
        success: true,
        message: format!("Successfully trained Phiano on \"{}\"", text),
        tokens: tokens_count,
        vocabulary: model.facet.vocabulary_size(),
        shifts,
    }))
}
