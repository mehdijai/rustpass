use std::{fs::File, io::Error, path::PathBuf};

pub fn get_db_path() -> PathBuf {
    let dir = get_dir();
    create_dir(&dir);

    let path = dir.join("db.pgr");
    create_db_file(&path);

    path
}

fn create_db_file(path: &PathBuf) {
    let file_exists = path.exists();

    if file_exists {
        return;
    }

    let file = File::create(path);

    handle_io_errors(path, file);
}

fn create_dir(path: &PathBuf) {
    let dir_exists = path.exists();

    if !dir_exists {
        let dir = std::fs::create_dir_all(&path);

        match dir {
            Err(error) => com::print_error(com::match_io_error_kind(&path, error)),
            _ => {}
        };
    };
}

fn get_dir() -> PathBuf {
    let root_path = get_home_dir_root();
    root_path.join(".jli").join(".passger")
}

fn get_home_dir_root() -> PathBuf {
    let path = std::env::home_dir();

    match path {
        Some(path) => path,
        None => com::print_error(com::Error::FileNotFound(
            "Cannot find home directory".to_string(),
        )),
    }
}

pub fn handle_io_errors<T>(path: &PathBuf, file: Result<T, Error>) -> T {
    match file {
        Err(error) => com::print_error(com::match_io_error_kind(&path, error)),
        Ok(file) => file,
    }
}
