
// Copyright (c) 2016 Arav Singhal.

use syntax;

use std::path::{Path, PathBuf};
use std::io;
use std::rc::Rc;
use std::cell::RefCell;

use std::collections::BTreeMap;

pub struct SessionFileManager {
    pub files: BTreeMap<String, String>,
}

impl SessionFileManager {
    pub fn file_exists(&self, path: &Path) -> bool {
        self.files.contains_key(&path.to_str().unwrap_or("").to_string())
    }

    pub fn add_file(&mut self, name: String, source: String) {
        self.files.insert(name, source);
    }

    pub fn load_file(&self, path: &Path) -> io::Result<String> {
        self.files
            .get(&path.to_str().unwrap_or("").to_string())
            .map(|s| s.clone())
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "File not found."))
    }

    pub fn new() -> SessionFileManager {
        SessionFileManager { files: BTreeMap::new() }
    }
}

pub struct SessionFileLoader {
    pub file_manager: Rc<RefCell<SessionFileManager>>,
}

impl syntax::codemap::FileLoader for SessionFileLoader {
    fn file_exists(&self, path: &Path) -> bool {
        self.file_manager.borrow().file_exists(path)
    }

    fn abs_path(&self, path: &Path) -> Option<PathBuf> {
        // We do not need absolute paths
        None
    }

    fn read_file(&self, path: &Path) -> io::Result<String> {
        self.file_manager.borrow().load_file(path)
    }
}

impl SessionFileLoader {
    pub fn new() -> SessionFileLoader {
        SessionFileLoader { file_manager: Rc::new(RefCell::new(SessionFileManager::new())) }
    }
}
