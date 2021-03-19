use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use crate::config::Config;
use crate::music_file::MusicFile;
use crate::ordinary_file::OrdinaryFile;

pub mod config;
pub mod dir_contents;
mod music_file;
mod music_metadata;
mod ordinary_file;
mod util;

pub fn rename_music_files(config: &Config) {
    let all_files_and_directories = util::get_list_of_dirs(&config);

    // iterate over directories containing at least one music file
    for dir in all_files_and_directories {
        if dir.file_type().is_dir() {
            if let Ok(readdir) = fs::read_dir(dir.path()) {
                let (music, others): (Vec<fs::DirEntry>, Vec<fs::DirEntry>) = readdir
                    .filter(|dir_entry| dir_entry.as_ref().unwrap().path().is_file())
                    .map(|dir_entry| dir_entry.unwrap())
                    .partition(|dir_entry| util::is_music_file(dir_entry));

                // only use directories containing music files
                if !music.is_empty() {
                    let mut music_files: Vec<MusicFile> = music
                        .into_iter()
                        .map(MusicFile::new)
                        .filter(|music_file| music_file.music_metadata.is_some())
                        .collect();
                    music_files.sort_by(|left, right| MusicFile::sort_func(left, right));

                    let ordinary_files: Vec<OrdinaryFile> =
                        others.into_iter().map(OrdinaryFile::new).collect();

                    handle_directory(dir, music_files, ordinary_files, config);
                }
            }
        }
    }
}

fn handle_directory(
    dir_entry: walkdir::DirEntry,
    music_files: Vec<MusicFile>,
    ordinary_files: Vec<OrdinaryFile>,
    config: &Config,
) {
    println!("==============");
    println!(
        "Entering directory \"{}\"",
        dir_entry.path().to_string_lossy()
    );

    let same_artist = dir_contents::same_artists(&music_files);
    if config.verbose {
        println!("Same artist: {}", same_artist);
    }

    // rename music files
    for music_file in &music_files {
        match music_file.canonical_name(config, same_artist, music_files.len()) {
            Some(canonical_name) => {
                if config.verbose {
                    println!("Canonical name: {}", canonical_name);
                }
                rename_file_or_directory(music_file.dir_entry.path(), config, &canonical_name)
            }
            None => eprintln!("Couldn't retrieve canonical name"),
        }
    }

    // remove ordinary files
    if !ordinary_files.is_empty() && config.remove_ordinary_files {
        for file in &ordinary_files {
            println!("Removing {}", file.dir_entry.path().to_string_lossy());
            if !config.dry_run {
                if let Err(err) = fs::remove_file(file.dir_entry.path()) {
                    eprintln!(
                        "Couldn't remove {}: {}",
                        file.dir_entry.path().to_string_lossy(),
                        err
                    );
                };
            }
        }
    }

    // rename the directory
    let same_album_title = dir_contents::same_album_title(&music_files);
    if let Some(album_title) = same_album_title {
        if config.verbose {
            println!("Same album title: {}", album_title);
        }
        if config.rename_directory {
            rename_file_or_directory(dir_entry.path().to_path_buf(), config, album_title)
        }
    } else if config.verbose {
        println!("Multiple album names.")
    }
}

/// Rename a file or directory name in a path
fn rename_file_or_directory(old_path: PathBuf, config: &Config, to_name: &str) {
    let old_name = old_path
        .file_name()
        .unwrap_or_else(|| {
            panic!(
                "Cannot retrieve name part from {}",
                old_path.to_string_lossy()
            )
        })
        .to_string_lossy();

    // sanitize the canonical name *without* extension to catch cases like
    // "Foo....mp3" which should become "Foo.mp3"
    let (extension, _): (String, i32) = util::get_extension(&old_path);
    let mut short_name_stem = util::get_name_stem(to_name, &extension); // both parameters use lowercase for the extension
    short_name_stem = util::sanitize_file_or_directory_name(&short_name_stem);

    // now rebuild the name *with* the extension to be able to shorten the canonical name
    let mut to_name = format!("{}{}", short_name_stem, extension);
    if config.shorten_names {
        to_name = util::shorten_names(&old_path, &to_name, config);
    }

    let new_path = old_path.with_file_name(OsString::from(to_name));
    let new_name = &new_path
        .file_name()
        .unwrap_or_else(|| {
            panic!(
                "Cannot retrieve name part from {}",
                new_path.to_string_lossy()
            )
        })
        .to_string_lossy();

    if old_name.eq(new_name) {
        return;
    }

    println!("Renaming \"{}\" to \"{}\"", old_name, new_name);

    if !config.dry_run {
        if let Err(e) = fs::rename(&old_path, new_path) {
            eprintln!("Error renaming \"{}\": {}", old_name, e);
        }
    }
}
