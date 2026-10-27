//! `arena` — phiano's own reader for the arena (docs/176): the words told as
//! rows into a persona's `World`, read by what the last word calls for. The
//! other readers — counts, concepts, the mirror, the operator and the
//! learner — need nothing of phiano's and live in [`phient::lineup`].
//!
//! | reader | the model it stands for |
//! |---|---|
//! | [`ecs::Ecs`] | phiecs: the words told as rows into a `World`, read by what the last word calls for |

pub mod ecs;

pub use ecs::Ecs;
