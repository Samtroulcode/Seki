//! Board representation and utility functions.

/// Size of the Go board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoardSize {
    N19,
    N13,
    N9,
}

impl BoardSize {
    // mon comm de merde
    pub fn dim(self) -> usize {
        match self {
            BoardSize::N19 => 19,
            BoardSize::N13 => 13,
            BoardSize::N9 => 9,
        }
    }
}

/// Represents the Go board.
pub struct Board {
    size: BoardSize,
    // TODO: use bitboards or Vec for board data
}

impl Board {
    /// Create a new empty board.
    /// First parameter : Size (9x9, 13x13, 19x19)
    pub fn new(size: BoardSize) -> Self {
        Self { size }
    }

    /// Board size accessor.
    pub fn size(&self) -> BoardSize {
        self.size
    }
}
