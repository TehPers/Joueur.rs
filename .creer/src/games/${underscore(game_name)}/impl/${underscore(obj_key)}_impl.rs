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
<%include file='functions.noCreer' />${shared['rs']['imports'](obj, obj_key, ['GameManagerImpl', obj_key] + obj['parentClasses'])}
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
# ref = "<'a>" if has_game_obj else ''
%>
pub struct ${obj_key}Impl<'a> {
    pub game_manager: &'a GameManagerImpl,

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
            fn_body = []
            if attr_type['is_game_object']:
                class_name = attr_type['name']
                underscore_name = underscore(class_name)
                impl_name = '{}_id_impl'.format(underscore_name)
                casting = 'game_object.as_{}()'.format(underscore_name)
                fn_body.append('let game_object = self.game_manager.get(&self.{});'.format(impl_name))
                if attr_type['nullable']:
                    fn_body.append('return {};'.format(casting))
                else:
                    fn_body.extend([
                        'if {}.is_none() {{'.format(underscore_name),
                        '    self.game_manager.handle_no_game_object(&self.{}, "{}");'.format(impl_name, class_name),
                        '}',
                        'return {}.unwrap();'.format(underscore_name)
                    ])
            else:
                fn_body.append('return &self.{}_impl;'.format(underscore(attr_name)))

%>    fn ${underscore(attr_name)}(&self): ${shared['rs']['type'](attr_type)} {
%           for fn_body_line in fn_body:
        ${fn_body_line}
%           endfor
    }
%       endfor
%       for j, func_name in enumerate(parent[1]['function_names']):
<%
            func = parent[1]['functions'][func_name]
            arg_strs = []
            if func['arguments']:
                arg_strs.append('')
                for arg in func['arguments']:
                    arg_strs.append('{}: {}'.format(underscore(arg['name']), shared['rs']['type'](arg['type'])))
            ret_signature = ''
            if func['returns']:
                ret_signature = '-> {}'.format(shared['rs']['type'](func['returns']['type']))
%>
    fn ${func_name}(&mut self${', '.join(arg_strs)}) ${ret_signature}{
        // TODO: do
    }
%       endfor
}
%   endfor

impl<'a> ${obj_key}Impl<'a> {
    fn new(game_manager: &'a GameManagerImpl) -> ${obj_key}Impl<'a> {
        ${obj_key}Impl<'a> {
            game_manager: game_manager,

%   for i, parent in enumerate(objs):
%       if i > 0:

%       endif
%       for attr_name in parent[1]['attribute_names']:
<%
            attr = parent[1]['attributes'][attr_name]
            field_name = field_name_for(attr_name, attr)
%>            ${field_name}_impl: ${shared['rs']['default'](attr['type'])},
%       endfor
%   endfor
        }
    }
}
% endif
