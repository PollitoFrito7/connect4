use crate::chip::Chip;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GameState {
    Playing(Chip),
    Won(Chip),
    Draw,
}

impl GameState {
    pub fn next_turn(&self) -> Self {
        match self {
            GameState::Playing(Chip::Red)       => GameState::Playing(Chip::Yellow),
            GameState::Playing(Chip::Yellow)    => GameState::Playing(Chip::Red),
            other => *other,
        }
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Playing(Chip::Red)    => write!(f, "Red player's turn"),
            Self::Playing(Chip::Yellow) => write!(f, "Yellow player's turn"),
            Self::Won(Chip::Red)        => write!(f, "RED WON!"),
            Self::Won(Chip::Yellow)     => write!(f, "YELLOW WON!"),
            Self::Draw                  => write!(f, "IT'S A DRAW!"),
        }
    }
}