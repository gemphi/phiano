//! How every command is used, read from `data/command/usage.tsv`.
//!
//! Each command used to print its own usage lines and `help` printed a
//! second, differently worded copy of all of them in code. The wording
//! lives in the table now (the ruling of 2026-11-07: no data in code), and
//! this module is the only reader: [`say`] for a command asked with no
//! argument, [`listing`] for `help`, [`wants_argument`] for the dispatcher
//! that asks the table instead of letting every command check for itself,
//! and [`canonical`] for the other words a command answers to
//! (`data/command/alias.tsv`, docs/239).

use phistd::lexfile;

/// One form of one command: what a person types, what it does, the group
/// `help` lists it under, and the example and tip that go with it.
pub struct Usage {
    pub name: String,
    pub form: String,
    pub what: String,
    pub group: String,
    pub example: String,
    pub note: String,
}

/// The groups `help` lists, in the order it lists them.
const GROUP: [(&str, &str); 2] = [("core", "Core commands:"), ("driver", "Drivers (source/device commands):")];

/// The table, in the file's order. A missing file is an empty table, so a
/// command still answers (with nothing to say about itself).
pub fn all() -> Vec<Usage> {
    lexfile::read(crate::data::path("command/usage.tsv")).iter().filter_map(|l| row(l)).collect()
}

/// One line of the table, or `None` when it does not hold six fields.
fn row(line: &str) -> Option<Usage> {
    let f: Vec<&str> = line.split('\t').map(str::trim).collect();
    match f.len() {
        6 => Some(Usage {
            name: f[0].to_string(),
            form: f[1].to_string(),
            what: f[2].to_string(),
            group: f[3].to_string(),
            example: said(f[4]),
            note: said(f[5]),
        }),
        _ => None,
    }
}

/// A cell that says nothing (`-`) read as nothing.
fn said(cell: &str) -> String {
    match cell {
        "-" => String::new(),
        said => said.to_string(),
    }
}

/// Whether a row is this command's: the command itself, or one of its
/// longer forms (`learn` is `learn` and `learn multi`; `chat --verbose` is
/// only itself).
fn of(u: &Usage, name: &str) -> bool {
    u.name == name || u.name.starts_with(&format!("{name} "))
}

/// Says how `name` is used, as the dispatcher does when a command is asked
/// with no argument: the one form with its example and tip, or, for a
/// command of several forms (`persona`, `oscillator`), each form with what
/// it does, as `help` lists them.
pub fn say(name: &str) {
    let table = all();
    let rows: Vec<&Usage> = table.iter().filter(|u| of(u, name)).collect();
    match rows.as_slice() {
        [u] => {
            println!("  Usage: {}", u.form);
            if !u.example.is_empty() {
                println!("  Example: {}", u.example);
            }
            if !u.note.is_empty() {
                println!("  Tip: {}", u.note);
            }
        }
        several => print!("{}", lines(several)),
    }
}

/// What a form asks for after the command word: nothing (`chat`, the
/// interactive form), a choice in brackets (`layers [query]`), or
/// something required (`define <word>`).
fn asks_for(form: &str, name: &str) -> String {
    form.strip_prefix(name).unwrap_or(form).trim().to_string()
}

/// Whether this command must be given an argument: the table holds a form
/// of it, and no form of it stands alone or asks only for a choice.
/// `cognitive` must, `chat` and `layers` need not, and a word the table
/// does not hold needs none, so unknown input is still text to learn from.
pub fn wants_argument(name: &str) -> bool {
    let forms: Vec<Usage> = all().into_iter().filter(|u| of(u, name)).collect();
    let alone = |u: &Usage| {
        let asks = asks_for(&u.form, name);
        asks.is_empty() || asks.starts_with('[')
    };
    !forms.is_empty() && !forms.iter().any(alone)
}

/// Every form of one command, as `help` lists them: the form, then what it
/// does. A command with subcommands says this when it is given one it does
/// not know.
pub fn forms(name: &str) -> String {
    lines(&all().iter().filter(|u| of(u, name)).collect::<Vec<&Usage>>())
}

/// Each row as a line of its own, the forms in one column.
fn lines(rows: &[&Usage]) -> String {
    let widest = rows.iter().map(|u| u.form.chars().count()).max().unwrap_or(0);
    rows.iter().map(|u| format!("  {:widest$}  - {}\n", u.form, u.what)).collect()
}

/// The command a typed word is: itself, or what `alias.tsv` says it is.
/// Lowercased, since a command is one word whatever its case.
pub fn canonical(typed: &str) -> String {
    let typed = typed.to_lowercase();
    let row = |l: &String| {
        let (a, name) = l.split_once('\t')?;
        match a.trim() == typed {
            true => Some(name.trim().to_string()),
            false => None,
        }
    };
    lexfile::read(crate::data::path("command/alias.tsv")).iter().find_map(row).unwrap_or(typed)
}

/// The whole listing `help` prints: every group's forms, the columns as
/// wide as that group's widest form.
pub fn listing() -> String {
    let table = all();
    let mut out = String::new();
    for (group, heading) in GROUP {
        let of_group: Vec<&Usage> = table.iter().filter(|u| u.group == group).collect();
        if of_group.is_empty() {
            continue;
        }
        out += &format!("{heading}\n");
        out += &lines(&of_group);
        out += "\n";
    }
    out + "  Any unrecognized input is treated as text to learn from.\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_is_read_and_every_command_in_the_listing_can_say_itself() {
        let table = all();
        assert!(table.len() > 30, "the table reads: {} rows", table.len());
        for u in &table {
            assert!(!u.name.is_empty() && !u.form.is_empty() && !u.what.is_empty(), "{}", u.name);
            assert!(u.form.starts_with(&u.name) || u.name.contains(' '), "the form begins with the command: {}", u.form);
        }
        let groups: Vec<&String> = table.iter().map(|u| &u.group).filter(|g| *g != "-").collect();
        assert!(groups.iter().all(|g| GROUP.iter().any(|(k, _)| k == *g)), "an unknown group");
        say("define");
        say("persona");
    }

    #[test]
    fn the_listing_holds_every_group_and_lines_up() {
        let listing = listing();
        for (_, heading) in GROUP {
            assert!(listing.contains(heading), "missing heading: {heading}");
        }
        assert!(listing.contains("learn"), "{listing}");
        assert!(listing.ends_with("text to learn from.\n"));
        let dash: Vec<usize> =
            listing.lines().filter(|l| l.starts_with("  ") && l.contains(" - ")).map(|l| l.find(" - ").unwrap()).collect();
        assert!(dash.windows(2).filter(|w| w[0] != w[1]).count() <= GROUP.len(), "the columns line up within a group");
    }

    /// The dispatcher's two questions of the table: which words are
    /// commands at all, and which of them take an argument.
    #[test]
    fn the_table_says_which_commands_take_an_argument() {
        for takes in ["cognitive", "define", "eval", "generate", "learn", "resonance", "synonym", "wave"] {
            assert!(wants_argument(takes), "{takes} is used with an argument");
        }
        for alone in ["chat", "help", "save", "load", "stats", "exit", "layers", "synthetic"] {
            assert!(!wants_argument(alone), "{alone} is used alone");
        }
        assert!(!wants_argument("a sentence the table never heard of"), "unknown input is text to learn from");
    }

    /// The other words a command answers to are data, so the dispatcher's
    /// match holds one name per command (docs/239).
    #[test]
    fn a_typed_word_is_the_command_it_names() {
        for (typed, command) in [("cog", "cognitive"), ("OM", "oscillator"), ("ask", "instruct"), ("?", "help")] {
            assert_eq!(canonical(typed), command, "{typed}");
        }
        assert_eq!(canonical("Define"), "define", "a command is one word whatever its case");
        assert_eq!(canonical("hello there"), "hello there", "a word the table does not hold is itself");
    }

    /// What a command with subcommands says when it is given one it does
    /// not know: the same lines `help` lists it by.
    #[test]
    fn a_commands_own_forms_are_the_lines_help_lists() {
        let oscillator = forms("oscillator");
        for form in ["oscillator eval", "oscillator wheel", "oscillator train"] {
            assert!(oscillator.contains(form), "{oscillator}");
        }
        assert!(oscillator.lines().all(|l| l.contains(" - ")), "{oscillator}");
        assert!(forms("persona").contains("persona list"), "{}", forms("persona"));
        assert!(forms("there-is-no-such-command").is_empty());
        // The same table, each padded to its own widest form: one command's
        // forms line up with each other, `help`'s with the whole group.
        let what = "Evaluate text in oscillator mode";
        assert!(oscillator.contains(what) && listing().contains(what), "{oscillator}");
    }

    /// A line the table does not hold is no usage at all, and no panic.
    #[test]
    fn a_command_the_table_does_not_hold_says_nothing() {
        say("there-is-no-such-command");
        assert!(row("one\ttwo").is_none(), "a short line is not a row");
    }
}
