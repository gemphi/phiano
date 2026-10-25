//! `arena` — the lineup (docs/176): one reader per model, so one run says
//! which of Phient's models is learning better. Every reader answers the
//! same question at the same position before any of them is told it
//! ([`phiplm::study::Arena`]); the table at the end is the leaderboard.
//!
//! | reader | the model it stands for |
//! |---|---|
//! | [`count::Count`] | phiplm's counts: the word before, nothing else (the line to beat) |
//! | [`concept::Concept`] | phiplm's concepts: the construction family the words said show, backing off onto the counts (docs/174–175) |
//! | [`ecs::Ecs`] | phiecs: the words told as rows into a `World`, read by what the last word calls for |
//! | [`mirror::Mirror`] | phical's conjugate: the relation as an all-pass filter on the circle, read by correlation |

pub mod concept;
pub mod count;
pub mod ecs;
pub mod mirror;
pub mod operator;

pub use concept::Concept;
pub use count::Count;
pub use ecs::Ecs;
pub use mirror::Mirror;
pub use operator::Operator;
