mod client;
mod color;

pub struct RunData {
    pub game_name: String,
    pub server: String,
    pub port: String,
}

pub fn run(run_data: &RunData) {
    color::cyan("Hello World!");


    let server = &run_data.server;
    let port = run_data.port.parse().unwrap();

    color::cyan(&format!("Connecting to: {}:{}", server, port));

    let client_instance = client::new();
    client_instance.connect(server, port);

    println!("Done?");
}
