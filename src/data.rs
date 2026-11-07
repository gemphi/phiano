//! Where phiano's data lives, found from the crate, not from wherever the
//! process was started.
//!
//! Every file the system reads and appends to — lexicons, intents, corpora,
//! the stores it saves — is under the crate's `data/`, or under `$PHIANO_DATA`
//! when that is set. A path written `data/chunks` resolved against the
//! working directory, so a binary run from the workspace root read nothing
//! and wrote its stores into a `data/` of its own; here every such path is
//! resolved once, from the manifest directory, by [`path`].

use std::path::{Path, PathBuf};

/// The data directory: `$PHIANO_DATA`, else the crate's `data/`.
pub fn dir() -> PathBuf {
    match std::env::var_os("PHIANO_DATA") {
        Some(d) => PathBuf::from(d),
        None => Path::new(env!("CARGO_MANIFEST_DIR")).join("data"),
    }
}

/// `rel` under the data directory, as a `String` so every reader and writer
/// that takes `&str` or `AsRef<Path>` can take it. An absolute `rel` is
/// returned as it is.
pub fn path(rel: &str) -> String {
    match Path::new(rel).is_absolute() {
        true => rel.to_string(),
        false => dir().join(rel).to_string_lossy().into_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_data_path_is_under_the_crate_wherever_the_process_runs() {
        let p = path("lex/pos.tsv");
        assert!(Path::new(&p).is_absolute(), "{p}");
        assert!(p.ends_with("pos.tsv") && p.contains("data"), "{p}");
        let absolute = match cfg!(windows) {
            true => "C:/x/y.txt",
            false => "/x/y.txt",
        };
        assert_eq!(path(absolute), absolute);
    }
}
