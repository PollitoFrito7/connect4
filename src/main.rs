use std::io;
use std::fmt;
use std::io::Write;
use colored::Colorize;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Chip {
    Red,
    Yellow,
}

impl fmt::Display for Chip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Red       => write!(f, "{}", "●".red()),
            Self::Yellow    => write!(f, "{}", "●".yellow()),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum GameState {
    Playing(Chip),
    Won(Chip),
    Draw,
}

impl GameState {
    fn next_turn(&self) -> Self {
        match self {
            GameState::Playing(Chip::Red)       => GameState::Playing(Chip::Yellow),
            GameState::Playing(Chip::Yellow)    => GameState::Playing(Chip::Red),
            other => *other,
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Playing(Chip::Red)    => write!(f, "Red player's turn"),
            Self::Playing(Chip::Yellow) => write!(f, "Yellow player's turn"),
            _                           => write!(f, "GAME FINISHED!"),
        }
    }
}

#[derive(Debug)]
struct Game {
    board: [[Option<Chip>; WIDTH]; HEIGHT],
    state: GameState,
    placed_chips: usize,
}

impl Game {
    fn new() -> Self {
        Self {
            board: [[None; WIDTH];HEIGHT],
            state: GameState::Playing(Chip::Red),
            placed_chips: 0,
        }
    }

    fn show_game(&self) {
        println!();
        println!("{}", self);
        println!();
    }

    fn winning_line(&self, column: usize) -> bool {
        // 1. locate the last played chip
        let mut last_chip_height: usize = HEIGHT - 1;

        for i in 0..HEIGHT {
            if self.board[i][column].is_some() {
                last_chip_height = i;
                break;
            }          
        }
        // 2. expand the chip in all directions until other color chip or line of 4
        let mut line_counter: usize = 0;
        let GameState::Playing(chip) = self.state else {
            return false
        };

        for i in (last_chip_height)..HEIGHT {
            if self.board[i][column] != Some(chip) {
                break;
            }
            
            line_counter += 1;

            if line_counter == 4 {
                return true
            }
        }

        line_counter = 1;
        
        for i in (0..(column)).rev() {
            if self.board[last_chip_height][i] != Some(chip) {
                break;
            }

            line_counter += 1;

            if line_counter == 4 {
                return true
            }
        }

        for i in (column + 1)..WIDTH {
            if self.board[last_chip_height][i] != Some(chip) {
                break;
            }

            line_counter += 1;

            if line_counter == 4 {
                return true
            }
        }

        line_counter = 1;

        //TODO: handle diagonals

        false
    }

    fn full_board(&self) -> bool {
        self.placed_chips == WIDTH * HEIGHT
    }

    fn insert_chip(&mut self, column: usize) -> bool {
        if column >= WIDTH {
            println!("NOT A VALID COLUMN FOR PLAY");
            return false
        }

        match self.state {
            GameState::Playing(chip) =>  {                
                for i in (0..HEIGHT).rev() {
                    if self.board[i][column].is_none() {
                        self.placed_chips += 1;
                        self.board[i][column] = Some(chip);
                        return true
                    }
                }
                
                println!("COLUMN IS FULL");
                false    
            },
            _ => false
        }
    }

    fn next_state(&mut self, column: usize) {
        if self.full_board() {
            self.state = GameState::Draw;
            return;
        }

        if self.winning_line(column) {
            self.state = match self.state {
                GameState::Playing(chip) => GameState::Won(chip),
                other => other,
            };
            return;
        }

        self.state = self.state.next_turn();
    }

    fn make_play(&mut self, column: usize) {
        if self.insert_chip(column) {
            self.next_state(column);
        }
    }

}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.state)?;
        writeln!(f, "------------------------")?;
        for i in 0..HEIGHT {
            write!(f, "{}|", HEIGHT - i)?;

            for j in 0..WIDTH {
                match self.board[i][j] {
                    None              => write!(f, " ○ ")?,
                    Some(chip)  => write!(f, " {chip} ")?,
                }
            }
            writeln!(f, "|")?;
        }

        write!(f, " |")?;

        for _ in 0..WIDTH*3 {
            write!(f, "=")?;
        }

        writeln!(f, "|")?;

        write!(f, "  ")?;
        for i in 0..WIDTH {
            write!(f, " {} ", i + 1)?;
        }

        Ok(())
    }
}


fn main() {
    let mut game: Game = Game::new();
    let mut column: String = String::new();
    
    while matches!(game.state, GameState::Playing(_)) {     
        game.show_game();
        column.clear();

        print!("Choose a column to place your chip: ");
        io::stdout().flush().expect("Failed to flush");
        io::stdin()
            .read_line(&mut column)
            .expect("Failed to read line.");
        println!();

        // human view
        let mut column: usize = match column.trim().parse() {
            Ok(i) => i,
            Err(_) => continue,
        };

        if column != 0 {
            column = column - 1;
        }
        
        game.make_play(column); // accepts machine view
    }

    game.show_game();
}
