//! How every command is used, read from `data/command/usage.tsv`.
//!
//! Each command used to print its own usage lines and `help` printed a
//! second, differently worded copy of all of them in code. The wording
//! lives in the table now (the ruling of 2026-11-07: no data in code), and
//! this module is the only reader: [`say`] for a command asked with no
//! argument, [`listing`] for `help`.

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

/// Says how `name` is used - every form of it, with its example and tip -
/// as a command does when it is asked with no argument.
pub fn say(name: &str) {
    for u in all().iter().filter(|u| u.name == name) {
        println!("  Usage: {}", u.form);
        if !u.example.is_empty() {
            println!("  Example: {}", u.example);
        }
        if !u.note.is_empty() {
            println!("  Tip: {}", u.note);
        }
    }
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
        let widest = of_group.iter().map(|u| u.form.chars().count()).max().unwrap_or(0);
        for u in of_group {
            out += &format!("  {:widest$}  - {}\n", u.form, u.what);
        }
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

    /// A line the table does not hold is no usage at all, and no panic.
    #[test]
    fn a_command_the_table_does_not_hold_says_nothing() {
        say("there-is-no-such-command");
        assert!(row("one\ttwo").is_none(), "a short line is not a row");
    }
}
