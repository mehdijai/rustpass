use std::{io::ErrorKind, path::PathBuf, process};
#[macro_export]
/// ## Dispatch import macro
/// import module and re-export them all.
macro_rules! dis_import {
    ($arg:ident) => {
        mod $arg;
        pub use $arg::*;
    };
}

pub enum Error {
    InvalidFlag(String),
    InvalidInput(String),
    InvalidPath(String),
    FileNotFound(String),
    FileAlreadyExists(String),
    DirectoryNotFound(String),
    DirectoryAlreadyExists(String),
    NotADirectory(String),
    NotAFile(String),
    NotEmpty(String),
    PermissionDenied(String),
    ReadOnlyFilesystem(String),
    StorageFull,
    Interrupted,
    DBLocked,
    FSUnknown(String),
}

pub fn print_error(error: Error) -> ! {
    match error {
        Error::InvalidFlag(message) => eprintln!("Invalid flag: {message}"),
        Error::InvalidInput(message) => eprintln!("Invalid input: {message}"),
        Error::InvalidPath(message) => eprintln!("Invalid path: {message}"),
        Error::FileNotFound(message) => eprintln!("Not found: {message}"),
        Error::FileAlreadyExists(message) => eprintln!("Already exists: {message}"),
        Error::DirectoryNotFound(message) => eprintln!("Not found: {message}"),
        Error::DirectoryAlreadyExists(message) => eprintln!("Already exists: {message}"),
        Error::NotADirectory(message) => eprintln!("Not a directory: {message}"),
        Error::NotAFile(message) => eprintln!("Not a file: {message}"),
        Error::NotEmpty(message) => eprintln!("Not empty: {message}"),
        Error::PermissionDenied(message) => eprintln!("Permission denied: {message}"),
        Error::ReadOnlyFilesystem(message) => eprintln!("Read-only filesystem: {message}"),
        Error::StorageFull => eprintln!("Storage full"),
        Error::Interrupted => eprintln!("Interrupted"),
        Error::DBLocked => eprintln!("Database locked"),
        Error::FSUnknown(message) => eprintln!("Unknown error: {message}"),
    }
    process::exit(0)
}

pub fn match_io_error_kind(path: &PathBuf, error: std::io::Error) -> Error {
    match error.kind() {
        ErrorKind::PermissionDenied => Error::PermissionDenied(path.display().to_string()),
        ErrorKind::NotFound => {
            if path.is_dir() {
                Error::DirectoryNotFound(path.display().to_string())
            } else {
                Error::FileNotFound(path.display().to_string())
            }
        }
        ErrorKind::InvalidInput => Error::InvalidPath(path.display().to_string()),
        ErrorKind::ReadOnlyFilesystem => Error::ReadOnlyFilesystem(path.display().to_string()),
        ErrorKind::StorageFull => Error::StorageFull,
        ErrorKind::Interrupted => Error::Interrupted,
        _ => {
            // Fallback for any other error kinds
            Error::FSUnknown(format!(
                "Failed to create directory '{}': {} (OS error: {:?})",
                path.display(),
                error,
                error.raw_os_error()
            ))
        }
    }
}
