mod ai;
mod game;

use crate::base::{Namespace};

pub fn make_namespace() -> Namespace {
    return Namespace{
        game_name: "Chess".to_string(),
        game_version: "cfa5f5c1685087ce2899229c04c26e39f231e897ecc8fe036b44bc22103ef801".to_string(),
        player_name: ai::PLAYER_NAME.to_string(),
        game: Box::new(game::ChessGameImpl::new()),
    };
}
