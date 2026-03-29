pub enum Tactic {
    OneMove,
    TwoMoves,
    ThreeMoves,
    Test,
}

impl Tactic {
    pub fn to_string(&self) -> String {
        match self {
            Tactic::OneMove => "one_turn".to_string(),
            Tactic::TwoMoves => "two_turns".to_string(),
            Tactic::ThreeMoves => "three_turns".to_string(),
            Tactic::Test => "test".to_string(),
        }
    }
}