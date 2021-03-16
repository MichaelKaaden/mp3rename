extern crate clap;

use std::fmt::Formatter;
use std::path::PathBuf;
use std::{env, fmt, fs, process};

use clap::{crate_authors, crate_version, App, Arg};

pub struct Config {
    pub dry_run: bool,
    pub name_length: u32,
    pub remove_artist: bool,
    pub remove_ordinary_files: bool,
    pub rename_directory: bool,
    pub shorten_names: bool,
    pub start_dir: PathBuf,
    pub use_fatfs_names: bool,
}

impl Config {
    pub fn new() -> Config {
        const ARTIST: &str = "artist";
        const DIRECTORY: &str = "directory";
        const DRY_RUN: &str = "dry-run";
        const FATFS: &str = "fatfs";
        const LENGTH: &str = "length";
        const LENGTH_VALUE: &str = "LENGTH";
        const REMOVE: &str = "remove";
        const START_DIR: &str = "START_DIR";

        let matches = App::new("mp3bandtitle")
            // use crate_version! to pull the version number
            .version(crate_version!())
            .author(crate_authors!())
            .about(
                "Traverses a directory tree and renames all music files and,
optionally, the directories containing them according to the
tags in the music files.",
            )
            .arg(
                Arg::with_name(ARTIST)
                    .short("a")
                    .long(ARTIST)
                    .help("Remove the artist from the filename if it is the same for all files in the current directory"),
            )
            .arg(
                Arg::with_name(DIRECTORY)
                    .short("d")
                    .long(DIRECTORY)
                    .help("Rename directory according to the album tag"),
            )
            .arg(
                Arg::with_name(DRY_RUN)
                    .short("n")
                    .long(DRY_RUN)
                    .help("Dry run mode"),
            )
            .arg(
                Arg::with_name(FATFS)
                    .short("f")
                    .long(FATFS)
                    .help("Prepare file and directory names for FAT filesystems"),
            )
            .arg(
                Arg::with_name(LENGTH)
                    .short("l")
                    .long(LENGTH)
                    .takes_value(true)
                    .value_name(LENGTH_VALUE)
                    .help("Limit file and directory names to <LENGTH> characters"),
            )
            .arg(
                Arg::with_name(REMOVE)
                    .short("r")
                    .long(REMOVE)
                    .help("Remove non-music files"),
            )
            .arg(
                // this is a positional argument
                Arg::with_name(START_DIR)
                    .help("the directory to start from (optional)")
                    .index(1)
                    .required(false),
            )
            .get_matches();

        let start_dir = String::from(
            matches
                .value_of(START_DIR)
                .unwrap_or(env::current_dir().unwrap().to_str().unwrap()),
        );

        let start_dir = match string_to_path(&start_dir) {
            Ok(path) => path,
            Err(_) => {
                eprintln!("Couldn't find the path \"{}\"", start_dir);
                process::exit(1);
            }
        };

        let name_length = match matches.value_of(LENGTH) {
            None => 0,
            Some(num) => match num.parse::<u32>() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Cannot parse length \"{}\"", num);
                    process::exit(1);
                }
            },
        };

        Config {
            dry_run: matches.is_present(DRY_RUN),
            name_length,
            remove_artist: matches.is_present(ARTIST),
            remove_ordinary_files: matches.is_present(REMOVE),
            rename_directory: matches.is_present(DIRECTORY),
            shorten_names: matches.is_present(LENGTH),
            start_dir,
            use_fatfs_names: matches.is_present(FATFS),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Dry run:                  {:?}", self.dry_run)?;
        writeln!(f, "Using path                {:?}", self.start_dir)?;
        writeln!(f, "Name length limit:        {:?}", self.name_length)?;
        writeln!(f, "Remove artist:            {:?}", self.remove_artist)?;
        writeln!(
            f,
            "Remove ordinary files:   {:?}",
            self.remove_ordinary_files
        )?;
        writeln!(f, "Rename directory:         {:?}", self.rename_directory)?;
        writeln!(f, "Shorten names:            {:?}", self.shorten_names)?;
        writeln!(f, "Use FAT-compatible names: {:?}", self.use_fatfs_names)
    }
}

fn string_to_path(file_name: &str) -> std::io::Result<PathBuf> {
    fs::canonicalize(PathBuf::from(file_name))
}
