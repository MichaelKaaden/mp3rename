use crate::config::Config;

mod config;

fn main() {
    let config = Config::new();

    println!("The config is: \n{}", config);
}
