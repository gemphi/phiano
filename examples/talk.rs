//! `talk` — ask the person (docs/192). The whole corpus is told to the
//! phasors once (on the fly — a longer corpus just takes longer, nothing is
//! retrained), then each question is a CONDITION: the words it ends on
//! are what the person is asked to go on from, in the voice the phasors
//! hold, read deepest channel first (the word before, the family, next).
//! A question's own words that the person has never spoken are skipped
//! back over until one it knows is found — it answers from what it knows.
//!
//! ```text
//! cargo run --release -p phiano --example talk -- <corpus> "question one" "question two" …
//! cargo run --release -p phiano --example talk -- <corpus> --words 14 "when do we use the present perfect"
//! ```

use phiano::arena::Ecs;
use phient::prose::tokens;
use phient::study::Reader;

fn sentences(path: &str) -> Vec<Vec<String>> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .split(|c| matches!(c, '.' | '!' | '?' | '\n'))
        .map(tokens)
        .map(|w| w.into_iter().filter(|t| t.chars().any(char::is_alphanumeric)).collect::<Vec<_>>())
        .filter(|w| w.len() >= 4)
        .collect()
}

/// The answer: go on from the end of the question for `words` words,
/// stopping when the voice starts repeating itself; if the person has
/// nothing after the question's last words, back off to earlier ones.
fn answer(person: &mut Ecs, question: &str, words: usize, tau: f64) -> String {
    let q = tokens(question);
    let q: Vec<String> = q.into_iter().filter(|t| t.chars().any(char::is_alphanumeric)).collect();
    let mut from = q.len();
    let mut said: Vec<String> = Vec::new();
    while from > 0 {
        said = q[..from].to_vec();
        if person.guess(&said).is_some() {
            break;
        }
        from -= 1;
    }
    if from == 0 {
        return "— (nothing of that is known yet)".into();
    }
    let start = said.len();
    let mut seen: Vec<(String, String)> = Vec::new();
    while said.len() < start + words {
        let read = person.ranked(&said);
        // Speak only where the difference is REAL. A crowded word reads as
        // a near-tie, the condition does not determine what follows, and a
        // voice that speaks anyway drifts into the crowd (docs/198).
        if Ecs::clearance(&read) < tau {
            break;
        }
        let Some((next, _)) = read.first().cloned() else { break };
        let pair = (said.last().cloned().unwrap_or_default(), next.clone());
        if seen.contains(&pair) {
            break;
        }
        seen.push(pair);
        said.push(next);
    }
    let picked = q[..from].join(" ");
    format!("[{picked}] {}", said[start..].join(" "))
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let Some(path) = a.first().cloned() else {
        eprintln!("which corpus?");
        return;
    };
    let tau: f64 = a.iter().position(|x| x == "--clear").and_then(|i| a.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(0.0);
    let words = a.iter().position(|x| x == "--words").and_then(|i| a.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(12usize);
    let questions: Vec<&String> = a.iter().skip(1).filter(|x| !x.starts_with("--") && x.parse::<f64>().is_err()).collect();

    let dump = a.iter().position(|x| x == "--read").and_then(|i| a.get(i + 1)).cloned();
    let lines = sentences(&path);
    let t0 = std::time::Instant::now();
    let mut person = Ecs::new();
    for w in &lines {
        person.tell(w);
    }
    println!("# Talking to {path}\n");
    println!("{} sentences told to the phasors in {:.1} s — on the fly, nothing retrained.\n", lines.len(), t0.elapsed().as_secs_f64());
    if let Some(d) = dump {
        // The read itself, for a condition: what the phasors actually say,
        // and by how much — the difference, laid out.
        let said: Vec<String> = tokens(&d).into_iter().filter(|t| t.chars().any(char::is_alphanumeric)).collect();
        let read = person.ranked(&said);
        println!("read after [{}] — {} answers, clearance {:.4}", said.join(" "), read.len(), Ecs::clearance(&read));
        for (w, s) in read.iter().take(10) {
            println!("  {s:>10.5}  {w}");
        }
        return;
    }
    for q in questions {
        println!("Q: {q}\nA: {}\n", answer(&mut person, q, words, tau));
    }
}
