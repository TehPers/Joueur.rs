extern crate termcolor;

use std::io;
use std::io::{Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn colorize_safe(line: &str, color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    writeln!(&mut stdout, "{}", line)?;
    stdout.set_color(&(ColorSpec::default()))?;

    Ok(())
}

fn colorize(line: &str, color: Color) {
    let result = colorize_safe(line, color);
    if result.is_err() {
        println!("!Color Error!: {}", line);
    }
}

pub fn cyan(line: &str) {
    colorize(line, Color::Cyan);
}

pub fn magenta(line: &str) {
    colorize(line, Color::Magenta);
}

pub fn red(line: &str) {
    colorize(line, Color::Red);
}

pub fn yellow(line: &str) {
    colorize(line, Color::Yellow);
}
