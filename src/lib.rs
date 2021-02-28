use walkdir::WalkDir;

use crate::config::Config;

pub mod config;

pub fn traverse_dirs(config: &Config) {
    for entry in WalkDir::new(&config.start_dir)
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir())
    {
        match entry {
            Ok(e) => println!("{}", e.path().display()),
            Err(e) => eprintln!("{}", e),
        };
    }
}
