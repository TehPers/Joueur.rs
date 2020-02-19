<%include file='functions.noCreer' /><%
seen_types = set()
all_types_dict = {}

def record_type(base_type):
    if base_type['is_game_object']:
        all_types_dict['game_object'] = base_type
    else:
        all_types_dict[shared['rs']['deep_type_name'](base_type)] = base_type
        if base_type['name'] in ['list', 'dictionary']:
            record_type(base_type['valueType'])

            if base_type['name'] == 'dictionary':
                record_type(base_type['keyType'])

for obj_name in ['Game'] + game_obj_names:
    obj = game if obj_name == 'Game' else game_objs[obj_name]
    for attr_name in obj['attribute_names']:
        record_type(obj['attributes'][attr_name]['type'])

all_types = [ (t, all_types_dict[t]) for t in sorted(all_types_dict) ]
%>

struct DeltaMerge<'a> {
    value: &'a serde_json::Value,
}

impl DeltaMerge {
% for type_name, base_type in all_types:
%   if type_name in ['string', 'boolean', 'int', 'float']:
fn ${type_name}(&self, value: &mut ${shared['rs']['type'](base_type, True)}) -> serde_json::Result<()> {
    *val = serde_json::from_value::<${shared['rs']['type'](base_type, True)}>(self.value)?;
%   elif type_name == 'game_object':
fn game_object_reference(&self, &mut String) -> serde_json::Result<()> {
    let game_object_ref = serde_json::from_value::${'<'}GameObjectReference>(self.value)?;
    &val = game_object_ref.id;
%   endif
    Ok()
}
% endfor
}
