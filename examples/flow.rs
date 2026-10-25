//! A river flow through the generator: one prompt, three sectors, the text.
use phiano::compose::flow::RiverFlow;
use phiano::facet::Facet;
use phiano::storage::Storage;
fn main() {
    let facet: Facet = Storage::load("data/austen_voice.chroma").expect("lexicon");
    let t0 = std::time::Instant::now();
    for s in [None, Some(3u16), Some(40u16)] {
        let f = RiverFlow::trace(&facet, "the morning was", s, 4);
        println!("sector {:>2}: {}", f.source_sector, f.text);
    }
    println!("{:.1} s for three flows", t0.elapsed().as_secs_f64());
}
