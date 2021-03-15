use std::ffi::OsString;
use std::fs;

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
        println!("==============");
        handle_directory(&dir, config);
    }
}

fn handle_directory(dir: &DirContents, config: &Config) {
    println!(
        "Handling directory \"{}\"",
        dir.dir_entry.path().to_string_lossy()
    );

    let same_artist = dir.same_artists();
    println!("Same artist:      {}", same_artist);
    let same_album_title = dir.same_album_title();
    if let Some(album_title) = same_album_title {
        println!("Same album title: {}", album_title);
        if config.rename_directory {
            let old_path = dir.dir_entry.path();
            let old_dir_name = old_path
                .file_name()
                .unwrap_or_else(|| {
                    panic!(
                        "Cannot retrieve file name from {}",
                        old_path.to_string_lossy()
                    )
                })
                .to_string_lossy();
            let new_path = old_path.with_file_name(OsString::from(album_title));
            let new_dir_name = new_path
                .file_name()
                .unwrap_or_else(|| {
                    panic!(
                        "Cannot retrieve file name from {}",
                        new_path.to_string_lossy()
                    )
                })
                .to_string_lossy();
            println!("Renaming \"{}\" to \"{}\"", old_dir_name, new_dir_name);
            if !config.dry_run {
                if let Err(e) = fs::rename(old_path, new_path) {
                    eprintln!("Error renaming \"{}\": {}", old_dir_name, e);
                }
            }
        }
    } else {
        println!("Multiple album names.")
    }

    for music_file in &dir.music_files {
        match music_file.canonical_name(config, same_artist, dir.music_files.len()) {
            Some(m) => println!("Canonical name: {}", m),
            None => eprintln!("Couldn't retrieve canonical name"),
        }
    }
    //println!("{}", dir);
}
