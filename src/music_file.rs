use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::fs;

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
