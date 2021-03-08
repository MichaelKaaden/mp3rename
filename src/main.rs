use std::cmp::Ordering;
use std::process;

use mp3bandtitle::config::Config;
use mp3bandtitle::{get_tags, DirContents, MusicTags};

fn main() {
    let config = Config::new();

    println!("==============");
    println!("Configuration:");
    println!("{}", config);
    println!("==============");

    let mut contents = DirContents::new(&config);
    for entry in &contents {
        println!("dir: {}", entry.dir_entry.path().to_str().unwrap());
        println!("music: {} entries", entry.music_files.len());
        println!("others: {} entries", entry.other_files.len());
        for o in &entry.other_files {
            println!("other file: {}", o.path().to_string_lossy());
        }
    }

    for entry in &mut contents {
        println!("=================");
        let mut tags: Vec<Option<MusicTags>> = entry
            .music_files
            .iter()
            .map(|music_file| get_tags(music_file))
            .collect();
        tags.sort_by(|a, b| sort_music_tag_func(a, b));
        entry.music_tags = tags;

        for tag in &entry.music_tags {
            match tag {
                Some(t) => println!("{}", t),
                None => {
                    eprintln!(
                        "No tags found in {}",
                        entry.dir_entry.path().to_string_lossy()
                    );
                    process::exit(1);
                }
            }
        }
    }
}

fn sort_music_tag_func(a: &Option<MusicTags>, b: &Option<MusicTags>) -> Ordering {
    let left = a.as_ref().unwrap_or_else(|| panic!("a is not defined"));
    let right = b.as_ref().unwrap_or_else(|| panic!("b is not defined"));

    if left.disk_number.is_some() && right.disk_number.is_none() {
        return Ordering::Greater;
    } else if left.disk_number.is_none() && right.disk_number.is_some() {
        return Ordering::Less;
    }

    if let Some(left_disk_number) = left.disk_number {
        if let Some(right_disk_number) = right.disk_number {
            let disk_number_comparison = left_disk_number.cmp(&right_disk_number);
            if disk_number_comparison != Ordering::Equal {
                return disk_number_comparison;
            }
        }
    }

    left.track_number.cmp(&right.track_number)
}
