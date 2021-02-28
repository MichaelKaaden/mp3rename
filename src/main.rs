use mp3bandtitle::config::Config;
use mp3bandtitle::get_list_of_dirs;

fn main() {
    let config = Config::new();

    println!("The config is: \n{}", config);

    println!("Path is {:?}", config.start_dir);

    let entries = get_list_of_dirs(&config);
    println!("get_list_of_dirs() finished");
    for entry in entries {
        match entry {
            Ok(e) => println!("Got {}", e.path().display()),
            Err(err) => eprintln!("Error iterating: {}", err),
        }
    }
}
