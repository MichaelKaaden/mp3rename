use std::fs;
use std::io;

use walkdir::WalkDir;

use crate::config::Config;

pub mod config;

pub struct DirContents {
    pub dir_entry: walkdir::DirEntry,
    pub music_files: Vec<std::fs::DirEntry>, // contained music files
    pub other_files: Vec<std::fs::DirEntry>, // contained other files (potentially being deleted)
}

/// Returns the list of directories.
pub fn get_list_of_dirs(config: &Config) -> Vec<walkdir::DirEntry> {
    WalkDir::new(&config.start_dir)
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir())
        // cannot print warnings
        //.filter_map(Result::ok)
        // filter and report errors
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

/// Returns directories containing music files
pub fn get_dirs_with_music(dirs: Vec<walkdir::DirEntry>) -> io::Result<Vec<DirContents>> {
    let mut dir_contents = vec![];

    for dir in dirs {
        if dir.file_type().is_dir() {
            let (music, others): (
                Vec<Result<std::fs::DirEntry, _>>,
                Vec<Result<std::fs::DirEntry, _>>,
            ) = fs::read_dir(dir.path())?
                .filter(|dir_entry| dir_entry.as_ref().unwrap().path().is_file())
                .partition(|dir_entry| {
                    let entry = dir_entry.as_ref().unwrap();
                    is_music_file(&entry)
                });
            // only return directories containing music files
            if music.len() > 0 {
                dir_contents.push(DirContents {
                    dir_entry: dir,
                    music_files: music.into_iter().map(|m| m.unwrap()).collect(),
                    other_files: others.into_iter().map(|o| o.unwrap()).collect(),
                });
            }
        }
    }

    Ok(dir_contents)
}

fn is_music_file(entry: &std::fs::DirEntry) -> bool {
    let path = entry.path();
    let file_name = path.to_str();
    match file_name {
        None => false,
        Some(file_name) => is_music_filename(file_name),
    }
}

fn is_music_filename(file_name: &str) -> bool {
    file_name.to_lowercase().ends_with(".mp3")
        || file_name.to_lowercase().ends_with(".flac")
        || file_name.to_lowercase().ends_with(".m4a")
        || file_name.to_lowercase().ends_with(".m4b")
        || file_name.to_lowercase().ends_with(".m4p")
        || file_name.to_lowercase().ends_with(".m4v")
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
}
