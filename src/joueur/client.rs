use std::collections;
use std::convert::TryFrom;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::json;

use crate::joueur::client_events;
use crate::joueur::color;
use crate::joueur::errors;

const EOT_CHAR: char = 4 as char;
const EOT_U8: u8 = 4;
const BUFFER_SIZE: usize = 1024;

pub struct Client {
    print_io: bool,
    stream: TcpStream,
    bytes_buffer: Vec<u8>,
    events: collections::VecDeque<Vec<u8>>,
}

pub fn new(print_io: bool, address: &String) -> io::Result<Client> {
    let connect_result = TcpStream::connect(address);

    if connect_result.is_err() {
        errors::handle_error(
            errors::ErrorCode::CouldNotConnect,
            &format!("Could not connect to {}", address),
            Some(&connect_result.unwrap_err()),
        );
    }
    let stream = connect_result.unwrap();

    stream.set_nodelay(true)?;

    Ok(Client{
        print_io: print_io,
        stream: stream,
        bytes_buffer: Vec::new(),
        events: collections::VecDeque::new(),
    })
}

impl Client {
    pub fn send_event(&mut self, event_name: &str, data: serde_json::Value) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH);
        let serde_payload = json!({
            "event": event_name,
            "data": data,
            "sentTime": i64::try_from(now.unwrap_or_default().as_millis()).unwrap_or_default(), // TODO: ugly
        });

        let mut s = serde_payload.to_string();
        s.push(EOT_CHAR);

        // TODO: handle both error types
        let _ = self.send_raw(s.as_bytes());
    }

    pub fn send_raw(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.print_io {
            color::magenta(&format!("TO SERVER --> {}", str::from_utf8(&bytes).unwrap_or("UTF8 error")));
        }

        self.stream.write_all(bytes)
    }

    // -- Client Events -- \\
    pub fn send_event_alias(&mut self, game_name: &str) {
        self.send_event("alias", json!(game_name));
    }

    pub fn send_event_play(&mut self, play_data: &client_events::ClientEventPlayData) {
        self.send_event("play", json!(play_data))
    }

    // -- Server Events -- \\

    pub fn wait_for_event_named(&mut self) -> String {
        return self.wait_for_event::<String>("named");
    }

    pub fn wait_for_event_lobbied(&mut self) -> client_events::ServerEventLobbiedData {
        return self.wait_for_event::<client_events::ServerEventLobbiedData>("lobbied");
    }

    pub fn wait_for_events(&mut self) {
        while self.events.len() == 0 {
            let mut buf = vec![0; BUFFER_SIZE];
            let read_result = self.stream.read(&mut buf);

            if read_result.is_err() {
                errors::handle_error(
                    errors::ErrorCode::CannotReadSocket,
                    "Cannot read socket while waiting for events",
                    Some(&read_result.unwrap_err()),
                )
            }

            let bytes_read = read_result.unwrap();
            if bytes_read == 0 {
                continue; // keep trying to read, probably want to add a timeout?
            }

            if self.print_io {
                color::magenta(&format!("FROM SERVER <-- {}", str::from_utf8(&buf).unwrap_or_default()));
            }

            buf.truncate(bytes_read); // chop off unread bytes
            let mut all_bytes = self.bytes_buffer.clone();
            all_bytes.append(&mut buf);
            self.bytes_buffer.truncate(0); // and flush it

            let mut events = all_bytes
                .split(|&c| c == EOT_U8)
                .into_iter()
                .collect::<Vec<&[u8]>>();
            let last_result = events.pop();

            if last_result.is_none() {
                errors::handle_error(
                    errors::ErrorCode::MalformedJSON,
                    "read buffer had elements but could not be split?",
                    None,
                )
            }
            let last = last_result.unwrap();
            if last.len() > 0 {
                // a partial json structure is present, so buffer it
                self.bytes_buffer.append(&mut last.to_vec());
            }

            for event in events {
                self.events.push_back(event.to_vec());
            }
        }
    }

    pub fn wait_for_event<T>(&mut self, event_name: &str)
        -> T where T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        loop {
            self.wait_for_events();
            // once that returns there should be events in the buffer to parse
            while let Some(event) = self.events.pop_front() {
                let de_serialized_result = serde_json::from_slice::<client_events::ServerEvent>(&event);

                if de_serialized_result.is_err() {
                    errors::handle_error(
                        errors::ErrorCode::MalformedJSON,
                        &format!(
                            "Could not parse data while waiting for event: '{:?}'.\nData: {:?}",
                            event_name,
                            str::from_utf8(&event).unwrap_or("event bytes not valid string")
                        ),
                        Some(&de_serialized_result.unwrap_err()),
                    );
                }

                let sent = de_serialized_result.unwrap();
                if sent.event == event_name {
                    let de_result = serde_json::from_value::<T>(sent.data);
                    if de_result.is_err() {
                        errors::handle_error(
                            errors::ErrorCode::MalformedJSON,
                            &format!("Could not transform {} event data to a String", event_name),
                            Some(&de_result.unwrap_err()),
                        );
                    }
                    return de_result.unwrap();
                }
                else {
                    self.auto_handle_event(&sent);
                }
            }
        }
    }

    fn auto_handle_event(&self, sent_event: &client_events::ServerEvent) {
        let event_name: &str = &sent_event.event;
        match event_name {
            // TODO: add more auto handlers here
            "fatal" => self.auto_handle_event_fatal(&sent_event.data),
            _ => errors::handle_error(
                errors::ErrorCode::UnknownEventFromServer,
                &format!(
                    "Could not auto handle event from server: '{}'",
                    sent_event.event,
                ),
                None,
            ),
        }
    }

    fn auto_handle_event_fatal(&self, data: &serde_json::Value) -> ! {
        let fatal_data_result = serde_json::from_value::<client_events::ServerEventFatalData>(data.to_owned());
        let fatal_message = if fatal_data_result.is_ok() {
            fatal_data_result.unwrap().message
        } else {
            format!("Could not parse fatal event from server {}", data.to_string())
        };

        errors::handle_error(
            errors::ErrorCode::FatalEvent,
            &fatal_message,
            None,
        )
    }
}
