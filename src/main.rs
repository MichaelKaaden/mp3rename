use std::process;

use mp3bandtitle::config::Config;
use mp3bandtitle::{get_dirs_with_music, get_list_of_dirs};

fn main() {
    let config = Config::new();

    println!("==============");
    println!("Configuration:");
    println!("{}", config);
    println!("==============");

    let entries = get_list_of_dirs(&config);
    println!("get_list_of_dirs() finished");
    for entry in &entries {
        println!("Got {}", entry.path().display());
    }

    let contents = get_dirs_with_music(entries);
    match contents {
        Ok(contents) => {
            for entry in contents {
                println!("dir: {}", entry.dir_entry.path().to_str().unwrap());
                println!("music: {} entries", entry.music_files.len());
                println!("others: {} entries", entry.other_files.len());
                for o in entry.other_files {
                    println!("other file: {}", o.path().to_string_lossy());
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading directories with music: {}", e);
            process::exit(1);
        }
    }
}
