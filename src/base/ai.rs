pub trait AI {
    fn start(&self);
    fn ended(&self, won: bool, reason: &str);
    fn game_updated(&self);
    fn invalid(&self);
    // fn get_setting();
}
