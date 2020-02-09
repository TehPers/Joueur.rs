use crate::base::{Game, Namespace};
use crate::games::chess::r#impl::{GameImpl};
use crate::games::chess::ai::{PLAYER_NAME};

// pub static GAME_NAME: &str = "Chess";
pub static GAME_VERSION: &str = "cfa5f5c1685087ce2899229c04c26e39f231e897ecc8fe036b44bc22103ef801";

pub struct NamespaceImpl {
    game: GameImpl,
}

impl NamespaceImpl {
    fn new() -> NamespaceImpl {
        return NamespaceImpl{
            game: GameImpl::new(),
        };
    }
}

impl Namespace<'a> for NamespaceImpl {
    fn get_game(&'a self) -> &'a dyn Game {
        return &self.game;
    }

    fn get_game_version(&self) -> &'static {
        return GAME_VERSION;
    }

    fn get_player_name(&self) -> &'static {
        return PLAYER_NAME;
    }
}
