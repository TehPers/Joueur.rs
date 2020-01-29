use crate::base::namespace::{BaseNamespace};

pub fn make_namespace() -> BaseNamespace {
    return BaseNamespace{
        game_name: "Chess".to_string(),
    };
}
