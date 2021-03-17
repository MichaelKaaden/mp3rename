use std::fs;

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
