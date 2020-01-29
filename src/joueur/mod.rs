mod client_events;
mod client;
mod color;
mod errors;
mod game_manager;

use std::error::Error;

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

fn run_safe(run_data: &RunData) -> Result<(), Box<dyn Error>> {
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

    let mut client_instance = client::new(run_data.print_io, address)?;

    client_instance.send_event_alias(&run_data.game_name);
    let game_name = client_instance.wait_for_event_named();

    println!("real game name is: {}", game_name);

    let manager = game_manager::new(&game_name);

    let default_player_name = "Rust Player";
    let player_name = (match true {
        _ if run_data.player_name != "" => &run_data.player_name,
        _ if manager.game_namespace.player_name != "" => &manager.game_namespace.player_name,
        _ => default_player_name,
    }).to_string();

    client_instance.send_event_play(&client_events::ClientEventPlayData{
        client_type: "rust".to_string(),
        game_name: game_name,
        game_settings: run_data.game_settings.to_string(),
        password: run_data.password.to_string(),
        player_index: player_index,
        player_name: player_name,
        requested_session: run_data.requested_session.to_string(),
    });

    Ok(())
}

pub fn run(run_data: &RunData) {
    let result = run_safe(run_data);

    if result.is_err() {
        println!("Unexpected error running!");
    }
}
