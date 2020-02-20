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
<%include file='functions.noCreer' />use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json::json;
${shared['rs']['imports'](obj, obj_key, ['GameManagerImpl', obj_key] + obj['parentClasses'])}
<%
parents = shared['rs']['parents'](obj_key, obj)
print('!!!> parents')
%>
pub struct ${obj_key}Impl {
%   for i, parent in enumerate(parents):
%       if i > 0:

%       endif
%       for attr_name in parent[1]['attribute_names']:
<%
            attr = parent[1]['attributes'][attr_name]
%>    pub ${underscore(attr_name)}: ${shared['rs']['type'](attr['type'], True)},
%       endfor
%   endfor
}

impl ${obj_key}Impl {
    fn new() -> ${obj_key}Impl {
        ${obj_key}Impl{
%   for i, parent in enumerate(parents):
%       if i > 0:

%       endif
%       for attr_name in parent[1]['attribute_names']:
<%
            attr = parent[1]['attributes'][attr_name]
%>            ${underscore(attr_name)}: ${shared['rs']['default'](attr['type'])},
%       endfor
%   endfor
        }
    }

    fn delta_merge(&mut self, key: &str, dv: &DeltaValue) -> serde_json::Result<()> {
        match key {
%   for i, parent in enumerate(parents):
%       for attr_name in parent[1]['attribute_names']:
<%
            attr = parent[1]['attributes'][attr_name]
%>            "${attr_name}" => dv.${shared['rs']['deep_type_name'](attr['type'])}(&self.${underscore(attr_name)})?,
%       endfor
%   endfor
            _ => return Err(serde_json::Error::custom(
                format!("Cannot delta merge {} in ${obj_key}", key),
            )),
        }

        Ok(())
    }
}
% endif
