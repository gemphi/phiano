#[allow(unused_imports)]
use super::*;

/// How many coupling kinds does the text actually support?
///
/// The architecture allows sixty-four. Allowing is not supplying. A channel
/// earns its place only if the pairs the corpus put *there* agree on it more
/// than pairs it never put there — otherwise the channel holds a name and no
/// arrangement, and sixty-four dimensions is a capacity claim rather than a
/// count of anything.
///
/// Per channel: gather the pairs the corpus coupled on it, take their mean
/// agreement on that channel, and compare against the same number of pairs
/// drawn from the same words that were never coupled there. The gap is what
/// the channel knows. The number that clears is how many verticals this text
/// supports, and it is a fact about the text and not about the design.
pub(super) fn coupling_kinds_section(
    rel: &RelationalFacet,
    shipped: &Facet,
    vocab: &RelationVocab,
    toks: &[Vec<String>],
) {
    println!("\n  === how many coupling kinds does the corpus support? ===");
    let mut observed: Vec<Vec<(usize, usize)>> = vec![Vec::new(); PHASE_CHANNELS];
    let mut words: Vec<&String> = Vec::new();
    let mut index: HashMap<&str, usize> = HashMap::new();
    for toks_doc in toks {
        for w in toks_doc {
            if !index.contains_key(w.as_str()) {
                index.insert(w.as_str(), words.len());
                words.push(w);
            }
        }
    }
    for doc in toks {
        for i in 0..doc.len() {
            for j in (i + 1)..(i + 1 + WINDOW).min(doc.len()) {
                if let Some(k) = vocab.channel(doc, i, j) {
                    if observed[k].len() < MAX_PAIRS_PER_CHANNEL {
                        observed[k].push((index[doc[i].as_str()], index[doc[j].as_str()]));
                    }
                }
            }
        }
    }

    let agree = |k: usize, a: usize, b: usize| -> f64 {
        rel.words[words[a]].on(&rel.words[words[b]], k)
    };
    let mut r = 0x2545_F491_4F6C_DD1Du64;
    let mut rng = || {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        r
    };

    let (mut live, mut listed) = (0usize, 0usize);
    let mut rows: Vec<(usize, usize, f64, f64)> = Vec::new();
    for (k, obs) in observed.iter().enumerate() {
        if obs.len() < MIN_PAIRS_FOR_A_KIND {
            continue;
        }
        let n = obs.len();
        let real: f64 = obs.iter().map(|(a, b)| agree(k, *a, *b)).sum::<f64>() / n as f64;
        // The null keeps the channel and the marginal word distribution and
        // destroys only the pairing, which is the thing being tested.
        let null: f64 = obs
            .iter()
            .map(|&(a, _)| {
                let b = obs[(rng() as usize) % n].1;
                agree(k, a, b)
            })
            .sum::<f64>()
            / n as f64;
        rows.push((k, n, real, null));
    }
    rows.sort_by(|a, b| (b.2 - b.3).partial_cmp(&(a.2 - a.3)).unwrap());
    for (k, n, real, null) in &rows {
        let gap = real - null;
        if gap > KIND_MIN_GAP {
            live += 1;
        }
        if listed < 20 {
            println!(
                "  {:<14} {:>8} {:>9.4} {:>9.4} {:>9.4}{}",
                vocab.name(*k),
                n,
                real,
                null,
                gap,
                match gap > KIND_MIN_GAP {
                    true => "",
                    false => "   —",
                }
            );
            listed += 1;
        }
    }
    println!(
        "\n  {} of {} channels carry a coupling the shuffle does not, out of {} \n\
         \x20 that saw enough pairs to test. The rest hold a name and no arrangement.",
        live,
        PHASE_CHANNELS,
        rows.len()
    );

    // And the end-to-end check: the graph draws the edge a reader would.
    // Chosen from words the corpus actually contains. The first version of
    // this line read "the dog eats the donut in the kitchen" and reported
    // only offset couplings, which looked like the marker rule failing and
    // was WikiText-2 not containing `donut` or `eats`.
    let demo = "the president of the state was born in the city";
    let flow = PhaseFlow::build_relational(shipped, rel, vocab, demo);
    let kinds = flow.coupling_kinds();
    println!("\n  \"{demo}\"");
    println!(
        "  {} named couplings: {}",
        kinds.len(),
        kinds
            .iter()
            .map(|(m, c)| format!("{m}(ch{c})"))
            .collect::<Vec<_>>()
            .join(", ")
    );
}
