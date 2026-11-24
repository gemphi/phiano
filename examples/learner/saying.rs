//! The sayings: every prompt gone on from by the learner and by the
//! counts, side by side, as the run's last table.
//!
//! What is asked is `data/person/prompt.txt`, one prompt a line - a hundred
//! of them since docs/238, seven in the code before, too few to read a
//! tendency from (docs/232, 235).

use phient::lineup::learner::Learner;
use phient::lineup::Count;
use phient::prose::tokens;

/// How many words each model is let say after the prompt.
const ON: usize = 12;

/// Prints the table: the prompt, the learner's continuation, the counts'.
pub fn say(learner: &Learner, count: &Count) {
    println!("\n| asked | the learner goes on | the counts go on |\n|---|---|---|");
    for q in phistd::lexfile::read(phiano::data::path("person/prompt.txt")) {
        let asked = tokens(&q);
        let mine = learner.speak(&asked, ON).join(" ");
        println!("| {q} | {mine} | {} |", counted(&asked, count).join(" "));
    }
}

/// Where the counts go from a prompt: the likeliest word after the last,
/// again and again, stopping where a pair of words comes round twice.
fn counted(asked: &[String], count: &Count) -> Vec<String> {
    let mut chain: Vec<String> = asked.to_vec();
    while chain.len() < asked.len() + ON {
        let Some(next) = count.after(chain.last().map(String::as_str).unwrap_or("")).cloned() else { break };
        let last = chain.last().cloned().unwrap_or_default();
        match chain.windows(2).any(|p| p[0] == last && p[1] == next) {
            true => break,
            false => chain.push(next),
        }
    }
    chain[asked.len()..].to_vec()
}
