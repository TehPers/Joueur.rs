pub trait Game {
    fn get_game_object(&self, id: &str) -> Option<&str>;
}

pub struct GameImpl {
    pub game_objects: std::collections::HashMap<String, String>,
}

impl GameImpl {
    pub fn new() -> GameImpl {
        GameImpl{
            game_objects: std::collections::HashMap::new(),
        }
    }

    pub fn get_game_object(&self, id: &str) -> Option<&str> {
        let got = self.game_objects.get(id);
        if got.is_some() {
            let got_got = got.unwrap();
            return Some(got_got);
        }
        else {
            return None;
        }
    }
}
