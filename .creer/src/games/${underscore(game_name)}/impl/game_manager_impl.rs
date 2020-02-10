use crate::base::{Game, GameManager};
use crate::games::${underscore(game_name)}::r#impl::{GameImpl};
use crate::games::${underscore(game_name)}::ai::{PLAYER_NAME};

// pub static GAME_NAME: &str = "${game_name}";
pub static GAME_VERSION: &str = "${game_version}";

pub struct GameManagerImpl {
    game: GameImpl,
}

impl GameManagerImpl {
    pub fn new() -> GameManagerImpl {
        return GameManagerImpl{
            game: GameImpl::new(),
        };
    }
}

impl GameManager for GameManagerImpl {
    fn get_game<'a>(&'a self) -> &'a dyn Game {
        return &self.game;
    }

    fn get_game_version(&self) -> &'static str {
        return GAME_VERSION;
    }

    fn get_player_name(&self) -> &'static str {
        return PLAYER_NAME;
    }
}
