use std::ffi::OsString;
use std::path::PathBuf;
use std::{cmp, fs};

use regex::Regex;

use crate::config::Config;
use crate::dir_contents::DirContents;

pub mod config;
pub mod dir_contents;
mod music_file;
mod music_metadata;
mod ordinary_file;
mod util;

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

    // sanitize the canonical name *without* extension to catch cases like
    // "Foo....mp3" which should become "Foo.mp3"
    let (extension, _): (String, i32) = get_extension(&old_path);
    let mut short_name_stem = get_name_stem(to_name, &extension); // both parameters use lowercase for the extension
    short_name_stem = sanitize_file_or_directory_name(&short_name_stem);

    // now rebuild the name *with* the extension to be able to shorten the canonical name
    let mut to_name = format!("{}{}", short_name_stem, extension);
    if config.shorten_names {
        to_name = shorten_names(&old_path, &to_name, config);
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

fn sanitize_file_or_directory_name(filename: &str) -> String {
    let mut name = filename.replace("$", "_");
    name = name.replace("???", "Fragezeichen");

    // Windows doesn't like any of the following characters
    name = name.replace("\\", "");
    name = name.replace("/", " & ");
    name = name.replace(":", " -");
    name = name.replace("*", "_");
    name = name.replace("?", "");
    name = name.replace("\"", "");
    name = name.replace("<", "");
    name = name.replace(">", "");
    name = name.replace("|", ", ");

    // now we added blanks, let's handle the ones in the beginning and at the end
    name = name.trim().to_string();

    // remove dots at the start
    let re = Regex::new(r"^[.]*").unwrap();
    name = re.replace_all(&name, "").to_string();

    // remove dots at the end
    let re = Regex::new(r"[.]*$").unwrap();
    name = re.replace_all(&name, "").to_string();

    // replace whitespace with only one blank each
    let re = Regex::new(r"\s+").unwrap();
    name = re.replace_all(&name, " ").to_string();

    // remove any blanks at the name's start or end
    name.trim().to_string()
}

fn shorten_names(new_path: &PathBuf, new_name: &str, config: &Config) -> String {
    let (extension, extension_len): (String, i32) = get_extension(new_path);
    let short_name_stem = get_name_stem(new_name, &extension);

    let len = cmp::min(
        cmp::min(
            cmp::max((config.name_length as i32) - extension_len, 0) as u32, // get a positive number
            config.name_length,
        ) as usize,
        short_name_stem.len(),
    );

    // trim to not have a blank before the extension
    format!("{}{}", &short_name_stem[..len].trim(), extension)
}

/// Returns the path's extension (or the empty string)
fn get_extension(path: &PathBuf) -> (String, i32) {
    if let Some(ext_without_dot) = path.extension() {
        let ext = format!(
            ".{}",
            ext_without_dot.to_string_lossy().to_string().to_lowercase()
        );
        if util::is_music_filename(&ext) {
            let len: i32 = ext.len() as i32;
            return (ext, len);
        }
    }
    ("".to_string(), 0)
}

/// Returns the file name's stem, i. e. the name without the extension given as second argument
fn get_name_stem(name: &str, extension: &str) -> String {
    name.replace(&extension, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitization() {
        assert_eq!(
            sanitize_file_or_directory_name("$foo $$ bar$"),
            "_foo __ bar_"
        );

        // special handling for "The Three ???"
        assert_eq!(
            sanitize_file_or_directory_name("foo ??? bar"),
            "foo Fragezeichen bar"
        );

        assert_eq!(sanitize_file_or_directory_name("foo\\bar"), "foobar");

        assert_eq!(sanitize_file_or_directory_name("foo/bar"), "foo & bar");
        assert_eq!(sanitize_file_or_directory_name("foo/"), "foo &");

        assert_eq!(sanitize_file_or_directory_name("foo: bar"), "foo - bar");
        assert_eq!(sanitize_file_or_directory_name("foo:"), "foo -");

        assert_eq!(
            sanitize_file_or_directory_name("*foo * bar*"),
            "_foo _ bar_"
        );
        assert_eq!(
            sanitize_file_or_directory_name("*foo ** bar*"),
            "_foo __ bar_"
        );

        assert_eq!(sanitize_file_or_directory_name("foo ? bar"), "foo bar");
        assert_eq!(sanitize_file_or_directory_name("?foo bar?"), "foo bar");

        assert_eq!(sanitize_file_or_directory_name("\"foo bar\""), "foo bar");

        assert_eq!(sanitize_file_or_directory_name("<foo bar>"), "foo bar");

        assert_eq!(sanitize_file_or_directory_name("foo|bar"), "foo, bar");

        // whitespace
        assert_eq!(sanitize_file_or_directory_name("foo\tbar"), "foo bar");
        assert_eq!(sanitize_file_or_directory_name("foo   bar"), "foo bar");
        assert_eq!(sanitize_file_or_directory_name("foo \t \t bar"), "foo bar");
        assert_eq!(sanitize_file_or_directory_name(" foo bar "), "foo bar");

        // leading and trailing dots
        assert_eq!(sanitize_file_or_directory_name("...foo bar"), "foo bar");
        assert_eq!(sanitize_file_or_directory_name(".foo bar"), "foo bar");
        assert_eq!(sanitize_file_or_directory_name("foo bar..."), "foo bar");
        assert_eq!(sanitize_file_or_directory_name("foo bar."), "foo bar");

        // example with french punctuation marks
        assert_eq!(
            sanitize_file_or_directory_name("Où est le bien ? Où est le mal ?"),
            "Où est le bien Où est le mal"
        );
    }

    #[test]
    fn name_shortening() {
        let config = Config {
            dry_run: false,
            name_length: 8,
            remove_artist: false,
            remove_ordinary_files: false,
            rename_directory: false,
            shorten_names: false,
            start_dir: Default::default(),
            verbose: false,
        };

        assert_eq!(
            shorten_names(&PathBuf::from("/foo/bar.mp3"), "foo.mp3", &config),
            "foo.mp3"
        );
        assert_eq!(
            shorten_names(&PathBuf::from("/foo/bar.flac"), "123456789.flac", &config),
            "123.flac"
        );
        assert_eq!(
            shorten_names(&PathBuf::from("/foo/bar.mp3"), "foo bar.mp3", &config),
            "foo.mp3"
        );
        assert_eq!(
            shorten_names(
                &PathBuf::from("/foo/bar.mp3"),
                "foo bar.mp3",
                &Config {
                    dry_run: false,
                    name_length: 9,
                    remove_artist: false,
                    remove_ordinary_files: false,
                    rename_directory: false,
                    shorten_names: false,
                    start_dir: Default::default(),
                    verbose: false,
                },
            ),
            "foo b.mp3"
        );
        assert_eq!(
            shorten_names(&PathBuf::from("/foo/bar.mp3"), "foo bar.blah.mp3", &config),
            "foo.mp3"
        );
        assert_eq!(
            shorten_names(&PathBuf::from("/foo/bar"), "foo bar", &config),
            "foo bar"
        );
        assert_eq!(
            shorten_names(&PathBuf::from("/foo/bar"), "foo bar   ", &config),
            "foo bar"
        );
        assert_eq!(
            shorten_names(&PathBuf::from("/foo/bar"), "123456789", &config),
            "12345678"
        );
    }
}
