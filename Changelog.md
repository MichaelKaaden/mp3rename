
v1.0.11 / 2022-01-03
==================

  * [chore] Bump version for release v1.0.11
  * [chore] Update used crates
  * [chore] Update the API key
  * [chore] Bump version for the next development cycle
  * Merge tag 'v1.0.10' into develop

v1.0.10 2021-12-15
==================

  * [chore] Bump version for release v1.0.10
  * [chore] Update crates
  * [chore] Update to Rust 2021
  * [chore] Update the API key
  * [chore] Bump version for the next development cycle

v1.0.9 / 2021-07-03
==================

  * [chore] Bump version for release v1.0.9
  * [chore] Update crates
  * [chore] Bump version for the next development cycle

v1.0.8 / 2021-06-15
==================

  * Add the option to completely omit the artist in the filename
  * [chore] Update libc and syn
  * [chore] Update dependencies
  * [chore] Add a minimal Dockerfile for building and running the binary
  * [chore] Update the "syn" crate
  * [chore] Bump version for the next development cycle

v1.0.7 / 2021-03-27
==================

  * Produce stable output by sorting the keys from the disk hash map
  * Refactor comparing disk_numbers into a function of its own
  * [chore] Add a Changelog.md file
  * Fix the last Clippy warning regarding Entry.on_insert()
  * Calculate the disk number's zero padding correctly
  * Handle music files per disk instead of per directory
  * Fix a Clippy warning
  * Unit-test MusicFile's canonical_name()
  * Use zero-padding for the disc number and separate it with blanks from the track number
  * [chore] Bump version for the next development cycle

v1.0.6 / 2021-03-24
===================

  * [chore] Bump version for release 1.0.6
  * [chore] Update libc and walkdir
  * [chore] Fix typos
  * [chore] Update wording
  * [chore] Add some tags to the README.md file
  * [chore] Bump version for the next development cycle

v1.0.5 / 2021-03-22
===================

  * [chore] Bump version for release 1.0.5
  * [chore] Build also for commits on master
  * [chore] Add a new API Key
  * [chore] Add Travis CI configuration
  * [chore] Add a link to the "releases" page
  * [chore] Bump version for the next development cycle

v1.0.4 / 2021-03-21
===================

  * [chore] Bump version for release 1.0.4
  * [chore] Add a screenshot to the README.md
  * Implement tests for MusicMetadata::sort_func()
  * Move the functions from dir_contents.rs to music_file.rs
  * Add a comment that it's intentional to only accept complete metadata
  * Add more tests
  * Simplify shorten_names()
  * Move string_to_path() to util and unit-test get_extension()
  * [chore] Update libc
  * [chore] Update the README.md
  * [chore] Add an MIT License
  * [chore] Bump version for the development cycle

v1.0.3 / 2021-03-19
===================

  * [chore] Bump version for release 1.0.3
  * Make the directory argument mandatory mitigating the last Clippy warning
  * Get rid of an unnecessary unwrap()
  * Use a slice reference instead of a vector reference
  * Implement (and use) Config::default()
  * Fix auto-fixable issues
  * [chore] Bump version for the development cycle

v1.0.2 / 2021-03-19
===================

  * [chore] Bump version for release 1.0.2
  * [chore] Re-configure IntelliJ IDEA for the new project name
  * [chore] Rename project to "mp3rename"

v1.0.1 / 2021-03-18
===================

  * [chore] Bump version number for release 1.0.1
  * [chore] Update the version number
  * Iterate over directories instead of building a vector
  * Better error message for incomplete tag information

v1.0.0 / 2021-03-18
===================

  * [chore] Release version 1.0.0 (yay!)
  * Report errors on instantiating AudioTag instances instead of panicking
  * [chore] Update packages
  * Improve the usage message
  * Refactor more functions into utils
  * Let get_extension() only get music extensions and leave others alone
  * Move the is_music... into an util module
  * [chore] Update the README.md on how to install the binary
  * Add some tests for the sanitization
  * Make the FATFS replacements now mandatory
  * Replace "/" with " & " instead of "&"
  * Add a sanitizer test for leading and trailing blanks
  * Sanitize the canonical name *without* extension
  * Refactor shorten_names() into three functions
  * Use the sanitized name to build the new file name
  * Rename the command line switch "length" to "limit-length"
  * Improve help message
  * Implement shorten names
  * Fix the Config formatter
  * Sanitize the name we rename to
  * Only rename if the name really changed
  * Handle directories down to top to correctly rename multiple levels
  * Implement verbose mode
  * Rename the album *after* removing the ordinary files
  * Implement removing ordinary files
  * Remove the "remove blanks" option
  * Unify rename_music_file() and rename_directory()
  * Add an explanation why *two* functions are needed to rename files and directories
  * Rename music files according to their canonical name
  * Refactor the code to rename the directory into a function of its own
  * Rename the containing directory according to the album title
  * Implement same_album_title() in DirContents
  * Prettify output
  * Get the dash right for the canonical filename
  * Zero-pad the track number in the canonical filename
  * Rename functions and add a more general rename handler
  * Implement a getter for the canonical music file name
  * Report an error for music files without tags
  * Move logic inside the lib.rs
  * Filter music files to contain tags, else produce an error and continue
  * Check if a DirContents has the same artist for all music files
  * Remove empty line in the ftm::Display implementation
  * Remove left over import
  * Implement fmt::Display for all structs
  * Sort the music_files vector
  * [core] Update dependencies
  * Own files for the structs
  * Move the tag extraction into the lib.rs
  * Append the tags to the DirContents struct
  * Extend DirContents to contain the tag list
  * Exit the process if at least one tag couldn't be found
  * Sort tags by track number and disk number
  * Sort tags by track number
  * Iterate over all music files in a directory
  * Retrieve the first audio tag from each file in each directory
  * Do no longer break out on unreadable directories
  * Define DirContents::new()
  * Add the "audiotags" crate and allow more extensions
  * Return directories containing music files
  * Add a hint on filtering only successfully retrieved directories
  * Use a reference to enable borrowing
  * Get a vector of DirEntry
  * Get a vector of Result<DirEntry, Error>
  * Improve error message
  * Filter for directories
  * Use a reference to walk directories
  * Start walking directories
  * [chore] Add the "walkdir" dependency
  * Add beautiful error handling
  * Make the Config.start_dir a PathBuf
  * Use the crate_authors macro to read the author from Cargo.toml
  * Implement command line parsing
  * [chore] Use package "clap"
  * [chore] Add a README.md
  * Initial commit

