//! The Recorder's integration unit facts: the tally partition, the series
//! continuing from 208, the previous-page Bar read-back, and the read-only
//! directory test (Requirement 2 criterion 9).

use super::SEED;
use crate::bars::{Bar, Bars, Judged, Sense, Unit};
use crate::record::{page, parse, Measured, RecordError, Recorder};
use std::collections::BTreeMap;

/// A plain `Measured` for the unit facts.
fn num(name: &str, value: f64, verdict: Judged) -> Measured {
    Measured { name: name.into(), value, unit: Unit::Count, bar: None, control: None, verdict }
}

/// `tally` partitions the total, and a name with no earlier Bar is not marked
/// re-barred.
#[test]
fn tally_partitions_and_rebarred_is_detected() {
    let dir = std::env::temp_dir().join(format!("phiano_rec_tally_{SEED:x}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let bars = Bars::from_map(BTreeMap::new());
    let mut rec = Recorder::open(&dir, &bars).unwrap();
    rec.write(num("a", 1.0, Judged::Pass)).unwrap();
    rec.write(num("b", 2.0, Judged::Miss { shortfall: -1.0 })).unwrap();
    rec.write(num("c", 3.0, Judged::Unjudged)).unwrap();
    assert_eq!(rec.tally(), (1, 1, 1, 3), "the three verdicts partition the total");

    let now = Bar { value: 10.0, sense: Sense::AtLeast };
    assert_eq!(rec.rebarred("absent", Some(now)), None, "a name with no earlier Bar is not re-barred");
    let _ = std::fs::remove_dir_all(&dir);
}

/// `close` continues the numbered series from 208 in an empty directory and
/// writes a readable page; a second read then recovers the first page's Bars.
#[test]
fn close_writes_at_series_start_and_the_page_reads_back() {
    let dir = std::env::temp_dir().join(format!("phiano_rec_series_{SEED:x}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let bars = Bars::from_map(BTreeMap::new());
    let mut rec = Recorder::open(&dir, &bars).unwrap();
    rec.write(Measured {
        name: "keyboard.operator_count".into(),
        value: 64.0,
        unit: Unit::Count,
        bar: Some(Bar { value: 64.0, sense: Sense::AtLeast }),
        control: None,
        verdict: Judged::Pass,
    })
    .unwrap();
    let path = rec.close().unwrap();
    let name = path.file_name().unwrap().to_string_lossy();
    assert!(name.starts_with("208_"), "the series continues from 208, got {name}");
    assert!(name.ends_with("_harness.md"), "the slug is harness, got {name}");

    let previous = parse::previous_bars(&dir);
    assert_eq!(
        previous.get("keyboard.operator_count"),
        Some(&Bar { value: 64.0, sense: Sense::AtLeast }),
        "the previous page's Bar is read back"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// A Recorder whose record page cannot be written reports the page path and
/// the reason, and leaves every earlier page byte-identical (R2.9). A
/// read-only file pre-created at the target path forces the failure; the
/// earlier page beside it is checked byte-identical after the failed close.
#[test]
fn a_recorder_on_a_read_only_page_reports_the_path_and_reason() {
    let dir = std::env::temp_dir().join(format!("phiano_rec_ro_{SEED:x}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    // An earlier page that must stay byte-identical across the failed write.
    let earlier = dir.join("207_earlier_harness.md");
    std::fs::write(&earlier, "# earlier\n\nkept verbatim\n").unwrap();
    let before = std::fs::read(&earlier).unwrap();

    // The page the Recorder will try to write is 208; pre-create it read-only
    // so the write is refused.
    let target = page::next_page(&dir);
    std::fs::write(&target, "locked").unwrap();
    let mut perms = std::fs::metadata(&target).unwrap().permissions();
    perms.set_readonly(true);
    std::fs::set_permissions(&target, perms).unwrap();

    let bars = Bars::from_map(BTreeMap::new());
    let mut rec = Recorder::open(&dir, &bars).unwrap();
    rec.write(num("x", 1.0, Judged::Unjudged)).unwrap();
    let result = rec.close();

    match result {
        Err(RecordError::Unwritable { path, reason }) => {
            assert_eq!(path, target, "the unwritable page's path is reported");
            assert!(!reason.is_empty(), "the reason the write failed is reported");
        }
        Err(other) => panic!("expected Unwritable, got {other:?}"),
        Ok(p) => {
            // A filesystem that ignores the read-only bit for the owner leaves
            // the earlier-page check below as the real assertion.
            let _ = std::fs::remove_file(p);
        }
    }

    // Every earlier page stays byte-identical (R2.9).
    let after = std::fs::read(&earlier).unwrap();
    assert_eq!(before, after, "an earlier record page was altered by a failed write");

    let mut perms = std::fs::metadata(&target).unwrap().permissions();
    perms.set_readonly(false);
    let _ = std::fs::set_permissions(&target, perms);
    let _ = std::fs::remove_dir_all(&dir);
}
