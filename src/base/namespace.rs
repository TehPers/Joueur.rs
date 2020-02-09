use crate::base::game::{Game};

pub trait Namespace {
    // fn get_game_name() -> &'static str;
    fn get_game_version(&self) -> &'static str;
    fn get_player_name(&self) -> &'static str;
    fn get_game<'a>(&'a self) -> &'a dyn Game;
}
