<%include file='functions.noCreer' /><%
print('--->', obj_key)
parents = []
if obj_key != 'Game':
    parents = shared['rs']['parents'](obj_key, obj)
can_be_cast_to = set(p[0] for p in parents)
%>use serde::ser::{Serialize, Serializer, SerializeStruct};

pub struct ${obj_key}Ref<'a> {
    game_manager: &'a GameManagerImpl,
    id: String,
}

impl<'a> ${obj_key}Ref<'a> {
    fn new(id: &str, game_manager: &'a GameManagerImpl) {
        ${obj_key}Ref {
            id: id.to_string(),
            game_manager: game_manager,
        }
    }

    fn get_impl(&self) -> &${obj_key}Impl {
        self.game_manager.get_game_object_impl(&self.id)
    }
}

impl Serialize for ${obj_key}Ref {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("${obj_key}", 1)?;
        state.serialize_field("id", &self.id)?;
        state.end()
    }
}

impl PartialEq for ${obj_key}Ref {
    fn eq(&self, other: &${obj_key}Ref) -> bool {
        self.id == other.id
    }
}

%   for i, parent in enumerate(parents):
%       if i > 0:

%       endif
impl ${parent[0]} for ${obj_key}Ref {
%       for j, attr_name in enumerate(parent[1]['attribute_names']):
%           if j > 0:

%           endif
<%
            attr = parent[1]['attributes'][attr_name]

%>    fn ${underscore(attr_name)}(&self): ${shared['rs']['type'](attr['type'])} {
        return &self.get_impl().${underscore(attr_name)};
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
        ${'let result = ' if func['returns'] else ''}self.game_manager.run_on_server(
            &self.id_impl,
            "${func_name}",
             json!({
%           for arg in func['arguments']:
                "${arg['name']}": ${underscore(arg['name'])},
%           endfor
             }),
        );
%           if func['returns']:
        de_result = serde_json::from_value::<${shared['rs']['type'](func['returns']['type'], True)}>(result);
        if de_result.is_err() {
            self.game_manager.handle_deserialization_error(
                &de_result.unwrap_err(),
                format!("${obj_key}.${function_names}() id:{}", self.id_impl),
            );
        }
        return de_result.unwrap();
%           endif
    }
%       endfor
%       if parent[0] == 'GameObject':
%           for game_obj_name in game_obj_names:

    fn as_${underscore(game_obj_name)}(&self) -> Option${'<'}&dyn ${game_obj_name}> {
        return ${'Some(&self)' if game_obj_name in can_be_cast_to else 'None'};
    }
%           endfor
%       endif
}
%   endfor
