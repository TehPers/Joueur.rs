use crate::base::game::{Game};

pub trait Namespace<'a> {
    // fn get_game_name() -> &'static str;
    fn get_game_version(&self) -> &'static str;
    fn get_player_name(&self) -> &'static str;
    fn get_game(&self) -> &'a dyn Game;
}
