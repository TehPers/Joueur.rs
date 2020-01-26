
extern crate backtrace;

use crate::joueur::color;

use std::string::ToString;
use backtrace::Backtrace;

#[derive(Debug)]
pub enum ErrorCode {
    // None                     = 0,
    // InvalidArgs              = 20,
    CouldNotConnect          = 21,
    // DisconnectedUnexpectedly = 22,
    // CannotReadSocket         = 23,
    // DeltaMergeFailure        = 24,
    // ReflectionFailed         = 25,
    // UnknownEventFromServer   = 26,
    // ServerTimeout            = 27,
    // FatalEvent               = 28,
    // GameNotFound             = 29,
    // MalformedJSON            = 30,
    // Unauthenticated          = 31,
    // AIErrored                = 42,
}

pub fn handle_error(error_code: ErrorCode, err: &dyn std::error::Error, message: &String) -> ! {
    let _ = color::red(&format!("---\nError: {:?}", error_code));
    let _ = color::red(&format!("---\n{}", message));
    let _ = color::red(&format!("---\n{}", err.to_string()));

    let bt = Backtrace::new();
    let _ = color::red(&format!("---\n{:?}", bt));

    std::process::exit(error_code as i32)
}
