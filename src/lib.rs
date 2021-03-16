use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use regex::Regex;

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
    println!("==============");
    println!(
        "Entering directory \"{}\"",
        dir.dir_entry.path().to_string_lossy()
    );

    let same_artist = dir.same_artists();
    if config.verbose {
        println!("Same artist: {}", same_artist);
    }

    // rename music files
    for music_file in &dir.music_files {
        match music_file.canonical_name(config, same_artist, dir.music_files.len()) {
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
    if dir.ordinary_files.len() > 0 && config.remove_ordinary_files {
        for file in &dir.ordinary_files {
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
    let same_album_title = dir.same_album_title();
    if let Some(album_title) = same_album_title {
        if config.verbose {
            println!("Same album title: {}", album_title);
        }
        if config.rename_directory {
            rename_file_or_directory(dir.dir_entry.path().to_path_buf(), config, album_title)
        }
    } else {
        if config.verbose {
            println!("Multiple album names.")
        }
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

    let new_path = old_path.with_file_name(OsString::from(to_name));
    let new_name = sanitize_file_or_directory_name(
        &new_path
            .file_name()
            .unwrap_or_else(|| {
                panic!(
                    "Cannot retrieve name part from {}",
                    new_path.to_string_lossy()
                )
            })
            .to_string_lossy(),
        config,
    );

    if old_name == new_name {
        return;
    }

    println!("Renaming \"{}\" to \"{}\"", old_name, new_name);

    if !config.dry_run {
        if let Err(e) = fs::rename(&old_path, new_path) {
            eprintln!("Error renaming \"{}\": {}", old_name, e);
        }
    }
}

fn sanitize_file_or_directory_name(filename: &str, config: &Config) -> String {
    let mut name = filename.trim().replace("\\", "/");
    name = name.replace("|", ", ");
    name = name.replace("/", "&");
    name = name.replace("???", "Fragezeichen");
    name = name.replace("?", "");
    name = name.replace("\"", "");
    name = name.replace("*", "_");
    name = name.replace("$", "_");

    if config.use_fatfs_names {
        name = name.replace(":", " -");
    }

    // replace multiple dots at the start with nothing
    let re = Regex::new(r"^[.]*").unwrap();
    name = re.replace_all(&name, "").to_string();

    // replace multiple dots at the end with nothing
    let re = Regex::new(r"[.]*$").unwrap();
    name = re.replace_all(&name, "").to_string();

    // replace multiple blanks with one blank
    let re = Regex::new(r"\s+").unwrap();
    name = re.replace_all(&name, " ").to_string();

    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitization() {
        let default_config = Config {
            dry_run: false,
            name_length: 0,
            remove_artist: false,
            remove_ordinary_files: false,
            rename_directory: false,
            shorten_names: false,
            start_dir: Default::default(),
            use_fatfs_names: false,
            verbose: false,
        };

        assert_eq!(
            sanitize_file_or_directory_name("foo\\bar", &default_config),
            "foo&bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name("foo|bar", &default_config),
            "foo, bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name("foo/bar", &default_config),
            "foo&bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name("foo\tbar", &default_config),
            "foo bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name("foo   bar", &default_config),
            "foo bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name("foo \t \t bar", &default_config),
            "foo bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name("foo ??? bar", &default_config),
            "foo Fragezeichen bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name("foo ? bar", &default_config),
            "foo bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name("?foo bar?", &default_config),
            "foo bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name("\"foo bar\"", &default_config),
            "foo bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name("*foo ** bar*", &default_config),
            "_foo __ bar_"
        );
        assert_eq!(
            sanitize_file_or_directory_name("$foo $$ bar$", &default_config),
            "_foo __ bar_"
        );
        assert_eq!(
            sanitize_file_or_directory_name("...foo bar", &default_config),
            "foo bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name(".foo bar", &default_config),
            "foo bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name("foo bar...", &default_config),
            "foo bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name("foo bar.", &default_config),
            "foo bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name("foo: bar", &default_config),
            "foo: bar"
        );
        assert_eq!(
            sanitize_file_or_directory_name(
                "foo: bar",
                &Config {
                    use_fatfs_names: true,
                    ..default_config
                }
            ),
            "foo - bar"
        );
    }
}
