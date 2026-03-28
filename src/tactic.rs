pub enum Tactic {
    OneMove,
    Test,
}

impl Tactic {
    pub fn to_string(&self) -> String {
        match self {
            Tactic::OneMove => "one_move".to_string(),
            Tactic::Test => "test".to_string(),
        }
    }
}