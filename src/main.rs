use mp3bandtitle::config::Config;
use mp3bandtitle::traverse_dirs;

fn main() {
    let config = Config::new();

    println!("The config is: \n{}", config);

    println!("Path is {:?}", config.start_dir);

    traverse_dirs(&config);
}
