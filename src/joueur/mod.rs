mod client_events;
mod client;
mod color;
mod errors;

use std::error::Error;
use crate::games;

pub struct RunData {
    pub game_name: String,
    pub server: String,
    pub port: String,
    pub print_io: bool,
}

fn run_safe(run_data: &RunData) -> Result<(), Box<dyn Error>> {
    color::cyan("Hello World!");

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

    // TODO: check if we have that game?
    let game_namespace = games::get_game(&game_name);

    match game_namespace {
        None => println!("no game for {}", game_name),
        Some(_) => println!("yay {}", game_name),
    }

    Ok(())
}

pub fn run(run_data: &RunData) {
    let result = run_safe(run_data);

    if result.is_err() {
        println!("Unexpected error running!");
    }
}
