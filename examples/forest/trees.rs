//! The sixteen trees the system already holds (docs/155), and where a
//! bounding-volume hierarchy sits among them.
//!
//! A BVH is a tree whose node is a *volume* holding its children's volumes,
//! and whose query is a ray tested against a node and, if it hits, against
//! its children — never against the whole scene. Phient's trees are that
//! shape with the volume read differently in each: a node holds a
//! distribution, a child's is narrower than its parent's, and the ray is a
//! few words asking which node says them (`relate::tree::cast`). Each entry
//! names the node, what makes a child, where it is built, and what plays
//! the part of the ray — with a count measured on this story where one is
//! defined.

use serde_json::{json, Value};

/// `(name, node, child, ray, code, bvh)` for each of the sixteen.
const TREES: [[&str; 6]; 16] = [
    [
        "1 · the scope tree",
        "a scope: a text, a chapter, a paragraph — a word too, since a kind and a condition are words",
        "told by a `within` row; a scope says everything its subtree says",
        "a few words and their pairs, cast from the roots down, entering only children that say them more than the whole tree does",
        "phiplm::relate::tree (ScopeTree::cast_under, Cast)",
        "the one the BVH analogy is written for: the volume is the scope's distribution over what it says, the hit test is D4's movement in nats, the pruning is the beam",
    ],
    [
        "2 · the text tree",
        "a directory, a `.txt` file, a blank-line paragraph",
        "the file system's own nesting; a paragraph is a leaf",
        "reading a text builds it; nothing is queried, it is the tree the scope tree is told from",
        "phiplm::prose::read_tree / for_each_paragraph",
        "the scene graph the volumes are fitted to — the authored hierarchy before any query",
    ],
    [
        "3 · the strand's cut tree",
        "a span of tokens, `start..end`",
        "the division that gains the most nats against the parent's distribution, taken first",
        "none: the tree is built once, then read at sixty-four resolutions",
        "phiplm::prose::strand::cut (Cut::divide, Span)",
        "the BVH *builder*: a top-down split that minimises a cost, the way SAH splits a scene",
    ],
    [
        "4 · the sixty-four levels",
        "a unit of one level, `AAA` (the word) to `TTT` (the whole story)",
        "the cut tree pruned to `m(k)` units, the counts spaced geometrically",
        "a word asked at a level reads the unit holding it",
        "phiplm::prose::strand (Strand::units, level, unit_of)",
        "the BVH's levels of detail: one tree read at the resolution the query can afford",
    ],
    [
        "5 · the reading path",
        "a unit on the path from the word up to the whole story",
        "the unit's parent in the cut tree",
        "the word being read; its ancestors are mixed, weights counted for it",
        "phiplm::prose::strand::path (Path, Scopes::path)",
        "the node stack a ray traversal keeps — here the whole stack is read, not just the leaf",
    ],
    [
        "6 · the tree of knowing's levels",
        "a condition: a scope, the last word, the last pair, or a word under a scope",
        "finer than its parent; the path's scopes, then the word, then the pair",
        "the words said so far; what the levels give is the next word's distribution",
        "phiplm::relate::levels, relate::path (Weights::Path)",
        "the traversal's result turned into an answer: every node on the path contributes, by a counted weight",
    ],
    [
        "7 · the condition tree (X ⇒ Y under c)",
        "a word, and a condition which is itself a word",
        "a condition told under another condition — the recursion the paradigm names",
        "a word plus the condition it is read under",
        "phiecs::condition::Under, World::calls_for_under",
        "nested volumes with no geometry: the enclosure is `told under`, not `contains`",
    ],
    [
        "8 · the quest's walk",
        "a word asked of the oracle",
        "a word the answer named, at depth + 1",
        "the frontier: the quest walks outward from the seed",
        "phiecs::quest (Quest::next_word, push, DEFAULT_DEPTH)",
        "the build queue of a BVH built lazily: nodes exist once something asks for them",
    ],
    [
        "9 · the code tree",
        "a crate, a module, a function, a token of its body",
        "the compiler's own nesting, read as `within` rows",
        "a function's words, cast to find where a new function belongs",
        "phiplm::code",
        "the same scope tree over a different text; the grammar fits the volumes exactly",
    ],
    [
        "10 · the kin tree",
        "a word",
        "not nesting but neighbourhood: the words with the closest left and right company",
        "a word never told after the last one; its kin answer in its place",
        "phiplm::relate::kin (Counts::kin, kin_base)",
        "the fallback when a ray misses every volume: ask what lies near instead",
    ],
    [
        "11 · the phasor's channels",
        "a word",
        "64 relation channels, each an angle and a count",
        "two words compared on a channel (agreement) or over all of them (resonance)",
        "phiecs::phasor::Phasor, phical::CHANNELS",
        "the node's *bounds*, if a BVH stored orientation instead of extent: a word has no position, only angles",
    ],
    [
        "12 · the helix",
        "a directory of the facet's tree, coded as a codon",
        "the channel's place on the helix: 64 codons, three bases of four",
        "`ls`, `grep`, `cat` — the file system is the query language",
        "phiano::helix",
        "a BVH written to disk: the path names the node, so a lookup is a path walk",
    ],
    [
        "13 · the sphere's degrees",
        "a degree `l` of the spherical harmonics",
        "its `2l+1` orders, tied to one weight",
        "a word read at a lower degree is the word at a coarser resolution",
        "phifno::sphere, phical::Tie::{Sphere, Anisotropic}",
        "mip levels: one object at many resolutions, the coarse level a bound on the fine one",
    ],
    [
        "14 · the operator's tape",
        "a value on the autograd tape",
        "the operation that made it, with its inputs",
        "the backward pass walks it from the loss",
        "phistd::tensor (Tape, Op, backward)",
        "not a BVH but the same discipline: a node knows its children, and the walk visits each once",
    ],
    [
        "15 · the Fourier modes",
        "a mode of the operator's spectral weight",
        "the modes kept below the cutoff, the rest zeroed by the mask",
        "a function passed through: low modes carry it, high modes must earn their place",
        "phical::operator (Fno, modes, mask_modes)",
        "the coarse bound again, in frequency: the low modes are the volume the detail sits inside",
    ],
    [
        "16 · the forest",
        "a whole told text with its own root",
        "the roots standing beside each other under no parent",
        "a story cast into every root at once; the one that says it most holds it",
        "phiplm::relate::tree (ScopeTree::roots), this binary",
        "the top-level acceleration structure over several scenes — a BVH of BVHs, which is what this story in Emma is",
    ],
];

/// The sixteen with what the story measured for each, where that is defined.
pub fn trees(measured: &[(usize, Value)]) -> Vec<Value> {
    TREES
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let m = measured.iter().find(|(j, _)| *j == i + 1).map(|(_, v)| v.clone());
            json!({ "name": t[0], "node": t[1], "child": t[2], "ray": t[3], "code": t[4], "bvh": t[5], "measured": m })
        })
        .collect()
}
