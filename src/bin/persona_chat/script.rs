#[allow(unused_imports)]
use super::*;

/// The questions a demo asks when none are given: ordinary, answerable by
/// anyone, and different enough that three writers should part company.
pub(crate) const SCRIPT: [&str; 6] = [
    "good morning how are you today",
    "what do you think about marriage",
    "tell me about money and family",
    "and what of love",
    "what should a young woman do with her afternoon",
    "why does any of this matter",
];

/// Eighteen questions for measurement (`PHIANO_CHAT_SCRIPT=long`): the six
/// above and twelve more on the same ordinary subjects, so a comparison of
/// two settings rests on 54 replies rather than 18.
pub(crate) const SCRIPT_LONG: [&str; 18] = [
    "good morning how are you today",
    "what do you think about marriage",
    "tell me about money and family",
    "and what of love",
    "what should a young woman do with her afternoon",
    "why does any of this matter",
    "what do you make of your neighbours",
    "is a letter better than a visit",
    "what is the use of a good education",
    "tell me about the weather and the roads",
    "what makes a man agreeable",
    "how should one behave at a dinner",
    "what do you fear most",
    "where would you go if you could travel",
    "what is the truth about pride",
    "do you believe in second chances",
    "what is a friend worth",
    "how does a story end",
];

/// Prints one exchange and what the engine did to produce it.
pub(crate) fn show(turn: usize, user: &str, name: &str, r: &Reply, verdict: Option<(String, f64)>, kn: Option<(String, f64, f64)>) {
    println!("\n**you** · {user}");
    let under = match (&r.topic, &r.calls_for) {
        (Some(t), Some(c)) => format!("under *{t}* ⇒ *{c}*"),
        (Some(t), None) => format!("under *{t}*"),
        _ => "no known condition in the question".to_string(),
    };
    let heard = match verdict {
        Some((who, conf)) => format!(" · sectors read **{who}** ({conf:.2})"),
        None => String::new(),
    };
    let heard = match kn {
        Some((who, ppl, margin)) => format!("{heard} · KN panel reads **{who}** (ppl {ppl:.0}, ×{margin:.2})"),
        None => heard,
    };
    println!("**{name}** · {}", r.text);
    println!("<sub>turn {turn} · {under} · fit {:.3} · quality {:.3} · repeat {:.0}%{heard}</sub>", r.fit, r.quality, 100.0 * r.repeat);
}
