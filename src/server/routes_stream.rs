use super::SharedModel;
use super::types::TextRequest;
use crate::generate::Generator;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::Json;
use serde::Serialize;
use std::convert::Infallible;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;

#[derive(Serialize)]
struct StreamToken {
    token: String,
    step: usize,
    collective_phase: f64,
    resonance: f64,
    done: bool,
}

pub async fn generate_stream(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let max_tok = req.max_tokens.unwrap_or(24);
    let temp = req.temperature.unwrap_or(0.15);
    let generator = Generator::new(max_tok, temp);
    let mut ctx = crate::generate::ContextWaveBuffer::new(4096);
    let (tokens, flow) = generator.decode(&guard.facet, &mut ctx, &req.text);
    drop(guard);

    let mut events: Vec<StreamToken> = tokens
        .iter()
        .enumerate()
        .map(|(i, token)| {
            let step = flow.trajectory.get(i);
            StreamToken {
                token: token.clone(),
                step: i,
                collective_phase: step.map(|s| s.collective_phase).unwrap_or(flow.collective_phase),
                resonance: step.map(|s| s.resonance_score).unwrap_or(0.0),
                done: false,
            }
        })
        .collect();

    events.push(StreamToken {
        token: String::new(),
        step: events.len(),
        collective_phase: flow.collective_phase,
        resonance: flow.order_parameter,
        done: true,
    });

    let (tx, rx) = tokio::sync::mpsc::channel(events.len().max(1));
    tokio::spawn(async move {
        for evt in events {
            let data = serde_json::to_string(&evt).unwrap_or_default();
            if tx.send(Ok(Event::default().data(data))).await.is_err() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(40)).await;
        }
    });

    Ok(Sse::new(ReceiverStream::new(rx)).keep_alive(KeepAlive::default()))
}
