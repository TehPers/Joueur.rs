mod ai;
mod r#impl;

mod game_object;
pub use game_object::{GameObject};

mod player;
pub use player::{Player};

mod game;
pub use game::{Game};

use crate::base::{GameManager};

pub fn new_game_manager() -> impl GameManager {
    return r#impl::GameManagerImpl::new();
}
