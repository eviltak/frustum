// Copyright (c) 2016 Arav Singhal.

use syntax;

use std::path::{Path, PathBuf};
use std::io;
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

pub struct SessionFileLoader {
    pub files: Rc<RefCell<HashSet<String>>>,
}

impl syntax::codemap::FileLoader for SessionFileLoader {
    fn file_exists(&self, path: &Path) -> bool {
        self.files.borrow().contains(&path.to_str().map(|p| p.to_string()).unwrap())
    }

    fn abs_path(&self, path: &Path) -> Option<PathBuf> {
        // We do not need absolute paths
        None
    }

    fn read_file(&self, path: &Path) -> io::Result<String> {
        // We SHOULD not need reading from a file
        Ok(String::new())
    }
}

impl SessionFileLoader {
    pub fn new() -> SessionFileLoader {
        SessionFileLoader { files: Rc::new(RefCell::new(HashSet::new())) }
    }
}
