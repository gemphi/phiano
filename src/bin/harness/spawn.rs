//! The capped spawn: one `cargo` child at a time, below normal priority,
//! killed at a 900-second cap through Windows' own `taskkill /T /F /PID`.
//!
//! Levels run one at a time in one thread (R1.5), so at most one compilation
//! is ever in flight. `child.kill()` does not reach cargo's grandchildren
//! (rustc, the test binary), so the kill goes through the OS tree-killer.
//!
//! `cargo test -p <crate> --no-run` runs first so a build failure (R1.9) is
//! distinguished from a test failure (R1.3); the test run passes
//! `--no-fail-fast` so every failing test surfaces. libtest's summary lines
//! are parsed so the counts passed and failed sum to the count run (R1.2).
#![allow(dead_code)]

use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// BELOW_NORMAL_PRIORITY_CLASS (R1.5).
pub const BELOW_NORMAL: u32 = 0x0000_4000;

/// The 900-second cap of R1.6.
pub const CAP_SECONDS: u64 = 900;

/// At most this many characters of any recorded message (R1.3, R1.9).
pub const MESSAGE_CAP: usize = 4_000;

/// The outcome of one capped child run.
#[derive(Clone, Debug, PartialEq)]
pub enum Capped {
    /// The child exited on its own before the cap.
    Finished { ok: bool, out: String, err: String },
    /// The cap fired; the process tree was killed.
    Killed,
}

/// The counts parsed from libtest's one summary line (R1.2).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub run: usize,
    pub passed: usize,
    pub failed: usize,
    pub ignored: usize,
    pub measured: usize,
    pub filtered: usize,
}

impl Counts {
    /// The invariant R1.2 requires: passed + failed == run.
    pub fn conserves(&self) -> bool {
        self.passed + self.failed == self.run
    }
}

/// One failing test, recorded with its name and its capped message (R1.3).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failing {
    pub name: String,
    pub message: String,
}

/// Truncate to the 4,000-character cap, taking a prefix of the original.
pub fn cap(s: &str) -> String {
    s.chars().take(MESSAGE_CAP).collect()
}

/// `creation_flags(BELOW_NORMAL)` on Windows; a no-op elsewhere so the file
/// still compiles for a non-Windows reader of the tests.
fn spawn_below_normal(args: &[&str]) -> std::io::Result<Child> {
    let mut cmd = Command::new("cargo");
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(BELOW_NORMAL);
    }
    cmd.spawn()
}

/// Kill the whole process tree. `taskkill /T /F /PID` reaches cargo's
/// grandchildren (rustc, the test binary) that `child.kill()` cannot.
fn kill_tree(pid: u32) {
    #[cfg(windows)]
    {
        let _ = Command::new("taskkill")
            .args(["/T", "/F", "/PID", &pid.to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    #[cfg(not(windows))]
    {
        let _ = pid;
    }
}

/// Drain a child's stdout and stderr on their own threads so a full pipe
/// buffer cannot deadlock the wait.
fn drain(child: &mut Child) -> (mpsc::Receiver<String>, mpsc::Receiver<String>) {
    let mut so = child.stdout.take();
    let mut se = child.stderr.take();
    let (ts, ro) = mpsc::channel();
    let (te, re) = mpsc::channel();
    thread::spawn(move || {
        let mut s = String::new();
        if let Some(h) = so.as_mut() {
            let _ = h.read_to_string(&mut s);
        }
        let _ = ts.send(s);
    });
    thread::spawn(move || {
        let mut s = String::new();
        if let Some(h) = se.as_mut() {
            let _ = h.read_to_string(&mut s);
        }
        let _ = te.send(s);
    });
    (ro, re)
}

/// Run one `cargo` invocation below normal priority, killing the whole tree
/// if it outlasts `seconds`. One child at a time in one thread (R1.5).
pub fn run_capped(args: &[&str], seconds: u64) -> std::io::Result<Capped> {
    let mut child = spawn_below_normal(args)?;
    let pid = child.id();
    let (ro, re) = drain(&mut child);

    let (done_tx, done_rx) = mpsc::channel();
    let waiter = thread::spawn(move || {
        let status = child.wait();
        let _ = done_tx.send(status);
    });

    match done_rx.recv_timeout(Duration::from_secs(seconds)) {
        Ok(status) => {
            let _ = waiter.join();
            let ok = status.map(|s| s.success()).unwrap_or(false);
            let out = ro.recv().unwrap_or_default();
            let err = re.recv().unwrap_or_default();
            Ok(Capped::Finished { ok, out, err })
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
            kill_tree(pid);
            let _ = waiter.join();
            Ok(Capped::Killed)
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            let _ = waiter.join();
            Ok(Capped::Killed)
        }
    }
}

/// The first `error:` line of a build, truncated to the cap (R1.9). `None`
/// when no such line appears.
pub fn first_build_error(stderr: &str) -> Option<String> {
    stderr
        .lines()
        .find(|l| l.trim_start().starts_with("error"))
        .map(cap)
}

/// Parse libtest's summary line, e.g.
/// `test result: ok. 12 passed; 0 failed; 2 ignored; 0 measured; 3 filtered out`.
/// Counts may arrive in any order and are tolerant of the `; finished in`
/// suffix cargo appends. Returns `None` when no summary line is present.
pub fn parse_counts(output: &str) -> Option<Counts> {
    let line = output.lines().rev().find(|l| l.contains("test result:"))?;
    let tail = line.split("test result:").nth(1)?;
    let mut c = Counts::default();
    let mut saw = false;
    // Each part looks like "<n> passed" or "<n> filtered out", but the first
    // part also carries the result verb ("ok." or "FAILED."), so scan for the
    // number immediately followed by its kind rather than assuming position.
    for part in tail.split(';') {
        let toks: Vec<&str> = part.split_whitespace().collect();
        for w in toks.windows(2) {
            let n: usize = match w[0].trim_matches('.').parse() {
                Ok(n) => n,
                Err(_) => continue,
            };
            match w[1] {
                "passed" => {
                    c.passed = n;
                    saw = true;
                }
                "failed" => {
                    c.failed = n;
                    saw = true;
                }
                "ignored" => c.ignored = n,
                "measured" => c.measured = n,
                "filtered" => c.filtered = n,
                _ => {}
            }
        }
    }
    if !saw {
        return None;
    }
    c.run = c.passed + c.failed;
    Some(c)
}

/// Collect every failing test's name from libtest's `failures:` block. Each
/// name is a line indented under the block and reappears in the summary.
pub fn failing_names(output: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut in_block = false;
    for line in output.lines() {
        let t = line.trim();
        if t == "failures:" {
            // Two such headers appear; the second lists plain names.
            in_block = true;
            continue;
        }
        if in_block {
            if t.is_empty() || t.starts_with("test result:") {
                in_block = false;
            } else if !t.contains(' ') && !t.starts_with("----") {
                names.push(t.to_string());
            }
        }
    }
    names.sort();
    names.dedup();
    names
}

/// The wall-clock a level took, to one decimal place (R1.2). A killed level
/// records the cap exactly (R1.6).
pub fn elapsed_or_cap(started: Instant, killed: bool, cap_seconds: u64) -> f64 {
    if killed {
        cap_seconds as f64
    } else {
        (started.elapsed().as_secs_f64() * 10.0).round() / 10.0
    }
}

#[cfg(test)]
#[path = "spawn/tests.rs"]
mod tests;
