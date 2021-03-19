use crate::music_file::MusicFile;

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
