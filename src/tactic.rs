pub enum Tactic {
    OneMove,
    TwoMoves,
    Test,
}

impl Tactic {
    pub fn to_string(&self) -> String {
        match self {
            Tactic::OneMove => "one_turn".to_string(),
            Tactic::TwoMoves => "two_turns".to_string(),
            Tactic::Test => "test".to_string(),
        }
    }
}