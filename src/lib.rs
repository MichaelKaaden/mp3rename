use walkdir::{DirEntry, WalkDir};

use crate::config::Config;

pub mod config;

/// Returns the list of directories.
pub fn get_list_of_dirs(config: &Config) -> Vec<DirEntry> {
    WalkDir::new(&config.start_dir)
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir())
        // filter and report errors
        .filter(|e| match e {
            Ok(_) => true,
            Err(err) => {
                eprintln!("Error traversing directories: {}", err);
                false
            }
        })
        // convert to DirEntry
        .map(|e| e.unwrap())
        .collect()
}
