// This trait is your interface to Player in Chess
use crate::games::chess::{
    GameObject,
};

pub trait Player: GameObject {
    // -- Attributes -- \\

    /// What type of client this is, e.g. "Python", "JavaScript", or some
    /// other language. For potential data mining purposes.
    fn client_type(&self) -> &str;

    /// The color (side) of this player. Either "white" or "black", with the
    /// "white" player having the first move.
    fn color(&self) -> &str;

    /// If the player lost the game or not.
    fn lost(&self) -> &bool;

    /// The name of the player.
    fn name(&self) -> &str;

    /// This player"s opponent in the game.
    fn opponent(&self) -> &dyn Player;

    /// The reason why the player lost the game.
    fn reason_lost(&self) -> &str;

    /// The reason why the player won the game.
    fn reason_won(&self) -> &str;

    /// The amount of time (in ns) remaining for this AI to send commands.
    fn time_remaining(&self) -> &f64;

    /// If the player won the game or not.
    fn won(&self) -> &bool;
}
