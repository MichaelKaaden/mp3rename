use crate::config::Config;
use crate::dir_contents::DirContents;

pub mod config;
pub mod dir_contents;
mod music_file;
mod music_metadata;
mod ordinary_file;

pub fn rename_music_files(config: &Config) {
    let contents = DirContents::new(&config);
    for dir in contents {
        handle_directory(&dir, config);
    }
}

fn handle_directory(dir: &DirContents, config: &Config) {
    println!("Directory: {}", dir.dir_entry.path().to_string_lossy());

    let same_artist = dir.same_artists();
    println!("Same Artist: {}", same_artist);

    for music_file in &dir.music_files {
        match music_file.canonical_name(config, same_artist) {
            Some(m) => println!("Canonical name: {}", m),
            None => eprintln!("Couldn't retrieve canonical name"),
        }
    }
    //println!("{}", dir);
}
