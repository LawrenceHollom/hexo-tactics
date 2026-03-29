use crate::direction::Direction;

/**
 * Represents a position in the hexagonal grid.
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Position {
    pub u: i32,
    pub v: i32,
}

impl Position {
    pub fn new(u: i32, v: i32) -> Position {
        Position { u, v }
    }

    pub fn offset(&self, dir: Direction, offset: i32) -> Position {
        let (du, dv) = dir.to_vector();
        Position {
            u: self.u + du * offset,
            v: self.v + dv * offset,
        }
    }

    pub fn get_normalised_u(&self, mid: i32) -> usize {
        (self.u + mid) as usize
    }

    pub fn get_normalised_v(&self, mid: i32) -> usize {
        (self.v + mid) as usize
    }
}