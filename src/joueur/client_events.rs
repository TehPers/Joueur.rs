use serde::{Deserialize, Serialize};

// -- Client Events -- ||

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientEventPlayData {
    pub client_type: String,
    pub game_name: String,
    pub game_settings: String,
    pub password: String,
    pub player_index: i32,
    pub player_name: String,
    pub requested_session: String,
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
