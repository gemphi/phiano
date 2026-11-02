//! The git stamp written when a record page opens (Requirement 2 criterion 5).
//!
//! The stamp names the workspace's commit and each submodule's commit, and for
//! each whether its working tree is clean or dirty; a dirty tree lists the
//! count of modified paths and up to 20 of their names. Everything is read
//! through plain `git` invocations, so no crate is added. A git that cannot be
//! run leaves an honest note on the record rather than crashing the walk — the
//! record is meant to say what it could and could not determine.

use super::{cap, DIRTY_NAMES_CAP};
use std::path::Path;
use std::process::Command;

/// The whole git stamp, as markdown lines: the workspace first, then each
/// submodule the workspace declares, each on its own line with its commit and
/// its clean-or-dirty state. `dir` is the record directory (`phinet/docs/`),
/// whose repository root is the phinet workspace.
pub fn git_stamp(dir: &Path) -> String {
    let root = repo_root(dir).unwrap_or_else(|| dir.to_path_buf());
    let mut lines = vec![one_tree("workspace", &root)];
    for sub in submodules(&root) {
        let path = root.join(&sub);
        lines.push(one_tree(&sub, &path));
    }
    lines.join("\n")
}

/// One tree's line: its name, its short commit, and clean or dirty with the
/// modified count and up to 20 names when dirty (R2.5).
fn one_tree(name: &str, path: &Path) -> String {
    let commit = commit_of(path).unwrap_or_else(|| "unknown".to_string());
    let modified = modified_paths(path);
    if modified.is_empty() {
        format!("- `{name}` {commit} clean")
    } else {
        let shown: Vec<String> =
            modified.iter().take(DIRTY_NAMES_CAP).map(|p| cap(p, 200)).collect();
        format!(
            "- `{name}` {commit} dirty, {} modified: {}",
            modified.len(),
            shown.join(", ")
        )
    }
}

/// The repository root containing `dir`, via `git rev-parse --show-toplevel`.
fn repo_root(dir: &Path) -> Option<std::path::PathBuf> {
    let out = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(dir)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(std::path::PathBuf::from(s))
    }
}

/// The short commit identifier of the tree rooted at `path`.
fn commit_of(path: &Path) -> Option<String> {
    let out = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(path)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

/// The names of the modified paths in the tree rooted at `path`, from
/// `git status --porcelain`. An empty list means a clean tree. A git that
/// cannot be run reports no modification, which `commit_of` already flags by
/// reporting `unknown`.
fn modified_paths(path: &Path) -> Vec<String> {
    let out = match Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(path)
        .output()
    {
        Ok(o) if o.status.success() => o,
        _ => return Vec::new(),
    };
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|l| {
            let rest = l.get(3..).unwrap_or("").trim();
            if rest.is_empty() {
                None
            } else {
                Some(rest.to_string())
            }
        })
        .collect()
}

/// The submodule paths the workspace declares, from `.gitmodules`, in the
/// order they are listed. An absent or unreadable file yields no submodule, so
/// the stamp carries the workspace line alone.
fn submodules(root: &Path) -> Vec<String> {
    let out = match Command::new("git")
        .args(["config", "--file", ".gitmodules", "--get-regexp", "path"])
        .current_dir(root)
        .output()
    {
        Ok(o) if o.status.success() => o,
        _ => return Vec::new(),
    };
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|l| l.split_whitespace().nth(1).map(str::to_string))
        .collect()
}
