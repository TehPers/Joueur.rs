// for each game
mod chess;
// end for each game

use crate::base::{GameManager};

pub fn get_game_manager(game_name: &str) -> Option<fn() -> Box<dyn GameManager>> {
    // for each game
    if game_name == "Chess" {
        return Some(|| Box::new(chess::new_game_manager()));
    }
    // end for each game

    None
}
