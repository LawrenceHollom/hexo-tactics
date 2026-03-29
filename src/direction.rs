#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    DownRight,
    DownLeft,
    Left,
    UpLeft,
    UpRight,
}

impl Direction {
    pub const ALL: [Direction; 6] = [
        Direction::Right,
        Direction::DownRight,
        Direction::DownLeft,
        Direction::Left,
        Direction::UpLeft,
        Direction::UpRight,
    ];

    pub const POSITIVE: [Direction; 3] = [
        Direction::Right,
        Direction::DownRight,
        Direction::DownLeft,
    ];

    pub fn to_vector(&self) -> (i32, i32) {
        match self {
            Direction::Right => (1, 0),
            Direction::DownRight => (0, 1),
            Direction::DownLeft => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::UpLeft => (0, -1),
            Direction::UpRight => (1, -1),
        }
    }
}