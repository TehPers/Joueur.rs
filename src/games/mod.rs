// for each game
mod chess;
// end for each game

use crate::base::{Namespace};

pub fn get_game(game_name: &str) -> Option<fn() -> Box<dyn Namespace>> {
    // for each game
    if game_name == "Chess" {
        return Some(|| Box::new(chess::make_namespace()));
    }
    // end for each game

    None
}
