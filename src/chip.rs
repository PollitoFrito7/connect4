use colored::Colorize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Chip {
    Red,
    Yellow,
}

impl std::fmt::Display for Chip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Red       => write!(f, "{}", "●".red()),
            Self::Yellow    => write!(f, "{}", "●".yellow()),
        }
    }
}