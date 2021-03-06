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
