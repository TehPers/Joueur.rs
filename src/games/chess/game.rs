// This trait is your interface to Game in Chess
use crate::games::chess::{
    GameObject,
    Player,
};

pub trait Game {
    // -- Attributes -- \\

    /// Forsyth-Edwards Notation (fen), a notation that describes the game
    /// board state.
    fn fen(&self) -> &str;

    /// A mapping of every game object"s ID to the actual game object.
    /// Primarily used by the server and client to easily refer to the game
    /// objects via ID.
    fn game_objects(&self) -> &std::collections::HashMap<&str, &dyn GameObject>;

    /// The list of [known] moves that have occurred in the game, in Standard
    /// Algebraic Notation (SAN) format. The first element is the first move,
    /// with the last being the most recent.
    fn history(&self) -> &[&str];

    /// List of all the players in the game.
    fn players(&self) -> &[&dyn Player];

    /// A unique identifier for the game instance that is being played.
    fn session(&self) -> &str;
}
