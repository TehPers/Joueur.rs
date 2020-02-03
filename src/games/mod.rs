// for each game
mod chess;
// end for each game

use crate::base::{Namespace};

pub fn get_game(game_name: &str) -> Option<fn() -> Namespace> {
    // for each game
    if game_name == "Chess" {
        return Some(|| chess::make_namespace());
    }
    // end for each game

    None
}
