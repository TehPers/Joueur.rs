#[macro_use]
extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("game")
            .help("The name of the game you want to play on the server")
            .required(true)
        )
        .arg(Arg::with_name("server")
            .short("s").long("server")
            .help("The url to the server you want to connect to")
            .default_value("localhost")
        )
        .arg(Arg::with_name("name")
            .short("n")
            .long("name")
            .help("The name you want to use as your AI's player name. This overrides the name you set in your code")
        )
        .arg(Arg::with_name("password")
            .short("w")
            .long("password")
            .help("The password required for authentication on official servers")
        )
        .arg(Arg::with_name("gameSettings")
            .long("gameSettings")
            .help("Any settings for the game server to force")
        )
        .arg(Arg::with_name("session")
            .short("r")
            .long("session")
            .help("The requested game session you want to play on the server")
        )
        .arg(Arg::with_name("aiSettings")
            .long("aiSettings")
            .help("Any settings for the AI")
        )
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .help("The port to connect on the server")
            .default_value("3000")
        )
        .arg(Arg::with_name("index")
            .short("i")
            .long("index")
            .help("The player number you want to be, with 0 being the first player")
        )
        .get_matches();

    let game = matches.value_of("game").take();
    match game {
        Some(v) => println!("TODO: play {}", v),
        None => println!("This should be impossible"),
    }
}
