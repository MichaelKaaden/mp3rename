use mp3rename::config::Config;
use mp3rename::rename_music_files;

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
