/**
 * Represents a position in the hexagonal grid.
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub u: i32,
    pub v: i32,
}

impl Position {
    pub fn new(u: i32, v: i32) -> Position {
        Position { u, v }
    }
}