use std::process;

pub enum Error {
    InvalidFlag(String),
    InvalidInput(String),
}

pub fn print_error(error: Error) -> ! {
    match error {
        Error::InvalidFlag(message) => eprintln!("Invalid flag: {message}"),
        Error::InvalidInput(message) => eprintln!("Invalid input: {message}"),
    }
    process::exit(0)
}
