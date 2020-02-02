// This is barrel for exports in the ${game_name} game.

mod castable;
pub use castable::{Castable};

mod game;
pub use game::{Game};
% for obj_name in game_obj_names:

mod ${underscore(obj_name)};
pub use ${underscore(obj_name)}::{${obj_name}};
% endfor
