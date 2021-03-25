use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::fs;

use crate::config::Config;
use crate::music_metadata::MusicMetadata;

pub struct MusicFile {
    pub dir_entry: fs::DirEntry,
    pub music_metadata: Option<MusicMetadata>,
}

impl MusicFile {
    pub fn new(dir_entry: fs::DirEntry) -> MusicFile {
        let music_metadata = MusicMetadata::new(&dir_entry);

        MusicFile {
            dir_entry,
            music_metadata,
        }
    }

    pub fn canonical_name(
        self: &MusicFile,
        config: &Config,
        is_same_artist_for_whole_album: bool,
        number_of_digits_for_disc_number: usize,
        number_of_music_files_in_this_directory: usize,
    ) -> Option<String> {
        if let Some(metadata) = &self.music_metadata {
            let disk_number = match metadata.disk_number {
                None => String::new(),
                Some(num) => format!(
                    "{:0width$} - ",
                    num,
                    width = number_of_digits_for_disc_number
                ),
            };

            // number of digits to zero-pad the track number
            let num_digits = number_of_music_files_in_this_directory.to_string().len();
            let track_number = format!("{:0width$}", metadata.track_number, width = num_digits);

            let artist = if config.remove_artist && is_same_artist_for_whole_album {
                String::new()
            } else {
                format!(" {} -", &metadata.artist)
            };

            let extension = match self.dir_entry.path().extension() {
                None => String::new(),
                Some(ext) => format!(".{}", ext.to_string_lossy().to_lowercase()),
            };

            let result = format!(
                "{}{}{} {}{}",
                disk_number, track_number, artist, metadata.title, extension
            );

            return Some(result);
        }

        None
    }

    pub fn sort_func(left: &MusicFile, right: &MusicFile) -> Ordering {
        MusicMetadata::sort_func(&left.music_metadata, &right.music_metadata)
    }
}

impl fmt::Display for MusicFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "File Name:    {}",
            self.dir_entry.path().to_string_lossy()
        )?;
        match &self.music_metadata {
            None => writeln!(f, "No tags found.")?,
            Some(tags) => writeln!(f, "{}", tags)?,
        };

        fmt::Result::Ok(())
    }
}

/// Has the whole directory the same artist for every music file?
pub fn same_artists(music_files: &[MusicFile]) -> bool {
    let artists: Vec<&String> = music_files
        .iter()
        .filter_map(|m| m.music_metadata.as_ref())
        .map(|m| &m.artist)
        .collect();

    if !artists.is_empty() {
        let first_artist = artists[0];
        for artist in artists {
            if artist != first_artist {
                return false;
            }
        }
        return true;
    }

    // an error for missing tags has already been reported in MusicMetadata::new()
    false
}

// Which album name does the whole directory have for all music files?
pub fn same_album_title(music_files: &[MusicFile]) -> Option<&String> {
    let albums: Vec<&String> = music_files
        .iter()
        .filter_map(|m| m.music_metadata.as_ref())
        .map(|m| &m.album)
        .collect();

    if !albums.is_empty() {
        let first_album = albums[0];
        for album in albums {
            if album != first_album {
                return None;
            }
        }
        return Some(first_album);
    }

    None
}

pub fn largest_disc_number(music_files: &[MusicFile]) -> Option<u16> {
    let mut largest: u16 = 0;

    for music_file in music_files {
        if let Some(meta_data) = &music_file.music_metadata {
            if let Some(disc_number) = meta_data.disk_number {
                if disc_number > largest {
                    largest = disc_number;
                }
            }
        }
    }

    if largest > 0 {
        return Some(largest);
    }

    None
}
