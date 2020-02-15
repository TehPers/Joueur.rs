<%include file='functions.noCreer' />use crate::base::{Game, GameManager};
use crate::games::${underscore(game_name)}::r#impl::{GameImpl};
% for game_obj_name in game_obj_names:
use crate::games::${underscore(game_name)}::r#impl::{${game_obj_name}Impl};
% endfor
use crate::games::${underscore(game_name)}::ai::{PLAYER_NAME};

// pub static GAME_NAME: &str = "${game_name}";
pub static GAME_VERSION: &str = "${game_version}";

pub struct GameManagerImpl {
    game: GameImpl,
    game_objects: std::collections::HashMap<String, Box<GameObject>>,
}

impl GameManagerImpl {
    pub fn new() -> GameManagerImpl {
        return GameManagerImpl{
            game: GameImpl::new(),
            game_objects: std::collections::HashMap::new(),
        };
    }
}

impl GameManagerImpl {
% for i, game_obj_name in enumerate(game_obj_names):
%   if i > 0:

%   endif
<%
    game_obj = game_objs[game_obj_name]
    all = game_obj['parentClasses'] + [game_obj_name]
%>    fn new_${underscore(game_obj_name)}(&self) -> dyn ${game_obj_name} {
        return ${game_obj_name}Impl {
%   for class_name in all:
%       for attr_name in game_objs[class_name]['attribute_names']:
<%
            attr = game_objs[class_name]['attributes'][attr_name]
%>          ${attr_name}: ${shared['rs']['default'](attr['type'])},
%       endfor
%   endfor
        };
    },
% endfor
}

impl GameManager for GameManagerImpl {
    fn ensure_game_object(&mut self, id: &str, game_object_name: &str) -> Optional<bool> {
        if self.game_objects.contains_key(id) {
            return Some(false);
        }

        let new_game_object = match game_object_name {
% for game_obj_name in game_obj_names:
            "${game_obj_name}" => self.new_${underscore(game_obj_name)},
% endfor
            _ => return None,
        }

        Some(true)
    }

    fn get_game<'a>(&'a self) -> &'a dyn Game {
        return &self.game;
    }

    fn get_game_version(&self) -> &'static str {
        return GAME_VERSION;
    }

    fn get_player_name(&self) -> &'static str {
        return PLAYER_NAME;
    }
}
