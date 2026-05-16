use crate::game_state::GameState;
use crate::chip::Chip;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

fn inbounds(row: isize, col: isize) -> bool {
    row >= 0
        && row < HEIGHT as isize
        && col >= 0
        && col < WIDTH as isize
}

#[derive(Debug)]
pub struct Game {
    board: [[Option<Chip>; WIDTH]; HEIGHT],
    state: GameState,
    placed_chips: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [[None; WIDTH];HEIGHT],
            state: GameState::Playing(Chip::Red),
            placed_chips: 0,
        }
    }

    pub fn is_playing(&self) -> bool {
        matches!(self.state, GameState::Playing(_))
    }

    pub fn show_game(&self) {
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
        let mut row: isize;
        let mut col: isize;
        let mut count: usize = 1;
        for mov in moves {
            for i in 1..=3 {
                row = mov.0*i + last_chip_height as isize;
                col = mov.1*i + column as isize;

                if !inbounds(row, col) {break;}

                let row = row as usize;
                let col = col as usize;

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

    pub fn make_play(&mut self, column: usize) {
        if self.insert_chip(column) {
            self.next_state(column);
        }
    }

}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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