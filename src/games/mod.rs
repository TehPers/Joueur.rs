// for each game
mod chess;
// end for each game

use crate::base::namespace::{BaseNamespace};

pub fn get_game(game_name: &str) -> Option<fn() -> BaseNamespace> {
    // for each game
    if game_name == "chess" {
        return Some(|| chess::make_namespace());
    }
    // end for each game

    None
}
