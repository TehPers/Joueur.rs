use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct SentEvent {
    pub event: String,
    pub data: serde_json::Value,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SentEventFatalData {
    pub message: String,
    pub timed_out: bool,
}
