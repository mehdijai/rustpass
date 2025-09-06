use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
    process,
};

use crate::{get_db_path, handle_io_errors};

pub struct DB {
    path: PathBuf,
    lock_path: PathBuf,
    file: File,
}

impl DB {
    pub fn new() -> Self {
        let path = get_db_path();
        let lock_path = path.with_extension("lock");

        // Check if already locked
        if lock_path.exists() {
            com::print_error(com::Error::DBLocked);
        }

        // Create lock file with current process ID
        let pid = process::id().to_string();
        let lock_result = fs::write(&lock_path, pid);
        handle_io_errors(&path, lock_result);

        let file = OpenOptions::new().read(true).write(true).open(&path);
        let file = handle_io_errors(&path, file);
        let lock_result = file.lock();
        handle_io_errors(&path, lock_result);
        DB {
            path,
            file,
            lock_path,
        }
    }

    pub fn close(&self) {
        self.remove_lock_file();
    }

    fn remove_lock_file(&self) {
        if self.lock_path.exists() {
            let result = fs::remove_file(&self.lock_path);
            handle_io_errors(&self.path, result);
        }
    }

    fn handle_seek_to_start(&mut self) {
        let seek_result = self.file.seek(SeekFrom::Start(0));
        handle_io_errors(&self.path, seek_result);
    }

    /// ### Get the whole content
    pub fn get(&mut self) -> String {
        let mut content = String::new();

        self.handle_seek_to_start();

        let result = self.file.read_to_string(&mut content);
        handle_io_errors(&self.path, result);
        content
    }
    /// ### Overwrite data
    pub fn write(&mut self, data: &str) {
        self.handle_seek_to_start();
        let truncate_result = self.file.set_len(0);
        handle_io_errors(&self.path, truncate_result);

        let result = self.file.write_all(data.as_bytes());
        handle_io_errors(&self.path, result);

        let flush_result = self.file.flush();
        handle_io_errors(&self.path, flush_result);
    }

    pub fn clear(&mut self) {
        self.handle_seek_to_start();
        let truncate_result = self.file.set_len(0);
        handle_io_errors(&self.path, truncate_result);

        let flush_result = self.file.flush();
        handle_io_errors(&self.path, flush_result);
    }
    /// ### Append data
    pub fn append(&mut self, data: &str) {
        let seek_results = self.file.seek(SeekFrom::End(0));
        handle_io_errors(&self.path, seek_results);

        let result = self.file.write_all(("\n".to_string() + data).as_bytes());
        handle_io_errors(&self.path, result);

        let flush_results = self.file.flush();
        handle_io_errors(&self.path, flush_results);
    }
}

impl Drop for DB {
    fn drop(&mut self) {
        self.remove_lock_file();
    }
}
