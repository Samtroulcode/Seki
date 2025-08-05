//! Game state and rules logic.

use crate::board::{Board, BoardSize};

/// Represents an entire game state.
pub struct State {
    board: Board,
    // TODO: add move history, player to move, etc.
}

impl State {
    /// New game with Japanese rules.
    pub fn new_japanese(size: BoardSize) -> Self {
        Self {
            board: Board::new(size),
        }
    }

    /// Returns a reference to the board.
    pub fn board(&self) -> &Board {
        &self.board
    }
}
