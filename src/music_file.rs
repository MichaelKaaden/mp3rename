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
        number_of_music_files_in_this_directory: usize,
    ) -> Option<String> {
        if let Some(metadata) = &self.music_metadata {
            let disk_number = match metadata.disk_number {
                None => String::new(),
                Some(num) => format!("{}", num),
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
