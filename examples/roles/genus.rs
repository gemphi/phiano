#[allow(unused_imports)]
use super::*;

/// Genus recovery on the hypernym family: role query against the bag control.
///
/// The hypernym family is the genus relation, so it is the family this can
/// be scored on without inventing a probe set.
pub(super) fn genus_experiment(
    facet: &Facet,
    rules: &RuleSet,
    fillers_of: &HashMap<String, Vec<String>>,
) {
    let families = RelationBenchmark::default_families();
    let hyper = families
        .iter()
        .find(|f| f.name == "hypernym")
        .expect("hypernym family present");

    let mut usable = 0usize;
    let (mut role_top1, mut role_top5) = (0usize, 0usize);
    let (mut bag_top1, mut bag_top5) = (0usize, 0usize);
    let mut examples: Vec<String> = Vec::new();

    for pair in &hyper.pairs {
        let (specific, general) = (&pair.a, &pair.b);
        if !facet.lexicon.contains_key(specific) || !facet.lexicon.contains_key(general) {
            continue;
        }
        let fillers = match fillers_of.get(specific) {
            Some(f) if !f.is_empty() => f.clone(),
            _ => continue,
        };
        usable += 1;

        let id = match Roles::identity(facet, rules, specific) {
            Some(i) => i,
            None => continue,
        };
        let by_role: Vec<String> = Roles::query(facet, &id, Role::Genus, &[specific], 5)
            .into_iter()
            .map(|(w, _)| w)
            .collect();

        let bag = match bag_identity(facet, &fillers) {
            Some(b) => b,
            None => continue,
        };
        let by_bag = nearest(facet, &bag, &[specific], 5);

        if by_role.first().is_some_and(|w| w == general) {
            role_top1 += 1;
        }
        if by_role.iter().any(|w| w == general) {
            role_top5 += 1;
        }
        if by_bag.first().is_some_and(|w| w == general) {
            bag_top1 += 1;
        }
        if by_bag.iter().any(|w| w == general) {
            bag_top5 += 1;
        }

        if examples.len() < 8 {
            examples.push(format!(
                "  {:<10} → genus? role: {:<28} bag: {:<28} (true: {})",
                specific,
                by_role.iter().take(3).cloned().collect::<Vec<_>>().join(", "),
                by_bag.iter().take(3).cloned().collect::<Vec<_>>().join(", "),
                general
            ));
        }
    }

    let n = usable.max(1) as f64;
    println!("\n=== genus recovery on {} hypernym pairs ===", usable);
    println!("  {:<14} {:>9} {:>9}", "scorer", "top-1", "top-5");
    println!(
        "  {:<14} {:>8.1}% {:>8.1}%",
        "role query",
        role_top1 as f64 / n * 100.0,
        role_top5 as f64 / n * 100.0
    );
    println!(
        "  {:<14} {:>8.1}% {:>8.1}%",
        "bag (control)",
        bag_top1 as f64 / n * 100.0,
        bag_top5 as f64 / n * 100.0
    );
    println!(
        "  {:<14} {:>8.4}% {:>8.4}%",
        "chance",
        100.0 / facet.vocabulary_size() as f64,
        500.0 / facet.vocabulary_size() as f64
    );

    println!("\n--- what came back ---");
    for e in &examples {
        println!("{}", e);
    }

    println!(
        "\n  VERDICT: role binding {} the bag on top-5 ({:+.1} pp).",
        match role_top5 > bag_top5 {
            true => "BEATS",
            false => "does NOT beat",
        },
        (role_top5 as f64 - bag_top5 as f64) / n * 100.0
    );
    println!(
        "  A tie would mean the rotation is decoration: the bag has the same\n\
         \x20 fillers and only lacks the roles, so only a gap is evidence that\n\
         \x20 typing the relation carries information."
    );
}
