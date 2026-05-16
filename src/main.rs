mod chip;
mod game_state;
mod game;

use std::io;
use std::io::Write;
use crate::game::Game;

fn main() {
    let mut game: Game = Game::new();
    let mut column: String = String::new();
    
    while game.is_playing() {     
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
