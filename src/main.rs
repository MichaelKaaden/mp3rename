use std::process;

use mp3bandtitle::config::Config;
use mp3bandtitle::DirContents;

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
