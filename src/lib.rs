use crate::config::Config;
use crate::dir_contents::DirContents;

pub mod config;
pub mod dir_contents;
mod music_file;
mod music_metadata;
mod ordinary_file;

pub fn rename(config: &Config) {
    let contents = DirContents::new(&config);
    check_artists(contents);
}

fn check_artists(dir_contents: Vec<DirContents>) {
    for dir in dir_contents {
        let same_artist = dir.same_artists();
        println!("Same Artist: {}", same_artist);
        println!("{}", dir);
    }
}
