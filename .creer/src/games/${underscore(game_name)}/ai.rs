use crate::games::${underscore(game_name)}::{
    Game,
    Player,
};

pub static PLAYER_NAME: &str = "${game_name} Rust Player";

pub struct AI<'a> {
    /// The Player your AI controls in the Game.
    player: &'a Player,
    /// The Game representation that you are playing.
    game: &'a Game,
}

impl AI<'a> {
    /// This is invoked when the game starts to create your AI struct.
    /// If you add other fields to your struct you can initialize them here.
    fn new(player: &'a Player, game: &'a Game) -> AI {
        AI{
            player: player,
            game: game,
        }
    }

% for func_name in ai['function_names']:
<%
    func = ai['functions'][func_name]
%>    /// ${func['description']}
${shared['rs']['function_top'](func)}
        return${' {}'.format(shared['rs']['default'](func['returns']['type]) if func['returns'] else '')};
    }
% endfor
}
