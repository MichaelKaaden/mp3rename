use std::fs;

pub struct OrdinaryFile {
    pub dir_entry: fs::DirEntry,
}

impl OrdinaryFile {
    pub fn new(dir_entry: fs::DirEntry) -> OrdinaryFile {
        OrdinaryFile { dir_entry }
    }
}
