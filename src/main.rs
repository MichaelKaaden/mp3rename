use mp3bandtitle::config::Config;
use mp3bandtitle::rename_music_files;

fn main() {
    let config = Config::new();

    if config.dry_run {
        println!("*** Dry run mode ***");
    }

    if config.verbose {
        println!("==============");
        println!("Configuration:");
        println!("{}", config);
    }
    rename_music_files(&config);
}
