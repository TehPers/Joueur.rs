mod ai;
mod r#impl;

% for obj_name in game_obj_names + ['Game']:
mod ${underscore(obj_name)};
pub use ${underscore(obj_name)}::${'{'+ obj_name +'}'};

% endfor
use crate::base::{GameManager};

pub fn new_game_manager() -> impl GameManager {
    return r#impl::GameManagerImpl::new();
}
