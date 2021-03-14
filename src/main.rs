use mp3bandtitle::config::Config;
use mp3bandtitle::dir_contents::DirContents;

fn main() {
    let config = Config::new();

    println!("==============");
    println!("Configuration:");
    println!("{}", config);
    println!("==============");

    let contents = DirContents::new(&config);
    for c in contents {
        println!("{}", c);
    }
}
