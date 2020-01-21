extern crate termcolor;

use std::io::{Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn colorize(line: &str, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let result = Ok(())
        .and_then(|_| stdout.set_color(ColorSpec::new().set_fg(Some(color))))
        .and_then(|_| writeln!(&mut stdout, "{}", line))
        .and_then(|_| stdout.set_color(&(ColorSpec::default())));

    if result.is_err() {
        println!("ERROR: could not write colored text to stdout");
    }
}

pub fn cyan(line: &str) {
    colorize(line, Color::Cyan);
}
