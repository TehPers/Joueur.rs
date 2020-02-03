use crate::base::game;

pub struct Namespace {
    pub game_name: String,
    pub game_version: String,
    pub player_name: String,
    pub game: Box<dyn game::Game>,
}
