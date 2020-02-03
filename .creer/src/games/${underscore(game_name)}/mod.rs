// This is barrel for exports in the ${game_name} game.

mod castable;
pub use castable::{Castable};

mod game;
pub use game::{Game};

mod ai;
pub use ai::{AI};
% for obj_name in game_obj_names:

mod ${underscore(obj_name)};
pub use ${underscore(obj_name)}::{${obj_name}};
% endfor

use crate::base::{Namespace};

pub fn make_namespace() -> Namespace {
    return Namespace{
        game_name: "${game_name}".to_string(),
        game_version: "${game_version}".to_string(),
        player_name: ai::PLAYER_NAME.to_string(),
        game: Box::new(_impl::GameImpl::new()),
    };
}
