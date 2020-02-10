use crate::games::chess::{
    Game,
    Player,
};

/// This should be the string name of your Player in games it plays.
pub static PLAYER_NAME: &str = "Chess Rust Player";

/// This is your AI struct. Add fields to track state here if you wish.
#[allow(dead_code)]
pub struct AI<'a> {
    /// The Player your AI controls in the Game.
    player: &'a dyn Player,
    /// The Game representation that you are playing.
    game: &'a dyn Game,
}

#[allow(dead_code)]
impl<'a> AI<'a> {
    /// This is invoked when the game starts to create your AI struct.
    /// If you add other fields to your struct you can initialize them here.
    fn new(player: &'a dyn Player, game: &'a dyn Game) -> AI<'a> {
        AI{
            player: player,
            game: game,
        }
    }

    /// This is called every time it is this AI.player"s turn to make a move.
    ///
    /// # Returns
    ///
    /// A string in Standard Algebraic Notation (SAN) for the move you want to
    /// make. If the move is invalid or not properly formatted you will lose
    /// the game.
    fn make_move(&self) -> String {
        return "".to_string();
    }

}
