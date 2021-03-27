use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::fs;

use crate::config::Config;
use crate::music_metadata::MusicMetadata;
use std::collections::HashMap;

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
        number_of_music_files_in_this_disk: usize,
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
            let num_digits = number_of_music_files_in_this_disk.to_string().len();
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
pub fn same_album_title(music_files: &[MusicFile]) -> Option<String> {
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
        return Some(String::from(first_album));
    }

    None
}

pub fn largest_disc_number(music_files: &HashMap<Option<u16>, Vec<MusicFile>>) -> Option<u16> {
    let mut largest: u16 = 0;

    for disk_number in music_files.keys() {
        if let Some(disk_number) = disk_number {
            if *disk_number > largest {
                largest = *disk_number;
            }
        }
    }

    if largest > 0 {
        return Some(largest);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_ALBUM: &str = "The Foos are Back";
    const DEFAULT_ARTIST: &str = "The Foos";
    const DEFAULT_TITLE: &str = "Foo de Foo";

    fn get_dir_entry() -> fs::DirEntry {
        let mut readdir = fs::read_dir("testfiles").unwrap();
        readdir.next().unwrap().unwrap()
    }

    fn get_music_metadata() -> MusicMetadata {
        MusicMetadata {
            album: DEFAULT_ALBUM.to_string(),
            artist: DEFAULT_ARTIST.to_string(),
            disk_number: None,
            title: DEFAULT_TITLE.to_string(),
            track_number: 1,
        }
    }

    #[test]
    fn test_canonical_name_for_default_config() {
        let config = Config::default();
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(get_music_metadata()),
        };
        let same_artist = false;

        // one digit
        let number_of_digits_for_disc_number = 0;
        let number_of_music_files_in_this_directory = 1;
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("1 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );

        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 9,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("9 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );

        // two digits
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(get_music_metadata()),
        };
        let number_of_music_files_in_this_directory = 10;
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("01 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 9,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("09 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 10,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("10 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );

        // three digits
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(get_music_metadata()),
        };
        let number_of_music_files_in_this_directory = 100;
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("001 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );

        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 10,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("010 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 99,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("099 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 100,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("100 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );

        // four digits
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(get_music_metadata()),
        };
        let number_of_music_files_in_this_directory = 1000;
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("0001 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );

        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 10,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("0010 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 99,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("0099 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 100,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("0100 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 999,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory,
            ),
            Some(format!("0999 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 1000,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("1000 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );
    }

    #[test]
    fn test_canonical_name_for_remove_artist() {
        let config = Config {
            remove_artist: true,
            ..Config::default()
        };
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(get_music_metadata()),
        };
        let same_artist = true;

        // one digit
        let number_of_digits_for_disc_number = 0;
        let number_of_music_files_in_this_directory = 1;
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("1 {}.mp3", DEFAULT_TITLE))
        );

        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 9,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("9 {}.mp3", DEFAULT_TITLE))
        );

        // two digits
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(get_music_metadata()),
        };
        let number_of_music_files_in_this_directory = 10;
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("01 {}.mp3", DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 9,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("09 {}.mp3", DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 10,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("10 {}.mp3", DEFAULT_TITLE))
        );

        // three digits
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(get_music_metadata()),
        };
        let number_of_music_files_in_this_directory = 100;
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("001 {}.mp3", DEFAULT_TITLE))
        );

        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 10,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("010 {}.mp3", DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 99,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("099 {}.mp3", DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 100,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("100 {}.mp3", DEFAULT_TITLE))
        );

        // four digits
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(get_music_metadata()),
        };
        let number_of_music_files_in_this_directory = 1000;
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("0001 {}.mp3", DEFAULT_TITLE))
        );

        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 10,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("0010 {}.mp3", DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 99,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("0099 {}.mp3", DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 100,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("0100 {}.mp3", DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 999,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory,
            ),
            Some(format!("0999 {}.mp3", DEFAULT_TITLE))
        );
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                track_number: 1000,
                ..get_music_metadata()
            }),
        };
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("1000 {}.mp3", DEFAULT_TITLE))
        );
    }

    #[test]
    fn test_canonical_name_with_disc_numbers() {
        let config = Config::default();
        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                disk_number: Some(1),
                ..get_music_metadata()
            }),
        };
        let same_artist = false;

        let number_of_digits_for_disc_number = 1;
        let number_of_music_files_in_this_directory = 1;
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("1 - 1 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );

        let music_file = MusicFile {
            dir_entry: get_dir_entry(),
            music_metadata: Some(MusicMetadata {
                disk_number: Some(1),
                ..get_music_metadata()
            }),
        };
        let number_of_digits_for_disc_number = 2;
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!("01 - 1 {} - {}.mp3", DEFAULT_ARTIST, DEFAULT_TITLE))
        );

        let number_of_digits_for_disc_number = 3;
        let number_of_music_files_in_this_directory = 100;
        assert_eq!(
            music_file.canonical_name(
                &config,
                same_artist,
                number_of_digits_for_disc_number,
                number_of_music_files_in_this_directory
            ),
            Some(format!(
                "001 - 001 {} - {}.mp3",
                DEFAULT_ARTIST, DEFAULT_TITLE
            ))
        );
    }
}
