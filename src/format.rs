pub enum Format {
    Standard,
    Modern,
    Legacy,
    Vintage,
    Commander,
    Future,
    Pauper,
    Frontier,
    Penny,
    Duel,
    Oldschool,
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Format::*;
        write!(
            f,
            "{}",
            match self {
                Standard => "standard",
                Modern => "modern",
                Legacy => "legacy",
                Vintage => "vintage",
                Commander => "commander",
                Future => "future",
                Pauper => "pauper",
                Frontier => "frontier",
                Penny => "penny",
                Duel => "duel",
                Oldschool => "oldschool",
            }
        )
    }
}
