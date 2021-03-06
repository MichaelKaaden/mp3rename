use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use crate::config::Config;
use crate::music_file::MusicFile;
use crate::ordinary_file::OrdinaryFile;

pub mod config;
mod music_file;
mod music_metadata;
mod ordinary_file;
mod util;

pub fn rename_music_files(config: &Config) {
    let all_files_and_directories = util::get_list_of_dirs(config);

    // iterate over directories containing at least one music file
    for dir in all_files_and_directories {
        if dir.file_type().is_dir() {
            if let Ok(readdir) = fs::read_dir(dir.path()) {
                let (music, others): (Vec<fs::DirEntry>, Vec<fs::DirEntry>) = readdir
                    .filter(|dir_entry| dir_entry.as_ref().unwrap().path().is_file())
                    .map(|dir_entry| dir_entry.unwrap())
                    .partition(util::is_music_file);

                // only use directories containing music files
                if !music.is_empty() {
                    let mut music_files: Vec<MusicFile> = music
                        .into_iter()
                        .map(MusicFile::new)
                        .filter(|music_file| music_file.music_metadata.is_some())
                        .collect();
                    // by now we can be sure all music_files *have* metadata, else we would have filtered them out above
                    music_files.sort_by(MusicFile::sort_func);

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

    let same_artist = music_file::same_artists(&music_files);
    if config.verbose {
        println!("Same artist: {}", same_artist);
    }
    let same_album_title = music_file::same_album_title(&music_files);

    // partition music files by an Option of their disk number to be able to
    // zero-pad the track numbers individually per *disk* instead of per *directory*
    let mut music_files_by_disk_number_map: HashMap<Option<u16>, Vec<MusicFile>> = HashMap::new();
    for music_file in music_files {
        if let Some(music_metadata) = &music_file.music_metadata {
            let vec = music_files_by_disk_number_map
                .entry(music_metadata.disk_number)
                .or_insert_with(Vec::new);
            vec.push(music_file);
        }
    }

    let number_of_digits_for_disc_number =
        match music_file::largest_disc_number(&music_files_by_disk_number_map) {
            None => 0,
            Some(number) => number.to_string().len(),
        };

    // Getting keys out of HashMaps is unstable. To always produce the same output,
    // we need to add stability by sorting the keys and producing the output according
    // to this order.
    let mut sorted_keys: Vec<&Option<u16>> =
        music_files_by_disk_number_map.keys().into_iter().collect();
    sorted_keys.sort_by(MusicFile::sort_by_disk_number);

    // rename music files
    for disk_number in sorted_keys {
        if let Some(music_files_by_disk_number) = music_files_by_disk_number_map.get(disk_number) {
            for music_file in music_files_by_disk_number {
                match music_file.canonical_name(
                    config,
                    same_artist,
                    number_of_digits_for_disc_number,
                    music_files_by_disk_number.len(),
                ) {
                    Some(canonical_name) => {
                        if config.verbose {
                            println!("Canonical name: {}", canonical_name);
                        }
                        rename_file_or_directory(
                            music_file.dir_entry.path(),
                            config,
                            &canonical_name,
                        )
                    }
                    None => eprintln!("Couldn't retrieve canonical name"),
                }
            }
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
    if let Some(album_title) = same_album_title {
        if config.verbose {
            println!("Same album title: {}", album_title);
        }
        if config.rename_directory {
            rename_file_or_directory(dir_entry.path().to_path_buf(), config, &album_title)
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
    let (extension, _): (String, usize) = util::get_extension(&old_path);
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
