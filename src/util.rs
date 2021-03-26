use std::path::{Path, PathBuf};
use std::{cmp, fs};

use regex::Regex;
use walkdir::WalkDir;

use crate::config::Config;

/// Returns the list of directories.
pub fn get_list_of_dirs(config: &Config) -> Vec<walkdir::DirEntry> {
    WalkDir::new(&config.start_dir)
        .contents_first(true)
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir())
        // filter out errors (cannot print warnings!)
        //.filter_map(Result::ok)
        // filter *and* report errors
        .filter(|e| match e {
            Ok(_) => true,
            Err(err) => {
                eprintln!("Error traversing directories: {}", err);
                false
            }
        })
        // convert to DirEntry
        .map(|e| e.unwrap())
        .collect()
}

pub fn is_music_file(entry: &fs::DirEntry) -> bool {
    let path = entry.path();
    let file_name = path.to_str();
    match file_name {
        None => false,
        Some(file_name) => is_music_filename(file_name),
    }
}

/// Checks if a name's extension is in a list of music file extensions
pub fn is_music_filename(file_name: &str) -> bool {
    let music_extensions = vec![".mp3", ".flac", ".m4a", ".m4b", ".m4p", ".m4v"];
    let file_name = file_name.to_lowercase();
    for ext in music_extensions {
        if file_name.ends_with(ext) {
            return true;
        }
    }

    false
}

pub fn sanitize_file_or_directory_name(filename: &str) -> String {
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

/// Shortens a file name so that it (together with the extension) fits in a given length
/// Combines the path's extension with the stem from the name.
pub fn shorten_names(path: &Path, name: &str, config: &Config) -> String {
    let (extension, extension_len): (String, usize) = get_extension(path);
    let stem = get_name_stem(name, &extension);

    let len = cmp::min(
        cmp::max((config.name_length as i32) - (extension_len as i32), 0) as usize, // get a positive number
        stem.len(),
    );

    // trim to not have a blank before the extension
    format!("{}{}", &stem[..len].trim(), extension)
}

/// Returns the path's extension with leading dot (or the empty string)
pub fn get_extension(path: &Path) -> (String, usize) {
    if let Some(ext_without_dot) = path.extension() {
        let ext = format!(
            ".{}",
            ext_without_dot.to_string_lossy().to_string().to_lowercase()
        );
        if is_music_filename(&ext) {
            let len = ext.len();
            return (ext, len);
        }
    }
    ("".to_string(), 0)
}

/// Returns the file name's stem, i. e. the name without the extension given as second argument
pub fn get_name_stem(name: &str, extension: &str) -> String {
    name.replace(&extension, "")
}

/// Returns a path made of the given string slice
pub fn string_to_path(file_name: &str) -> std::io::Result<PathBuf> {
    fs::canonicalize(PathBuf::from(file_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_music_filename() {
        assert_eq!(is_music_filename("/tmp/music.mp3"), true);
        assert_eq!(is_music_filename("/tmp/music.mp33"), false);
        assert_eq!(is_music_filename("/tmp/music.Mp3"), true);
        assert_eq!(is_music_filename("/tmp/music.FlAc"), true);
        assert_eq!(is_music_filename("/tmp/music.m4a"), true);
        assert_eq!(is_music_filename("/tmp/music.m4p"), true);
        assert_eq!(is_music_filename("/tmp/music.m4v"), true);
        assert_eq!(is_music_filename("/tmp/music.mp4"), false);
    }

    #[test]
    fn test_sanitize_file_or_directory_name() {
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
    fn test_shorten_names() {
        let config = Config {
            name_length: 8,
            ..Config::default()
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
                    name_length: 9,
                    ..Config::default()
                },
            ),
            "foo b.mp3"
        );
        assert_eq!(
            shorten_names(
                &PathBuf::from("/foo/bar.mp3"),
                "foo bar.mp3",
                &Config {
                    name_length: 1,
                    ..Config::default()
                },
            ),
            ".mp3"
        );
        assert_eq!(
            shorten_names(
                &PathBuf::from("/foo/bar.mp3"),
                "foo bar.mp3",
                &Config {
                    name_length: 4,
                    ..Config::default()
                },
            ),
            ".mp3"
        );
        assert_eq!(
            shorten_names(
                &PathBuf::from("/foo/bar.mp3"),
                "foo bar.mp3",
                &Config {
                    name_length: 5,
                    ..Config::default()
                },
            ),
            "f.mp3"
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

    #[test]
    fn test_get_extension() {
        assert_eq!(
            get_extension(&PathBuf::from("Foo Bar")),
            (String::from(""), 0),
            "wrong handling of names without extension"
        );
        assert_eq!(
            get_extension(&PathBuf::from("Titan A.E.")),
            (String::from(""), 0),
            "wrong handling of names without music file extension"
        );
        assert_eq!(get_extension(&PathBuf::from("E.T.")), (String::from(""), 0));
        assert_eq!(
            get_extension(&PathBuf::from("Titan.flac")),
            (String::from(".flac"), 5),
            "wrong handling of names with extension"
        );
    }

    #[test]
    fn test_get_name_stem() {
        assert_eq!(get_name_stem("foo.mp3", ".mp3"), "foo");
        assert_eq!(get_name_stem("foo.mp3", "mp3"), "foo.");
        assert_eq!(get_name_stem("foo.mp3", ""), "foo.mp3");
    }
}
