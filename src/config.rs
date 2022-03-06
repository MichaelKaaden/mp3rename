extern crate clap;

use std::fmt::Formatter;
use std::path::PathBuf;
use std::{env, fmt, process};

use crate::util;
use clap::{crate_authors, crate_version, App, Arg};

#[derive(Default)]
pub struct Config {
    pub dry_run: bool,
    pub name_length: u32,
    pub omit_artist: bool,
    pub remove_artist: bool,
    pub remove_ordinary_files: bool,
    pub rename_directory: bool,
    pub shorten_names: bool,
    pub start_dir: PathBuf,
    pub verbose: bool,
}

impl Config {
    pub fn new() -> Config {
        const ARTIST: &str = "artist";
        const DIRECTORY: &str = "directory";
        const DRY_RUN: &str = "dry-run";
        const LENGTH: &str = "limit-length";
        const LENGTH_VALUE: &str = "LENGTH";
        const OMIT_ARTIST: &str = "omit-artist";
        const REMOVE: &str = "remove";
        const START_DIR: &str = "START_DIR";
        const VERBOSE: &str = "verbose";

        let matches = App::new("mp3rename")
            // use crate_version! to pull the version number
            .version(crate_version!())
            .author(crate_authors!())
            .about(
                "Traverses a directory tree and renames all music files and,
optionally, the directories containing them according to the
tags in the music files.
The resulting file name will have the form
[<Disc Number> - ]<Track Number> [<Artist> - ]<Track Title>.<extension>
(with extension in <mp3|flac|m4a|m4b|m4p|m4v>).",
            )
            .arg(
                Arg::with_name(ARTIST)
                    .short("a")
                    .long(ARTIST)
                    .help("Removes the artist from the filename if it is the same for all files in a directory"),
            )
            .arg(
                Arg::with_name(DIRECTORY)
                    .short("d")
                    .long(DIRECTORY)
                    .help("Renames directories according to the album tag"),
            )
            .arg(
                Arg::with_name(DRY_RUN)
                    .short("n")
                    .long(DRY_RUN)
                    .help("Uses dry-run mode"),
            )
            .arg(
                Arg::with_name(LENGTH)
                    .short("l")
                    .long(LENGTH)
                    .takes_value(true)
                    .value_name(LENGTH_VALUE)
                    .help("Limits the file and directory names to <LENGTH> characters"),
            )
            .arg(
                Arg::with_name(OMIT_ARTIST)
                    .short("o")
                    .long(OMIT_ARTIST)
                    .help("Omit artist"),
            )
            .arg(
                Arg::with_name(REMOVE)
                    .short("r")
                    .long(REMOVE)
                    .help("Removes non-music files"),
            )
            .arg(
                // this is a positional argument
                Arg::with_name(START_DIR)
                    .help("The directory to start from")
                    .index(1)
                    .required(true),
            )
            .arg(
                Arg::with_name(VERBOSE)
                    .short("v")
                    .long(VERBOSE)
                    .help("Be verbose"),
            )

            .get_matches();

        // the directory is mandatory
        let start_dir = matches.value_of(START_DIR).unwrap();
        let start_dir = match util::string_to_path(start_dir) {
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
            omit_artist: matches.is_present(OMIT_ARTIST),
            remove_artist: matches.is_present(ARTIST),
            remove_ordinary_files: matches.is_present(REMOVE),
            rename_directory: matches.is_present(DIRECTORY),
            shorten_names: matches.is_present(LENGTH),
            start_dir,
            verbose: matches.is_present(VERBOSE),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Dry run:                  {:?}", self.dry_run)?;
        writeln!(f, "Using path                {:?}", self.start_dir)?;
        writeln!(f, "Name length limit:        {:?}", self.name_length)?;
        writeln!(f, "Omit artist:              {:?}", self.omit_artist)?;
        writeln!(f, "Remove artist:            {:?}", self.remove_artist)?;
        writeln!(
            f,
            "Remove ordinary files:    {:?}",
            self.remove_ordinary_files
        )?;
        writeln!(f, "Rename directory:         {:?}", self.rename_directory)?;
        writeln!(f, "Shorten names:            {:?}", self.shorten_names)?;
        writeln!(f, "Verbose mode:             {:?}", self.verbose)
    }
}
