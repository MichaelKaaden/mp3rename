# mp3rename

This command line utility has only one purpose: It traverses a directory tree and renames all music files it discovers
and, optionally, the directories containing them according to the tags in the music files.

## Requirements

The program searches for music files with an extension in `<mp3|flac|m4a|m4b|m4p|m4v>`.

The tags for the track number, track tile, artist name, and album name are mandatory. Without them, the program will
omit the files.

## Result

The resulting file name will have the form
`<Disc Number><Track Number> <Artist> - <Track Title>.<extension>`.

Optionally, the directory containing the music files will be renamed to the album title (if the same for all music files
within this directory).

## Installation

To install this as a binary, please use `cargo install --path .` in this project's root directory. 
