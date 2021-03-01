use mp3bandtitle::config::Config;
use mp3bandtitle::get_list_of_dirs;

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
}
