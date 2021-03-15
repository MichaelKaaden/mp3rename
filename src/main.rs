use mp3bandtitle::config::Config;
use mp3bandtitle::rename_music_files;

fn main() {
    let config = Config::new();

    println!("==============");
    println!("Configuration:");
    println!("{}", config);
    println!("==============");

    rename_music_files(&config);
}
