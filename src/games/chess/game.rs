use crate::base;

pub struct ChessGameImpl {
    parent_game: base::game::GameImpl,
}

impl ChessGameImpl {
    pub fn new() -> ChessGameImpl {
        ChessGameImpl{
            parent_game: base::game::GameImpl::new(),
        }
    }
}

impl base::game::Game for ChessGameImpl {
    // Dislike this boilerplate...
    fn get_game_object(&self, id: &str) -> Option<&str> {
        return self.parent_game.get_game_object(id);
    }
}
