use std::path::PathBuf;
use std::{cmp, fs};

use regex::Regex;

use crate::config::Config;

pub fn is_music_file(entry: &fs::DirEntry) -> bool {
    let path = entry.path();
    let file_name = path.to_str();
    match file_name {
        None => false,
        Some(file_name) => is_music_filename(file_name),
    }
}

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

pub fn shorten_names(new_path: &PathBuf, new_name: &str, config: &Config) -> String {
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
pub fn get_extension(path: &PathBuf) -> (String, i32) {
    if let Some(ext_without_dot) = path.extension() {
        let ext = format!(
            ".{}",
            ext_without_dot.to_string_lossy().to_string().to_lowercase()
        );
        if is_music_filename(&ext) {
            let len: i32 = ext.len() as i32;
            return (ext, len);
        }
    }
    ("".to_string(), 0)
}

/// Returns the file name's stem, i. e. the name without the extension given as second argument
pub fn get_name_stem(name: &str, extension: &str) -> String {
    name.replace(&extension, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_music() {
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
