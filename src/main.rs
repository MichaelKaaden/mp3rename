use std::cmp::Ordering;

use mp3bandtitle::config::Config;
use mp3bandtitle::{get_tags, DirContents, MusicTags};

fn main() {
    let config = Config::new();

    println!("==============");
    println!("Configuration:");
    println!("{}", config);
    println!("==============");

    let contents = DirContents::new(&config);
    for entry in &contents {
        println!("dir: {}", entry.dir_entry.path().to_str().unwrap());
        println!("music: {} entries", entry.music_files.len());
        println!("others: {} entries", entry.other_files.len());
        for o in &entry.other_files {
            println!("other file: {}", o.path().to_string_lossy());
        }
    }

    for entry in &contents {
        println!("=================");
        let mut tags: Vec<Option<MusicTags>> = entry
            .music_files
            .iter()
            .map(|music_file| get_tags(music_file))
            .collect();
        tags.sort_by(|a, b| sort_options(a, b));

        for tag in &tags {
            match tag {
                Some(t) => println!("{}", t),
                None => println!("No tags found",),
            }
        }
    }
}

fn sort_options(a: &Option<MusicTags>, b: &Option<MusicTags>) -> Ordering {
    let left = a.as_ref().unwrap_or_else(|| panic!("a is not defined"));
    let right = b.as_ref().unwrap_or_else(|| panic!("b is not defined"));
    left.track_number.cmp(&right.track_number)
}
