#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    Yellow,
    Blue
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::Yellow => Player::Blue,
            Player::Blue => Player::Yellow,
        }
    }
}