use crate::base::namespace::BaseNamespace;
use crate::games;
use crate::joueur::errors;

pub struct GameManager {
    pub game_namespace: BaseNamespace,
}

impl GameManager {
    pub fn new(game_name: &str) -> GameManager {
        let make_game_namespace_result = games::get_game(game_name);

        if make_game_namespace_result.is_none() {
            errors::handle_error(
                errors::ErrorCode::GameNotFound,
                &format!("Could not find a game with name '{}' to play", game_name),
                None,
            );
        }

        let make_game_namespace = make_game_namespace_result.unwrap();

        return GameManager{
            game_namespace: make_game_namespace(),
        }
    }
}
