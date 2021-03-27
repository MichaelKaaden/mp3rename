use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

pub struct MusicMetadata {
    pub album: String,
    pub artist: String,
    pub disk_number: Option<u16>,
    pub title: String,
    pub track_number: u16,
}

impl MusicMetadata {
    pub fn new(music_file: &std::fs::DirEntry) -> Option<MusicMetadata> {
        let tag = match audiotags::Tag::new().read_from_path(music_file.path()) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}: {}", music_file.path().to_string_lossy(), e);
                return None;
            }
        };

        // we only accept *complete* metadata
        if let Some(album) = tag.album_title() {
            if let Some(artist) = tag.artist() {
                if let Some(title) = tag.title() {
                    if let Some(track_number) = tag.track_number() {
                        return Some(MusicMetadata {
                            album: album.to_string(),
                            artist: artist.to_string(),
                            disk_number: tag.disc_number(),
                            title: title.to_string(),
                            track_number,
                        });
                    }
                }
            }
        }

        eprintln!(
            "Error: Incomplete tags found in {} -- need album, artist, title, and track number.",
            music_file.path().to_string_lossy()
        );
        None
    }

    pub fn sort_func(a: &Option<MusicMetadata>, b: &Option<MusicMetadata>) -> Ordering {
        let left = a.as_ref().unwrap_or_else(|| panic!("No tags defined"));
        let right = b.as_ref().unwrap_or_else(|| panic!("No tags defined"));

        let disk_number_comparison =
            MusicMetadata::sort_by_disk_number_func(&left.disk_number, &right.disk_number);
        if disk_number_comparison != Ordering::Equal {
            return disk_number_comparison;
        }

        left.track_number.cmp(&right.track_number)
    }

    pub fn sort_by_disk_number_func(left: &Option<u16>, right: &Option<u16>) -> Ordering {
        let left = *left;
        let right = *right;

        if left.is_none() && right.is_none() {
            return Ordering::Equal;
        } else if left.is_some() && right.is_none() {
            return Ordering::Greater;
        } else if left.is_none() && right.is_some() {
            return Ordering::Less;
        }

        if let Some(left_disk_number) = left {
            if let Some(right_disk_number) = right {
                return left_disk_number.cmp(&right_disk_number);
            }
        }

        Ordering::Equal
    }
}

impl fmt::Display for MusicMetadata {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "No tags defined")]
    fn test_sort_func_panic() {
        MusicMetadata::sort_func(&None, &None);
        MusicMetadata::sort_func(
            &None,
            &Some(MusicMetadata {
                album: "".to_string(),
                artist: "".to_string(),
                disk_number: None,
                title: "".to_string(),
                track_number: 0,
            }),
        );
        MusicMetadata::sort_func(
            &Some(MusicMetadata {
                album: "".to_string(),
                artist: "".to_string(),
                disk_number: None,
                title: "".to_string(),
                track_number: 0,
            }),
            &None,
        );
    }

    #[test]
    fn test_sort_func_empty() {
        assert_eq!(
            MusicMetadata::sort_func(
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: None,
                    title: "".to_string(),
                    track_number: 0,
                }),
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: None,
                    title: "".to_string(),
                    track_number: 0,
                }),
            ),
            Ordering::Equal
        );
    }

    #[test]
    fn test_sort_func_disc_number() {
        assert_eq!(
            MusicMetadata::sort_func(
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: Some(1),
                    title: "".to_string(),
                    track_number: 0
                }),
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: None,
                    title: "".to_string(),
                    track_number: 0
                }),
            ),
            Ordering::Greater,
        );
        assert_eq!(
            MusicMetadata::sort_func(
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: None,
                    title: "".to_string(),
                    track_number: 0
                }),
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: Some(1),
                    title: "".to_string(),
                    track_number: 0
                }),
            ),
            Ordering::Less,
        );
        assert_eq!(
            MusicMetadata::sort_func(
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: Some(1),
                    title: "".to_string(),
                    track_number: 0
                }),
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: Some(2),
                    title: "".to_string(),
                    track_number: 0
                }),
            ),
            Ordering::Less,
        );
        assert_eq!(
            MusicMetadata::sort_func(
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: Some(2),
                    title: "".to_string(),
                    track_number: 0
                }),
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: Some(1),
                    title: "".to_string(),
                    track_number: 0
                }),
            ),
            Ordering::Greater,
        );
    }

    #[test]
    fn test_sort_func_track_number() {
        assert_eq!(
            MusicMetadata::sort_func(
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: None,
                    title: "".to_string(),
                    track_number: 1
                }),
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: None,
                    title: "".to_string(),
                    track_number: 2
                })
            ),
            Ordering::Less
        );
        assert_eq!(
            MusicMetadata::sort_func(
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: None,
                    title: "".to_string(),
                    track_number: 2
                }),
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: None,
                    title: "".to_string(),
                    track_number: 1
                })
            ),
            Ordering::Greater
        );
        assert_eq!(
            MusicMetadata::sort_func(
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: None,
                    title: "".to_string(),
                    track_number: 1
                }),
                &Some(MusicMetadata {
                    album: "".to_string(),
                    artist: "".to_string(),
                    disk_number: None,
                    title: "".to_string(),
                    track_number: 1
                })
            ),
            Ordering::Equal
        );
    }
}
