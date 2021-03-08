use std::fmt::Formatter;
use std::{fmt, fs};

use audiotags;
use walkdir::WalkDir;

use crate::config::Config;

pub mod config;

pub struct DirContents<'a> {
    pub dir_entry: walkdir::DirEntry,
    pub music_files: Vec<std::fs::DirEntry>, // contained music files
    pub music_tags: Vec<Option<MusicTags<'a>>>, // tags read from the music_files
    pub other_files: Vec<std::fs::DirEntry>, // contained other files (potentially being deleted)
}

impl<'a> DirContents<'a> {
    pub fn new(config: &Config) -> Vec<DirContents> {
        let entries = get_list_of_dirs(&config);
        get_dirs_with_music(entries)
    }
}

/// Returns the list of directories.
fn get_list_of_dirs(config: &Config) -> Vec<walkdir::DirEntry> {
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
fn get_dirs_with_music<'a>(dirs: Vec<walkdir::DirEntry>) -> Vec<DirContents<'a>> {
    let mut dir_contents = vec![];

    for dir in dirs {
        if dir.file_type().is_dir() {
            let readdir = fs::read_dir(dir.path());
            if readdir.is_ok() {
                let (music, others): (
                    Vec<Result<std::fs::DirEntry, _>>,
                    Vec<Result<std::fs::DirEntry, _>>,
                ) = readdir
                    .unwrap()
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
                        music_tags: vec![],
                        other_files: others.into_iter().map(|o| o.unwrap()).collect(),
                    });
                }
            }
        }
    }

    dir_contents
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

pub struct MusicTags<'a> {
    pub album: String,
    pub artist: String,
    pub dir_entry: &'a std::fs::DirEntry,
    pub disk_number: Option<u16>,
    pub title: String,
    pub track_number: u16,
}

impl<'a> fmt::Display for MusicTags<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Album:        {}", self.album)?;
        if let Some(disk_number) = self.disk_number {
            writeln!(f, "Disk Number:  {}", disk_number)?;
        }
        writeln!(f, "Track Number: {}", self.track_number)?;
        writeln!(f, "Artist:       {}", self.artist)?;
        writeln!(f, "Title:        {}", self.title)
    }
}

pub fn get_tags(music_file: &std::fs::DirEntry) -> Option<MusicTags> {
    let tag = audiotags::Tag::new()
        .read_from_path(music_file.path())
        .unwrap_or_else(|_| panic!("Could not read \"{}\"", music_file.path().to_string_lossy()));
    let mut result: Option<MusicTags> = None;

    if let Some(album) = tag.album_title() {
        if let Some(artist) = tag.artist() {
            if let Some(title) = tag.title() {
                if let Some(track_number) = tag.track_number() {
                    result = Some(MusicTags {
                        album: album.to_string(),
                        artist: artist.to_string(),
                        dir_entry: music_file,
                        disk_number: tag.disc_number(),
                        title: title.to_string(),
                        track_number,
                    });
                }
            }
        }
    }

    result
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
