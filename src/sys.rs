//! The inner layer (docs/160): what every agent in the phinet system is,
//! whoever it is — a persona learned from a quest, a body the kit tells, a
//! device — so the wider agenting system talks to one interface.
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c), grows from it. An agent, then, is a [`Persona`] (its plan:
//! seeds, kinds, voice, team, kudos) and a [`World`] (what it has been told),
//! and it has three moves and no others:
//!
//! - **tell** — rows in, counted at once (`World::tell`);
//! - **watch** — the same move, from a sensor: the kit's rows, a body, a
//!   track, a device; an agent brought into a world of your making learns
//!   what you do by being told what the tracking system saw;
//! - **ask** — what a word calls for under a kind, heaviest first.
//!
//! [`Being`] is the one concrete agent; the [`race`] runs teams of them,
//! the [`circus`] lets two of them talk, and [`sets`] lets two of them
//! think — language as a set game with unions and relations.

mod agent;
mod being;
pub mod circus;
pub mod eye;
pub mod lisp;
pub mod race;
pub mod set;
pub mod trace;

pub use agent::Agent;
pub use being::{find, persona_files, Being};
pub use phiecs::persona::Persona;
pub use phiecs::world::World;
pub use phiecs::Row;
pub use phikit::Kit;
