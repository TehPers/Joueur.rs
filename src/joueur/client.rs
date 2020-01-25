use std::convert::TryFrom;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::json;

use crate::joueur::color;

const EOT_CHAR: char = 4 as char;
const EOT_U8: u8 = 4;
const BUFFER_SIZE: usize = 1024;

#[derive(Debug, Clone)]
struct SendEventError;

pub struct Client {
    print_io: bool,
    stream: TcpStream,
    bytes_buffer: Vec<u8>,
}

pub fn new(print_io: bool, address: &String) -> io::Result<Client> {
    let stream = TcpStream::connect(address)?;
    stream.set_nodelay(true)?;

    Ok(Client{
        print_io: print_io,
        stream: stream,
        bytes_buffer: Vec::new()
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

        let mut s = serde_payload.to_string();
        s.push(EOT_CHAR);

        // TODO: handle both error types
        let _ = self.send_raw(s.as_bytes());

        Ok(())
    }

    pub fn send_raw(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.print_io {
            let stringified = str::from_utf8(&bytes).unwrap_or("UTF8 error");
            color::magenta("TO SERVER -->")?;
            color::magenta(stringified)?;
        }

        self.stream.write_all(bytes)
    }

    pub fn wait_for_events(&mut self) -> io::Result<()> {
        if self.bytes_buffer.contains(&EOT_U8) {
            // already at least 1 event
            return Ok(());
        }

        loop {
            let mut buf = vec![0; BUFFER_SIZE];
            let chars_read = self.stream.read(&mut buf)?;

            if chars_read == 0 {
                continue; // keep trying to read, probably want to add a timeout?
            }

            if self.print_io {
                color::magenta("FROM SERVER <--")?;
                let str_result = str::from_utf8(&buf);
                color::magenta(str_result.unwrap_or_default())?;
            }

            self.bytes_buffer.append(&mut buf);

            if buf.contains(&EOT_U8) {
                break;
            }
        }
        Ok(())
    }

    pub fn print_events(&mut self) -> io::Result<()> {
        self.wait_for_events()?;

        let split = self.bytes_buffer.split(|char_byte| char_byte == &EOT_U8);
        for event_bytes in split {
            println!("got: {}", event_bytes[1].to_string())
        }

        Ok(())
    }
}
