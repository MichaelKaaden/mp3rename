use std::fs;
use std::path::{Path, PathBuf};

use audiotags::{AudioTagEdit, AudioTagWrite, Id3v2Tag};
use mp3rename::config::Config;
use mp3rename::rename_music_files;

/// A directory below the system's temp directory that is removed when dropped
struct TestDir(PathBuf);

impl TestDir {
    fn new(name: &str) -> TestDir {
        let path = std::env::temp_dir().join(format!("mp3rename-{}-{}", std::process::id(), name));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        TestDir(path)
    }

    /// Creates an MP3 file containing nothing but complete tags
    fn add_tagged_file(&self, file_name: &str, track_number: u16, title: &str) {
        let path = self.0.join(file_name);
        fs::write(&path, b"").unwrap();
        let mut tag = Id3v2Tag::new();
        tag.set_album_title("Album");
        tag.set_artist("Artist");
        tag.set_title(title);
        tag.set_track_number(track_number);
        tag.write_to_path(path.to_str().unwrap()).unwrap();
    }

    fn file_names(&self) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(&self.0)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
            .collect();
        names.sort();
        names
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn config_for(dir: &TestDir) -> Config {
    Config {
        start_dir: dir.path().to_path_buf(),
        ..Config::default()
    }
}

#[test]
fn renames_file_according_to_its_tags() {
    let dir = TestDir::new("renames");
    dir.add_tagged_file("a.mp3", 1, "Title");

    rename_music_files(&config_for(&dir));

    assert_eq!(dir.file_names(), vec!["1 Artist - Title.mp3"]);
}

#[test]
fn renames_file_whose_name_only_differs_in_case() {
    // on case-insensitive file systems the target "exists" already, but it is the same file
    let dir = TestDir::new("case-only");
    dir.add_tagged_file("1 artist - title.mp3", 1, "Title");

    rename_music_files(&config_for(&dir));

    assert_eq!(dir.file_names(), vec!["1 Artist - Title.mp3"]);
}

#[test]
fn does_not_rename_files_that_would_get_the_same_name() {
    let dir = TestDir::new("same-name");
    dir.add_tagged_file("a.mp3", 1, "Title");
    dir.add_tagged_file("b.mp3", 1, "Title");

    rename_music_files(&config_for(&dir));

    assert_eq!(dir.file_names(), vec!["a.mp3", "b.mp3"]);
}

#[test]
fn does_not_overwrite_an_existing_file() {
    let dir = TestDir::new("existing");
    dir.add_tagged_file("a.mp3", 1, "Title");
    // has no tags, so it isn't renamed itself, but it is in the way
    fs::write(dir.path().join("1 Artist - Title.mp3"), b"keep me").unwrap();

    rename_music_files(&config_for(&dir));

    assert_eq!(dir.file_names(), vec!["1 Artist - Title.mp3", "a.mp3"]);
    assert_eq!(
        fs::read(dir.path().join("1 Artist - Title.mp3")).unwrap(),
        b"keep me"
    );
}

#[test]
fn does_not_rename_to_an_empty_name_when_the_length_limit_is_too_short() {
    let dir = TestDir::new("too-short");
    dir.add_tagged_file("a.mp3", 1, "Title");

    rename_music_files(&Config {
        name_length: 3,
        shorten_names: true,
        ..config_for(&dir)
    });

    assert_eq!(dir.file_names(), vec!["a.mp3"]);
}
