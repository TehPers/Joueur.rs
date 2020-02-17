mod game_manager_impl;
pub use game_manager_impl::{GameManagerImpl};

mod game_impl;
pub use game_impl::{GameImpl};
% for game_obj_name in game_obj_names:

mod ${underscore(game_obj_name)}_impl;
pub use ${underscore(game_obj_name)}_impl:{${game_obj_name}};
% endfor
