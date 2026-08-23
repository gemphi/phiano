use super::*;

fn make_fp(histogram: Vec<f64>) -> Fingerprint {
    let mut dominant: Vec<(u16, f64)> = histogram
        .iter()
        .enumerate()
        .map(|(i, &w)| (i as u16, w))
        .collect();
    dominant.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    dominant.retain(|(_, w)| *w > 0.0);
    Fingerprint {
        sample_count: 1,
        sector_histogram: histogram,
        dominant,
        avg_length: 5.0,
        diversity: 2.0,
    }
}

#[test]
fn test_similarity_identical() {
    let h = vec![0.5, 0.5, 0.0, 0.0];
    let a = make_fp(h.clone());
    let b = make_fp(h);
    assert!((a.similarity(&b) - 1.0).abs() < 1e-10);
}

#[test]
fn test_similarity_orthogonal() {
    let a = make_fp(vec![1.0, 0.0, 0.0, 0.0]);
    let b = make_fp(vec![0.0, 0.0, 1.0, 0.0]);
    assert!(a.similarity(&b).abs() < 1e-10);
}

#[test]
fn test_dominant_sectors() {
    let fp = make_fp(vec![0.1, 0.5, 0.3, 0.1]);
    let top = fp.dominant_sectors(2);
    assert_eq!(top.len(), 2);
    assert_eq!(top[0].0, 1);
    assert_eq!(top[1].0, 2);
}

#[test]
fn test_difference_vector() {
    let a = make_fp(vec![0.5, 0.5, 0.0, 0.0]);
    let b = make_fp(vec![0.0, 0.5, 0.5, 0.0]);
    let diffs = a.difference_vector(&b);
    assert!((diffs[0].1.abs() - 0.5).abs() < 1e-10);
}
