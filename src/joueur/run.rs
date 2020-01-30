use crate::joueur::color;
use crate::joueur::client;
use crate::joueur::client_events;
use crate::joueur::errors;
use crate::joueur::game_manager;

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

    let mut client_instance = client::new(run_data.print_io, address);

    client_instance.send_event_alias(&run_data.game_name);
    let game_name = client_instance.wait_for_event_named();

    let manager = game_manager::new(&game_name);

    let player_name =
        if run_data.player_name != "" { &run_data.player_name }
        else if manager.game_namespace.player_name != "" { &manager.game_namespace.player_name }
        else { "Rust Player" };

    client_instance.send_event_play(&client_events::ClientEventPlayData{
        client_type: "rust".to_string(),
        game_name: game_name,
        game_settings: run_data.game_settings.to_string(),
        password: run_data.password.to_string(),
        player_index: player_index,
        player_name: player_name.to_string(),
        requested_session: run_data.requested_session.to_string(),
    });

    let lobbied_data = client_instance.wait_for_event_lobbied();
    color::cyan(&format!("In lobby for game {} in session {}", &lobbied_data.game_name, &lobbied_data.game_session));

    println!("Done?")
}
