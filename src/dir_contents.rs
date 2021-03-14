use std::fmt;
use std::fmt::Formatter;
use std::fs;

use walkdir::WalkDir;

use crate::config::Config;
use crate::music_file::MusicFile;
use crate::ordinary_file::OrdinaryFile;

pub struct DirContents {
    pub dir_entry: walkdir::DirEntry,
    pub music_files: Vec<MusicFile>,       // contained music files
    pub ordinary_files: Vec<OrdinaryFile>, // contained other files (potentially being deleted)
}

impl DirContents {
    pub fn new(config: &Config) -> Vec<DirContents> {
        let all_files_and_directories = get_list_of_dirs(&config);
        let music_directories = get_dirs_with_music(all_files_and_directories);
        music_directories
    }

    pub fn same_artists(&self) -> bool {
        let artists: Vec<&String> = self
            .music_files
            .iter()
            .filter_map(|m| m.music_metadata.as_ref())
            .map(|m| &m.artist)
            .collect();

        if artists.len() > 0 {
            let first_artist = artists[0];
            for artist in artists {
                if artist != first_artist {
                    return false;
                }
            }
            return true;
        }

        false
    }
}

impl fmt::Display for DirContents {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Directory Name: {}",
            self.dir_entry.path().to_string_lossy()
        )?;
        writeln!(f, "music files:    {} entries", self.music_files.len())?;
        writeln!(f, "ordinary files: {} entries", self.ordinary_files.len())?;
        for o in &self.ordinary_files {
            writeln!(
                f,
                "ordinary file:  {}",
                o.dir_entry.path().to_string_lossy()
            )?;
        }
        for m in &self.music_files {
            writeln!(f, "{}", m)?;
        }

        fmt::Result::Ok(())
    }
}

/// Returns the list of directories.
fn get_list_of_dirs(config: &Config) -> Vec<walkdir::DirEntry> {
    WalkDir::new(&config.start_dir)
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

/// Returns directories containing music files
fn get_dirs_with_music(files_and_directories: Vec<walkdir::DirEntry>) -> Vec<DirContents> {
    let mut dir_contents = vec![];

    for dir in files_and_directories {
        if dir.file_type().is_dir() {
            let readdir = fs::read_dir(dir.path());
            if readdir.is_ok() {
                let (music, others): (Vec<fs::DirEntry>, Vec<fs::DirEntry>) = readdir
                    .unwrap()
                    .filter(|dir_entry| dir_entry.as_ref().unwrap().path().is_file())
                    .map(|dir_entry| dir_entry.unwrap())
                    .partition(|dir_entry| is_music_file(dir_entry));

                // only return directories containing music files
                if music.len() > 0 {
                    let mut music_files: Vec<MusicFile> = music
                        .into_iter()
                        .map(|dir_entry| MusicFile::new(dir_entry))
                        .collect();
                    music_files.sort_by(|left, right| MusicFile::sort_func(left, right));

                    let ordinary_files: Vec<OrdinaryFile> =
                        others.into_iter().map(|o| OrdinaryFile::new(o)).collect();

                    dir_contents.push(DirContents {
                        dir_entry: dir,
                        music_files,
                        ordinary_files,
                    });
                }
            }
        }
    }

    dir_contents
}

fn is_music_file(entry: &fs::DirEntry) -> bool {
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
