extern crate termcolor;

use std::io;
use std::io::{Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn colorize(line: &str, color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    writeln!(&mut stdout, "{}", line)?;
    stdout.set_color(&(ColorSpec::default()))?;

    Ok(())
}

pub fn cyan(line: &str) -> io::Result<()> {
    colorize(line, Color::Cyan)
}

pub fn magenta(line: &str) -> io::Result<()> {
    colorize(line, Color::Magenta)
}

pub fn red(line: &str) -> io::Result<()> {
    colorize(line, Color::Red)
}
