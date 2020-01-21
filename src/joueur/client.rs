pub struct Client {
    pub connected: bool
}

pub fn new() -> Client {
    return Client{
        connected: false
    }
}

impl Client {
    pub fn connect(&self, server: &String, port: i32) {
        if !self.connected {
            println!("TODO: connect to {}:{}", server, port);
        }
    }
}
