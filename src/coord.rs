// src/coord.rs

//! Goban coordinates (A1, T19, etc.).

/// Represents a coordinate on the goban (board).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

impl Coord {
    /// Create a new coordinate.
    ///
    /// # Examples
    /// ```
    /// use seki::Coord;
    /// let c = Coord::new(3, 4);
    /// assert_eq!(c.x, 3);
    /// ```
    pub const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}
