use crate::base;

pub struct ChessGameImpl {
    parent_game: base::game::GameImpl,
}

impl base::game::Game for ChessGameImpl {
    // Dislike this boilerplate...
    fn get_game_object(&self, id: &str) -> Option<&str> {
        return self.parent_game.get_game_object(id);
    }
}

pub fn new() -> impl base::game::Game {
    return ChessGameImpl{
        parent_game: base::game::new(),
    };
}
