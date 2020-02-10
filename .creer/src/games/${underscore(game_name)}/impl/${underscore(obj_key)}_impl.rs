% if obj_key == 'Game':
use crate::base;

pub struct GameImpl {
    parent_game: base::GameImpl,
}

impl GameImpl {
    pub fn new() -> GameImpl {
        GameImpl{
            parent_game: base::GameImpl::new(),
        }
    }
}

impl base::Game for GameImpl {
    // Dislike this boilerplate...
    fn get_game_object(&self, id: &str) -> Option${'<'}&str> {
        return self.parent_game.get_game_object(id);
    }
}
% else:
// TODO: do this implimentation
% endif
