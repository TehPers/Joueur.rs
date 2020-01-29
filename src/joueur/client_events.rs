use serde::{Deserialize, Serialize};

// -- Client Events -- ||

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientEventPlayData {
    client_type: String,
    game_name: String,
    game_settings: String,
    password: String,
    player_index: i32,
    player_name: String,
    requested_session: String,
}

//-- Server Events -- \\

#[derive(Deserialize, Debug)]
pub struct ServerEvent {
    pub event: String,
    pub data: serde_json::Value,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerEventFatalData {
    pub message: String,
    pub timed_out: bool,
}
