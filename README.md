# mp3bandtitle

Traverses a directory tree and renames all music files and, optionally, the directories containing them according to the
tags in the music files.

Options:

- `-a`: Remove the artist from the filename if it is the same for all files in the current directory
- `-b`: Replace underscores with blanks in directory names
- `-d`: Rename directory according to the album tag
- `-f`: Prepare file and directory names for FAT filesystems
- `-l <num>`: Limit file and directory names to <num> characters
- `-n`: Dry run mode
- `-r`: Remove non-music files
