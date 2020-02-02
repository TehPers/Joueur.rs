// This allows up and down casting of game objects.
// Because Rust does not provide an easy way to cast Trait to and from
// one another, we provide these helper functions to all GameObject traits
// for you.

use crate::games::${game_name}::{
% for game_obj_name in game_obj_names:
    ${game_obj_name},
% endfor
}

pub trait Castable {
% for game_obj_name in game_obj_names:
    /// Returns an attempts to cast to a ${game_obj_name}.
    /// Is None when the cast is invalid, otherwise contains the valid cast.
    fn as_${underscore(game_obj_name)}(&self) -> Option${'<'}&${game_obj_name}>;

% endfor
}
