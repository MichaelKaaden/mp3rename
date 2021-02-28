use walkdir::{DirEntry, Error, WalkDir};

use crate::config::Config;

pub mod config;

pub fn get_list_of_dirs(config: &Config) -> Vec<Result<DirEntry, Error>> {
    WalkDir::new(&config.start_dir)
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir())
        .filter(|e| match e {
            Ok(_) => true,
            Err(err) => {
                eprintln!("Error traversing directories: {}", err);
                false
            }
        })
        .collect()
}
