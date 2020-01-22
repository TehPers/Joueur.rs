use std::io;
use std::io::Write;
use std::net::TcpStream;
use std::str;

use crate::joueur::color;

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
    pub fn send_raw(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.print_io {
            let stringified = str::from_utf8(&bytes).unwrap_or("UTF8 error");
            color::magenta(stringified)?;
        }

        self.stream.write_all(bytes)
    }
}
