use std::convert::TryFrom;
use std::io;
use std::io::Write;
use std::net::TcpStream;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::json;

use crate::joueur::color;

#[derive(Debug, Clone)]
struct SendEventError;

pub struct Client {
    print_io: bool,
    stream: TcpStream,
}

pub fn new(print_io: bool, address: &String) -> io::Result<Client> {
    let stream = TcpStream::connect(address)?;
    stream.set_nodelay(true)?;

    Ok(Client{
        print_io: print_io,
        stream: stream,
    })
}

impl Client {
    pub fn send_event_alias(&mut self, game_name: &String) -> serde_json::Result<()> {
        self.send_event("alias", json!(game_name))
    }

    pub fn send_event(&mut self, event_name: &str, data: serde_json::Value) -> serde_json::Result<()> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH);
        let serde_payload = json!({
            "name": event_name,
            "data": data,
            "sentTime": i64::try_from(now.unwrap_or_default().as_millis()).unwrap_or_default(), // TODO: ugly
        });

        let s = serde_payload.to_string();

        // TODO: handle both error types
        let _ = self.send_raw(s.as_bytes());

        Ok(())
    }

    pub fn send_raw(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.print_io {
            let stringified = str::from_utf8(&bytes).unwrap_or("UTF8 error");
            color::magenta(stringified)?;
        }

        self.stream.write_all(bytes)
    }
}
