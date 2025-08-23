//! Core Go engine library.
//!
//! Provides types and logic for representing Go boards, moves, and rules.

pub mod board;
pub mod coord;
pub mod state;

pub use crate::board::Board;
pub use crate::coord::Coord;
pub use crate::state::State;
