// This trait is your interface to ${obj_key} in ${game_name}
<%include file='functions.noCreer' /><%
parents = list(obj['parentClasses'])
including = parents
if obj_key == 'GameObject':
    including = game_obj_names
%>${shared['rs']['imports'](obj, obj_key, including)}

pub trait ${obj_key}${': ' if parents else ''}${' + '.join(parents)} {
% if len(obj['attribute_names']) > 0:
    // -- Attributes -- ${'\\\\'}
%   for attr_name in obj['attribute_names']:
<%
        attr = obj['attributes'][attr_name]
%>
${shared['rs']['docstring'](attr)}
    fn ${underscore(attr_name)}(&self) -> ${shared['rs']['type'](attr['type'])};
%   endfor
% endif
% if len(obj['function_names']) > 0:
%   if len(obj['attribute_names']) > 0: # blank line to seperate

%   endif
    // -- Functions -- ${'\\\\'}
%   for func_name in obj['function_names']:
<%
        func = obj['functions'][func_name]
%>
${shared['rs']['function_top'](func_name, func)};
%   endfor
% endif
% if obj_key == 'GameObject':

    // -- Castings -- ${'\\\\'}

    // NOTE: castings do not create new instances of game objects on the heap.
    // Instead we just attempt to wrap the underlying struct in a different
    // trait for you, so there is no additional memory cost to casting.
% for game_obj_name in game_obj_names:

    /// Returns an attempts to cast to a ${game_obj_name}.
    /// Is None when the cast is invalid, otherwise contains the valid cast.
    fn as_${underscore(game_obj_name)}(&self) -> Option${'<'}&${game_obj_name}>;
% endfor
% endif
}
