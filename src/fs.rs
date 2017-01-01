
// Copyright (c) 2016 Arav Singhal.

use std::path::Path;
use std::io;

use std::rc::Rc;
use std::collections::BTreeMap;

pub struct FileCache {
    pub files: BTreeMap<String, Rc<String>>,
}

impl FileCache {
    pub fn file_exists(&self, path: &Path) -> bool {
        self.files.contains_key(&path.to_str().unwrap_or("").to_string())
    }

    pub fn add_file(&mut self, name: String, source: String) {
        self.files.insert(name, Rc::new(source));
    }

    pub fn load_file(&self, path: &Path) -> io::Result<Rc<String>> {
        self.files
            .get(&path.to_str().unwrap_or("").to_string())
            .map(|s| s.clone())
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "File not found."))
    }

    pub fn new() -> FileCache {
        FileCache { files: BTreeMap::new() }
    }
}
