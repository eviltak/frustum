
// Copyright (c) 2016 Arav Singhal.

use syntax;
use syntax_errors;

use fs::FileCache;

use std::path::{Path, PathBuf};
use std::io;
use std::rc::Rc;
use std::cell::RefCell;

struct SessionFileLoader {
    pub file_cache: Rc<RefCell<FileCache>>,
}

impl syntax::codemap::FileLoader for SessionFileLoader {
    fn file_exists(&self, path: &Path) -> bool {
        self.file_cache.borrow().file_exists(path)
    }

    fn abs_path(&self, path: &Path) -> Option<PathBuf> {
        // We do not need absolute paths
        None
    }

    fn read_file(&self, path: &Path) -> io::Result<String> {
        self.file_cache.borrow().load_file(path).map(|s| (*s).clone())
    }
}

impl SessionFileLoader {
    pub fn new() -> SessionFileLoader {
        SessionFileLoader { file_cache: Rc::new(RefCell::new(FileCache::new())) }
    }
}

pub struct Session {
    // TODO: Complete definition
    pub crate_root: Option<String>,

    parse_sess: syntax::parse::ParseSess,
    file_cache: Rc<RefCell<FileCache>>,
}

impl Session {
    pub fn add_file(&self, name: String, source: String) {
        self.parse_sess.codemap().new_filemap(name.clone(), None, source.clone());
        self.file_cache.borrow_mut().add_file(name, source);
    }

    pub fn new() -> Session {
        let file_loader = Box::new(SessionFileLoader::new());

        let file_cache = file_loader.as_ref().file_cache.clone();

        let codemap = Rc::new(syntax::codemap::CodeMap::with_file_loader(file_loader));

        let handler = syntax_errors::Handler::with_tty_emitter(syntax_errors::ColorConfig::Auto,
                                                               false,
                                                               false,
                                                               Some(codemap.clone()));

        Session {
            parse_sess: syntax::parse::ParseSess::with_span_handler(handler, codemap),
            crate_root: None,
            file_cache: file_cache,
        }
    }

    pub fn new_from_crate_root(name: String, source: String) -> Session {
        let mut session = Session::new();
        session.crate_root = Some(name.clone());
        session.add_file(name, source);
        session
    }
}
/*
pub fn parse_sess(session: &Session) -> &syntax::parse::ParseSess {
    &session.parse_sess
}
*/
