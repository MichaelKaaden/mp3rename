use mp3bandtitle::config::Config;
use mp3bandtitle::rename;

fn main() {
    let config = Config::new();

    println!("==============");
    println!("Configuration:");
    println!("{}", config);
    println!("==============");

    rename(&config);
}
