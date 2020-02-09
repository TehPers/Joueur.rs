use crate::base::{GameManager};
use crate::joueur::color;
use crate::joueur::client::{Client};
use crate::joueur::client_events;
use crate::joueur::errors;
use crate::games;

pub struct RunData {
    pub game_name: String,
    pub server: String,
    pub port: String,
    pub print_io: bool,
    pub game_settings: String,
    pub password: String,
    pub player_index: String,
    pub player_name: String,
    pub requested_session: String,
}

pub fn run(run_data: &RunData) {
    color::cyan("Hello World!");

    let player_index_parsed = run_data.player_index.parse::<i32>();
    if player_index_parsed.is_err() {
        errors::handle_error(
            errors::ErrorCode::InvalidArgs,
            &format!("playerIndex CLI arg is invalid: '{}'", run_data.player_index),
            Some(&player_index_parsed.unwrap_err()),
        )
    }
    let player_index = player_index_parsed.unwrap();

    let combined_address = format!("{}:{}", run_data.server, run_data.port);
    let address = if run_data.server.contains(":") {
        &run_data.server
    } else {
        &combined_address
    };

    color::cyan(&format!("Connecting to: {}", address));

    let mut client = Client::new(run_data.print_io, address);

    client.send_event_alias(&run_data.game_name);
    let game_name = client.wait_for_event_named();

    let make_game_manager_result = games::get_game_manager(&game_name);
    if make_game_manager_result.is_none() {
        errors::handle_error(
            errors::ErrorCode::GameNotFound,
            &format!("Could not find a game with name '{}' to play", game_name),
            None,
        );
    }

    let new_game_manager = make_game_manager_result.unwrap();
    let game_manager_box = new_game_manager();
    let game_manager: &dyn GameManager = &*game_manager_box;

    let player_name =
        if run_data.player_name != "" { &run_data.player_name }
        else if game_manager.get_game_version() != "" { game_manager.get_player_name() }
        else { "Rust Player" };

    client.send_event_play(&client_events::ClientEventPlayData{
        client_type: "rust".to_string(),
        game_name: game_name,
        game_settings: run_data.game_settings.to_string(),
        password: run_data.password.to_string(),
        player_index: player_index,
        player_name: player_name.to_string(),
        requested_session: run_data.requested_session.to_string(),
    });

    let lobbied_data = client.wait_for_event_lobbied();
    color::cyan(&format!("In lobby for game {} in session {}", &lobbied_data.game_name, &lobbied_data.game_session));

    let our_game_version = game_manager.get_game_version();
    if lobbied_data.game_version != our_game_version {
        color::yellow(&format!(
            "WARNING: Game versions do not match.
-> Your local game version is:     {}
-> Game Server's game version is:  {}

Version mismatch means that unexpected crashes may happen due to differing game structures",
            &our_game_version[..8],
            &lobbied_data.game_version[..8],
        ));
    }

    let start_data = client.wait_for_event_start();
    println!("game is starting, we are player id: {:?}", start_data.player_id);

    println!("Done?")
}
