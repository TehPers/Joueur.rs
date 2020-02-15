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
<%include file='functions.noCreer' />${shared['rs']['imports'](obj, obj_key, [obj_key] + obj['parentClasses'])}
<%
objs = [(obj_key, obj)] + [ (n, game_objs[n]) for n in obj['parentClasses'] ]
objs = [r for r in reversed(objs)]
def check_has_game_obj():
    for parent in obj:
        for attr in obj['attributes'].values():
            if attr['type']['is_game_object']:
                return True
    return False

def field_name_for(attr_name, attr):
    field_name = underscore(attr_name)
    if attr['type']['is_game_object']:
        field_name += '_id'
    return field_name

has_game_obj = check_has_game_obj()
ref = "<'a>" if has_game_obj else ''
%>
pub struct ${obj_key}Impl${ref} {
    pub game_manager: GameManagerImpl,

%   for i, parent in enumerate(objs):
%       if i > 0:

%       endif
%       for attr_name in parent[1]['attribute_names']:
<%
            attr = parent[1]['attributes'][attr_name]
            field_name = field_name_for(attr_name, attr)
%>    pub ${field_name}_impl: ${shared['rs']['type'](attr['type'], True)},
%       endfor
%   endfor
}

%   for i, parent in enumerate(objs):
%       if i > 0:

%       endif
impl ${parent[0]} for ${obj_key}Impl {
%       for j, attr_name in enumerate(parent[1]['attribute_names']):
%           if j > 0:

%           endif
<%
            attr = parent[1]['attributes'][attr_name]
            attr_type = attr['type']
            field_name = field_name_for(attr_name, attr)
            fn_body = ''
            if attr_type['is_game_object']:
                fn_body = """        let game_object = self.game_manager.get(&self.{impl})';
        let {class_name} = game_object.as_{class_name};
"""
                if attr_type['nullable']:
                    fn_body += '        return {};'.format(class_name)
                else:
                    fn_body += """        if {class_name}.is_none() {{
            self.game_manager.handle_no_game_object(&self.{impl}, "{class_name}");
        }}
        return {class_name}.unwrap();
"""

%>    fn ${underscore(attr_name)}(&self): ${shared['rs']['type'](attr_type)} {
        return self.${field_name}_impl${'.unwrap()' if attr_type['is_game_object'] and not attr_type['nullable'] else ''};
    }
%       endfor
}
%   endfor
% endif
