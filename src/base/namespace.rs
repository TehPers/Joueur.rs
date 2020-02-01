use crate::base::game;

pub struct BaseNamespace {
    pub game_name: String,
    pub game_version: String,
    pub player_name: String,
    pub game: Box<dyn game::Game>,
}
