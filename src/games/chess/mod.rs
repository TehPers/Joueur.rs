mod ai;
mod r#impl;

use crate::base::{GameManager};

pub fn new_game_manager() -> impl GameManager {
    return r#impl::GameManagerImpl::new();
}
