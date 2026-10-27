#[allow(unused_imports)]
use super::*;

use phiecs::phasor::facet::Facet;
use crate::phasor::SpectralPhasor;
use crate::phasor::{EVIDENCE_WORDS, PACKED_WORDS};
use std::io;
use std::path::Path;

/// Read a helix tree back into a facet: every strand becomes a phasor with
/// both sides and its evidence; the relation table is rebuilt from
/// `codons.tsv`. N-grams are not in the helix and come back empty.
pub fn read(dir: &Path) -> io::Result<Facet> {
    let mut facet = Facet::new();
    let table = std::fs::read_to_string(dir.join("codons.tsv"))?;
    let mut entries = Vec::new();
    for line in table.lines().skip(1) {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 9 {
            continue;
        }
        let k: usize = f[0].parse().unwrap_or(0);
        if k >= 8 && !f[8].starts_with("ch") && !f[8].starts_with("offset") {
            entries.push((f[8].to_string(), k));
        }
    }
    facet.relations = phiecs::relation::RelationVocab::from_entries(entries);
    let strands = dir.join("strands");
    let mut shards: Vec<_> = std::fs::read_dir(&strands)?.filter_map(Result::ok).collect();
    shards.sort_by_key(|e| e.file_name());
    for shard in shards {
        let mut files: Vec<_> = std::fs::read_dir(shard.path())?.filter_map(Result::ok).collect();
        files.sort_by_key(|e| e.file_name());
        for f in files {
            let text = std::fs::read_to_string(f.path())?;
            if let Some((word, p)) = strand(&text) {
                facet.lexicon.insert(word, p);
            }
        }
    }
    Ok(facet)
}

fn strand(text: &str) -> Option<(String, SpectralPhasor)> {
    let mut lines = text.lines();
    let head = lines.next()?.strip_prefix("# ")?;
    let hf: Vec<&str> = head.split('\t').collect();
    let word = hf.first()?.to_string();
    let field = |name: &str| hf.iter().find_map(|s| s.strip_prefix(name)).map(str::trim);
    let amplitude: f64 = field("amplitude ")?.parse().ok()?;
    let band_n: u32 = field("band ")?.parse().ok()?;
    let count: u32 = field("count ")?.parse().ok()?;
    let mut packed = [0u64; PACKED_WORDS];
    let mut tgt = [0u64; PACKED_WORDS];
    let mut evid = [0u64; EVIDENCE_WORDS];
    for line in lines {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 4 {
            continue;
        }
        let k = codon_index(f[0])?;
        let src: u64 = f[1].parse().ok()?;
        let dst: u64 = f[2].parse().ok()?;
        let e: u64 = f[3].parse().ok()?;
        packed[k / 8] |= (src & 0xFF) << (8 * (k % 8));
        tgt[k / 8] |= (dst & 0xFF) << (8 * (k % 8));
        evid[k / 2] |= (e & 0xFFFF_FFFF) << (32 * (k % 2));
    }
    Some((word, SpectralPhasor::from_packed_sides(packed, tgt, evid, amplitude, band_n, count)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LEARNING_RATE;
    use crate::trainer::Trainer;

    #[test]
    fn a_helix_round_trips_the_lexicon() {
        let docs: Vec<Vec<String>> = ["the cat sat on the mat", "a dog eats a donut", "cat of mat"]
            .iter()
            .map(|s| s.split(' ').map(String::from).collect())
            .collect();
        let mut facet = Facet::new();
        facet.learn_relations(&docs);
        for d in &docs {
            for w in d {
                facet.get_or_init(w);
            }
        }
        let t = Trainer::new(LEARNING_RATE);
        for _ in 0..3 {
            for d in &docs {
                t.couple_relations(&mut facet, d);
            }
        }
        let dir = std::env::temp_dir().join(format!("phiano-helix-{}", std::process::id()));
        let n = write(&facet, &dir).unwrap();
        assert_eq!(n, facet.lexicon.len());
        let back = read(&dir).unwrap();
        assert_eq!(back.lexicon.len(), facet.lexicon.len());
        for (w, p) in &facet.lexicon {
            let q = &back.lexicon[w];
            assert_eq!(p.packed(), q.packed(), "{w} source side");
            assert_eq!(p.packed_target(), q.packed_target(), "{w} target side");
            assert_eq!(p.packed_evidence(), q.packed_evidence(), "{w} evidence");
            assert_eq!(p.count, q.count);
        }
        assert_eq!(back.relations.entries(), facet.relations.entries());
        assert!(dir.join("codons").join("AAA").join("words.tsv").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
