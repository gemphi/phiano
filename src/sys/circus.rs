//! The circus (docs/161): two beings talk. Nothing is generated — a line is
//! a row the speaker's world was told, said under one of its intents, and
//! the listener *hears* it: the row is told to the listener's world, so
//! talking changes what each knows (Investigations §43: meaning is use —
//! here, literally counted).
//!
//! A turn: the speaker takes the word on the table, picks the intent under
//! which its world has the most to say about it, says the heaviest
//! answer, and the answer is the next word. A speaker with nothing to say
//! about the word changes the subject to one of its own seeds.

use super::{Agent, Being, Row};

/// One line of the script.
#[derive(Debug, Clone, PartialEq)]
pub struct Turn {
    pub speaker: String,
    pub about: String,
    pub kind: String,
    pub other: String,
    pub weight: f64,
    /// True when the speaker had nothing on the word and changed subject.
    pub changed_subject: bool,
}

/// The one move: what `who` says about `word`, under its intents — the
/// heaviest line not already said in `script` (a circus does not repeat).
pub fn line(who: &Being, word: &str, script: &[Turn]) -> Option<(String, String, f64)> {
    let said = |k: &str, o: &str| script.iter().any(|t| t.about == word && t.kind == k && t.other == o);
    who.persona
        .intents
        .iter()
        .flat_map(|k| who.ask(word, k).into_iter().map(move |(o, w)| (k.clone(), o, w)))
        .filter(|(k, o, _)| !said(k, o))
        .max_by(|a, b| a.2.total_cmp(&b.2).then_with(|| b.0.cmp(&a.0)))
}

/// `turns` lines between `a` and `b`, starting from `word` (or `a`'s first
/// seed), each line heard by the other. Returns the script.
pub fn talk(a: &mut Being, b: &mut Being, word: Option<&str>, turns: usize) -> Vec<Turn> {
    let mut script = Vec::new();
    let mut word = word.map(str::to_string).or_else(|| a.persona.seeds.first().cloned()).unwrap_or_default();
    for t in 0..turns {
        let (speaker, listener) = match t % 2 == 0 {
            true => (&mut *a, &mut *b),
            false => (&mut *b, &mut *a),
        };
        let mut changed = false;
        let said = match line(speaker, &word, &script) {
            Some(x) => Some(x),
            None => {
                changed = true;
                speaker.persona.seeds.iter().find_map(|s| line(speaker, s, &script).map(|x| (s.clone(), x))).map(|(s, x)| {
                    word = s;
                    x
                })
            }
        };
        let Some((kind, other, weight)) = said else { break };
        listener.tell(&[Row::weighted(&word, &kind, &other, weight)]);
        script.push(Turn { speaker: speaker.name().to_string(), about: word.clone(), kind, other: other.clone(), weight, changed_subject: changed });
        word = other;
    }
    script
}

/// The script as lines a person reads: `Name: word — kind → other (w)`.
pub fn render(script: &[Turn]) -> String {
    script
        .iter()
        .map(|t| {
            let aside = if t.changed_subject { " (changing the subject)" } else { "" };
            format!("{}{aside}: {} — {} → {} ({:.2})\n", t.speaker, t.about, t.kind, t.other, t.weight)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sys::Persona;

    fn being(name: &str, seeds: &str, rows: &[(&str, &str, &str)]) -> Being {
        let p = Persona::from_md(&format!("---\nname: {name}\nseeds: {seeds}\nkinds: has, is-a\n---\n")).unwrap();
        let mut b = Being::new(p);
        b.tell(&rows.iter().map(|(a, k, o)| Row::new(a, k, o)).collect::<Vec<_>>());
        b
    }

    #[test]
    fn talking_is_hearing() {
        let mut gates = being("Gates", "software", &[("software", "has", "license"), ("license", "is-a", "contract")]);
        let mut zuck = being("Zuckerberg", "graph", &[("license", "has", "terms"), ("graph", "has", "feed")]);
        let script = talk(&mut gates, &mut zuck, None, 4);
        assert_eq!(script[0].speaker, "Gates");
        assert_eq!((script[0].about.as_str(), script[0].other.as_str()), ("software", "license"));
        assert_eq!(script[1].speaker, "Zuckerberg");
        assert_eq!(script[1].about, "license", "the answer is the next word");
        assert!(zuck.ask("software", "has").iter().any(|(o, _)| o == "license"), "Zuckerberg heard the first line");
        assert!(render(&script).starts_with("Gates: software — has → license"));
    }

    #[test]
    fn nothing_to_say_changes_the_subject() {
        let mut a = being("A", "x", &[("x", "has", "y")]);
        let mut b = being("B", "z", &[("z", "has", "w")]);
        let script = talk(&mut a, &mut b, None, 2);
        assert_eq!(script.len(), 2);
        assert!(script[1].changed_subject, "B knows nothing of y and turns to z");
        assert_eq!(script[1].about, "z");
    }
}
