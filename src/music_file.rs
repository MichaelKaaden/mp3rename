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
}
