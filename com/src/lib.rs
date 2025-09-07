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
    InvalidData(String),
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
    CorruptedData,
    NotInitialized,
    AlreadyInitialized,
}

pub fn parse_error(error: Error) -> String {
    match error {
        Error::InvalidFlag(message) => format!("Invalid flag: {message}"),
        Error::InvalidInput(message) => format!("Invalid input: {message}"),
        Error::InvalidData(message) => format!("Invalid data: {message}"),
        Error::InvalidPath(message) => format!("Invalid path: {message}"),
        Error::FileNotFound(message) => format!("Not found: {message}"),
        Error::FileAlreadyExists(message) => format!("Already exists: {message}"),
        Error::DirectoryNotFound(message) => format!("Not found: {message}"),
        Error::DirectoryAlreadyExists(message) => format!("Already exists: {message}"),
        Error::NotADirectory(message) => format!("Not a directory: {message}"),
        Error::NotAFile(message) => format!("Not a file: {message}"),
        Error::NotEmpty(message) => format!("Not empty: {message}"),
        Error::PermissionDenied(message) => format!("Permission denied: {message}"),
        Error::ReadOnlyFilesystem(message) => format!("Read-only filesystem: {message}"),
        Error::StorageFull => format!("Storage full"),
        Error::Interrupted => format!("Interrupted"),
        Error::DBLocked => format!("Database locked"),
        Error::FSUnknown(message) => format!("Unknown error: {message}"),
        Error::CorruptedData => format!("Invalid data from DB"),
        Error::NotInitialized => {
            format!("The app is not yet initialized. Please run `init` command first.")
        }
        Error::AlreadyInitialized => {
            format!("The app is already initialized. Re-initializing will reset your vault.")
        }
    }
}
pub fn print_error(error: Error) -> ! {
    let error_str = parse_error(error);
    eprintln!("{}", error_str);
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
        _ => Error::FSUnknown(format!(
            "Failed to create directory '{}': {} (OS error: {:?})",
            path.display(),
            error,
            error.raw_os_error()
        )),
    }
}
