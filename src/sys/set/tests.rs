use super::*;
use crate::sys::{Being, Persona, Row};

fn being(name: &str, rows: &[(&str, &str, &str)]) -> Being {
    let mut b = Being::new(Persona::from_md(&format!("---\nname: {name}\nseeds: x\nkinds: is-a, part-of\n---\n")).unwrap());
    b.tell(&rows.iter().map(|(a, k, o)| Row::new(a, k, o)).collect::<Vec<_>>());
    b
}

fn pairs(v: &[(&str, &str)]) -> Relation {
    v.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
}

#[test]
fn relations_are_sets_of_pairs() {
    let russell = being("Russell", &[("socrates", "is-a", "man"), ("man", "is-a", "mortal")]);
    let whitehead = being("Whitehead", &[("man", "is-a", "mortal"), ("process", "part-of", "reality")]);
    let (r, w) = (held(&russell, "is-a"), held(&whitehead, "is-a"));
    assert_eq!(union(&r, &w).len(), 2);
    assert_eq!(meet(&r, &w).len(), 1, "they agree that man is mortal");
    let syllogism = compose(&r, &w);
    assert!(syllogism.contains(&("socrates".into(), "mortal".into())), "socrates is-a man ∘ man is-a mortal");
    assert_eq!(converse(&r).iter().next().unwrap(), &("man".to_string(), "socrates".to_string()));
    let t = think(&russell, &whitehead);
    assert_eq!(t.iter().map(|x| x.kind.as_str()).collect::<Vec<_>>(), ["is-a", "part-of"]);
    assert!(render("R", "W", &t).contains("socrates→mortal"));
}

#[test]
fn principia_on_a_chain() {
    let r = pairs(&[("a", "b"), ("b", "c"), ("c", "d")]);
    assert_eq!(domain(&r).len(), 3);
    assert_eq!(range(&r).len(), 3);
    assert_eq!(field(&r).len(), 4);
    assert_eq!(relata(&r, "a"), ["b".to_string()].into_iter().collect());
    assert_eq!(referents(&r, "d"), ["c".to_string()].into_iter().collect());
    assert_eq!(image(&r, &["a".to_string(), "b".to_string()].into_iter().collect()).len(), 2);
    assert_eq!(power(&r, 2), pairs(&[("a", "c"), ("b", "d")]));
    assert_eq!(closure(&r).len(), 6, "a→b,c,d; b→c,d; c→d");
    assert_eq!(ancestral(&r).len(), 10, "plus the four identities");
    assert!(is_transitive(&closure(&r)) && !is_transitive(&r));
    assert!(is_reflexive(&ancestral(&r)) && !is_symmetric(&r));
    let only_a: Class = ["a".to_string()].into_iter().collect();
    assert_eq!(restrict_domain(&r, &only_a), pairs(&[("a", "b")]));
    assert_eq!(restrict_range(&r, &only_a), Relation::new());
    assert_eq!(product(&only_a, &only_a), identity(&only_a));
    assert_eq!(difference(&r, &pairs(&[("a", "b")])).len(), 2);
    assert!(included(&pairs(&[("a", "b")]), &r));
}
