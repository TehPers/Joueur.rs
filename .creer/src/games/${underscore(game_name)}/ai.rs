<%include file='functions.noCreer' />${shared['rs']['imports'](ai, 'AI', ['Game', 'Player'])}

/// This should be the string name of your Player in games it plays.
pub static PLAYER_NAME: &str = "${game_name} Rust Player";

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

% for func_name in ai['function_names']:
<%
    func = ai['functions'][func_name]
    returns = ''
    if func['returns']:
        returns = ' {}'.format(shared['rs']['default'](func['returns']['type']))
%>${shared['rs']['function_top'](func_name, func)} {
        return${returns};
    }

% endfor
}
