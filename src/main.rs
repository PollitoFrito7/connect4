use std::io;
use std::fmt;
use std::io::Write;
use colored::Colorize;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

fn inbounds(row: usize, col: usize) -> bool {
    row < HEIGHT && col < WIDTH
}

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
            Self::Won(Chip::Red)        => write!(f, "RED WON!"),
            Self::Won(Chip::Yellow)     => write!(f, "YELLOW WON!"),
            Self::Draw                  => write!(f, "IT'S A DRAW!"),
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

        let moves: [(isize, isize); 7] = [(1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)];
        let final_vertical = 0; 
        let final_horizontal = 2;
        let final_diagonal = 4;
        let final_antidiagonal = 6;

        let GameState::Playing(chip) = self.state else {return false};
        let mut row: usize;
        let mut col: usize;
        let mut count: usize = 1;
        for mov in moves {
            for i in 1..=3 {
                row = (mov.0*i + last_chip_height as isize) as usize;
                col = (mov.1*i + column as isize) as usize;

                if !inbounds(row, col) {break;}
                if self.board[row][col] != Some(chip) {break;}   
                
                count += 1;
                if count == 4 {return true}
            }

            if mov == moves[final_horizontal] || mov == moves[final_vertical] 
            || mov == moves[final_diagonal] || mov == moves[final_antidiagonal] {
                    count = 1;
            }
        }

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
