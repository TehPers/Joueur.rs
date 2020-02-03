use crate::base;

pub struct ChessGameImpl {
    parent_game: base::GameImpl,
}

impl ChessGameImpl {
    pub fn new() -> ChessGameImpl {
        ChessGameImpl{
            parent_game: base::GameImpl::new(),
        }
    }
}

impl base::Game for ChessGameImpl {
    // Dislike this boilerplate...
    fn get_game_object(&self, id: &str) -> Option<&str> {
        return self.parent_game.get_game_object(id);
    }
}
