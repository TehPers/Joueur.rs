// This trait is your interface to ${obj_key} in ${game_name}
<%include file='functions.noCreer' /><%
parents = list(obj['parentClasses'])
if obj_key == 'GameObject':
    parents.append('Castable')
%>${shared['rs']['imports'](obj)}

pub trait ${obj_key}${': ' if parents else ''}${' + '.join(parents)} {
% if len(obj['attribute_names']) > 0:
    // -- Attributes -- ${'\\\\'}
%   for attr_name in obj['attribute_names']:
<%
        attr = obj['attributes'][attr_name]
%>    /// ${attr['description']}
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
%>    /// ${func['description']}
    fn ${underscore(func_name)}(&self${''.join([', {}: {}'.format(underscore(arg['name']), shared['rs']['type'](arg['type'])) for arg in func['arguments']])})${' -> {}'.format(shared['rs']['type'](func['returns']['type'])) if func['returns'] else ''};
%   endfor
% endif
}
