pub enum Tactic {
    OneMove,
    TwoMoves,
    Test,
}

impl Tactic {
    pub fn to_string(&self) -> String {
        match self {
            Tactic::OneMove => "one_move".to_string(),
            Tactic::TwoMoves => "two_moves".to_string(),
            Tactic::Test => "test".to_string(),
        }
    }
}