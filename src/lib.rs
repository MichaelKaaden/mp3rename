use walkdir::WalkDir;

use crate::config::Config;

pub mod config;

pub fn traverse_dirs(config: Config) {
    for entry in WalkDir::new(config.start_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        println!("{}", entry.path().display());
    }
}
