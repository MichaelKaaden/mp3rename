use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

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
                .or_default();
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
    let mut sorted_keys: Vec<&Option<u16>> = music_files_by_disk_number_map.keys().collect();
    sorted_keys.sort_by(MusicFile::sort_by_disk_number);

    // determine the new names of all music files *before* renaming any of them
    // to be able to detect files that would end up with the same name
    let mut renames: Vec<(PathBuf, String)> = Vec::new();
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
                        let path = music_file.dir_entry.path();
                        if let Some(new_name) = new_name(&path, config, &canonical_name) {
                            renames.push((path, new_name));
                        }
                    }
                    None => eprintln!("Couldn't retrieve canonical name"),
                }
            }
        }
    }

    // rename music files, but never several files to the same name
    let mut name_counts: HashMap<&str, usize> = HashMap::new();
    for (_, new_name) in &renames {
        *name_counts.entry(new_name).or_default() += 1;
    }
    for (path, new_name) in &renames {
        if name_counts[new_name.as_str()] > 1 {
            eprintln!(
                "Not renaming \"{}\": several files would be named \"{}\"",
                path.to_string_lossy(),
                new_name
            );
        } else {
            rename_file_or_directory(path, config, new_name);
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
            if let Some(new_name) = new_name(dir_entry.path(), config, &album_title) {
                rename_file_or_directory(dir_entry.path(), config, &new_name)
            }
        }
    } else if config.verbose {
        println!("Multiple album names.")
    }
}

/// Returns the sanitized (and maybe shortened) name a file or directory should get,
/// or `None` if nothing would be left of the name but its extension
fn new_name(old_path: &Path, config: &Config, to_name: &str) -> Option<String> {
    // sanitize the canonical name *without* extension to catch cases like
    // "Foo....mp3" which should become "Foo.mp3"
    let (extension, _): (String, usize) = util::get_extension(old_path);
    let mut short_name_stem = util::get_name_stem(to_name, &extension); // both parameters use lowercase for the extension
    short_name_stem = util::sanitize_file_or_directory_name(&short_name_stem);

    // now rebuild the name *with* the extension to be able to shorten the canonical name
    let mut new_name = format!("{}{}", short_name_stem, extension);
    if config.shorten_names {
        new_name = util::shorten_names(old_path, &new_name, config);
    }

    if util::get_name_stem(&new_name, &extension).is_empty() {
        eprintln!(
            "Not renaming \"{}\": the new name would be empty",
            old_path.to_string_lossy()
        );
        return None;
    }

    Some(new_name)
}

/// Rename a file or directory name in a path, but never overwrite another file or directory
fn rename_file_or_directory(old_path: &Path, config: &Config, to_name: &str) {
    let old_name = old_path
        .file_name()
        .unwrap_or_else(|| {
            panic!(
                "Cannot retrieve name part from {}",
                old_path.to_string_lossy()
            )
        })
        .to_string_lossy();

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

    // on case-insensitive file systems, a name differing only in case denotes the same file
    let target_exists = fs::symlink_metadata(&new_path).is_ok();
    if target_exists && !same_file::is_same_file(old_path, &new_path).unwrap_or(false) {
        eprintln!(
            "Not renaming \"{}\": \"{}\" already exists",
            old_name, new_name
        );
        return;
    }

    println!("Renaming \"{}\" to \"{}\"", old_name, new_name);

    if !config.dry_run {
        if let Err(e) = fs::rename(old_path, &new_path) {
            eprintln!("Error renaming \"{}\": {}", old_name, e);
        }
    }
}
