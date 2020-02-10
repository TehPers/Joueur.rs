// This trait is your interface to GameObject in Chess
use crate::games::chess::{
    Player,
};

pub trait GameObject {
    // -- Attributes -- \\

    /// String representing the top level Class that this game object is an
    /// instance of. Used for reflection to create new instances on clients,
    /// but exposed for convenience should AIs want this data.
    fn game_object_name(&self) -> &str;

    /// A unique id for each instance of a GameObject or a sub class. Used for
    /// client and server communication. Should never change value after being
    /// set.
    fn id(&self) -> &str;

    /// Any strings logged will be stored here. Intended for debugging.
    fn logs(&self) -> &[&str];

    // -- Functions -- \\

    /// Adds a message to this GameObject"s logs. Intended for your own
    /// debugging purposes, as strings stored here are saved in the gamelog.
    ///
    /// # Arguments
    ///
    /// * `message` - A string to add to this GameObject"s log. Intended for
    /// debugging.
    ///
    fn log(&self, message: &str) ;

    // -- Castings -- \\

    // NOTE: castings do not create new instances of game objects on the heap.
    // Instead we just attempt to wrap the underlying struct in a different
    // trait for you, so there is no additional memory cost to casting.

    /// Returns an attempts to cast to a GameObject.
    /// Is None when the cast is invalid, otherwise contains the valid cast.
    fn as_game_object(&self) -> Option<&dyn GameObject>;

    /// Returns an attempts to cast to a Player.
    /// Is None when the cast is invalid, otherwise contains the valid cast.
    fn as_player(&self) -> Option<&dyn Player>;
}
