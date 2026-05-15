use std::io;
use std::fmt;
use std::io::Write;
use std::u16;
use colored::Colorize;

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
            GameState::Playing(Chip::Red) => GameState::Playing(Chip::Yellow),
            GameState::Playing(Chip::Yellow) => GameState::Playing(Chip::Red),
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



const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(Debug)]
struct Game {
    board: [[Option<Chip>; WIDTH]; HEIGHT],
    state: GameState,
    placed_chips: u16,
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

    fn winning_line(&self) -> bool {
        false
    }

    fn full_board(&self) -> bool {
        self.placed_chips == (WIDTH * HEIGHT) as u16
    }

    fn insert_chip(&mut self, column: usize) -> bool {
        if column >= WIDTH {
            println!("NOT A VALID COLUMN FOR PLAY");
            return false
        }

        match self.state {
            GameState::Playing(chip) =>  {
                let mut lowest_empty = HEIGHT - 1;
                
                // first condition takes care of usize wrapping
                while lowest_empty < HEIGHT && self.board[lowest_empty][column] != None {
                    lowest_empty = lowest_empty.wrapping_sub(1);
                }
                
                if lowest_empty > HEIGHT {
                    println!("COLUMN IS FULL");
                    return false    
                }
                
                self.placed_chips += 1;
                self.board[lowest_empty][column] = Some(chip);
                true
            },
            _ => false
        }
    }

    fn next_state(&mut self) {
        if self.full_board() {
            self.state = GameState::Draw;
            return;
        }

        if self.winning_line() {
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
            self.next_state();
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
        let column: usize = match column.trim().parse() {
            Ok(i) => i,
            Err(_) => continue,
        };
        
        game.make_play(column.wrapping_sub(1)); // accepts machine view
    }

    game.show_game();
}
