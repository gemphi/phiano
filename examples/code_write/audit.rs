//! Auditing the checks (docs/119): a check is only worth something if a
//! look-alike body fails it. Every learned answer whose signature has the
//! same shape as a task's is said again under the task's names (as family
//! resemblance would write it) and run against the task's check; a pass by
//! another family's body is a coincidence in the check, and is listed.

use super::analog;
use super::checks;
use super::tasks::Task;
use super::write::render;

/// Families that are the same function under two names: a pass is no coincidence.
const SAME: [(&str, &str); 2] = [("sum_to", "triangle"), ("product_to", "factorial")];

pub fn run(fns: &[Task], refs: &[(String, String, Vec<String>)]) -> String {
    let mut out = String::from("| task | passed by the body of | \n|---|---|\n");
    let mut n = 0;
    for t in fns {
        let toks = super::lex::tokens(&format!("{} {{}}", t.signature)).unwrap_or_default();
        let sig = toks[..toks.len() - 2].to_vec();
        for (fam, _, body) in refs.iter().filter(|(f, _, _)| *f != t.family && !SAME.iter().any(|(a, b)| (a == f && *b == t.family) || (b == f && *a == t.family))) {
            let one = [analog::answer(body)];
            for b in analog::write(&sig, &one, 1) {
                let text = render(&[sig.clone(), b].concat());
                if checks::works(t, &text) == Some(true) {
                    out += &format!("| {} | {fam} |\n", t.name);
                    n += 1;
                }
            }
        }
    }
    format!("{n} coincidences\n\n{out}")
}
