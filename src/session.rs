
// Copyright (c) 2016 Arav Singhal.

use syntax;
use syntax_errors;

use ::internal::session::{SessionFileLoader, SessionFileManager};

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

pub struct Session {
    // TODO: Complete definition
    // TODO: Make private
    pub parse_sess: syntax::parse::ParseSess,

    crate_root: Option<String>,
    file_manager: Rc<RefCell<SessionFileManager>>,
}

impl Session {
    pub fn add_file(&self, name: String, source: String) {
        self.parse_sess.codemap().new_filemap(name.clone(), None, source.clone());
        self.file_manager.borrow_mut().add_file(name, source);
    }

    pub fn parse_as_crate(&self) {
        if self.crate_root.is_none() {
            // TODO: Error handling
            panic!("No crate root defined!");
        }
        let mut parser =
            syntax::parse::filemap_to_parser(&self.parse_sess,
                                             self.parse_sess
                                                 .codemap()
                                                 .get_filemap(&self.crate_root.as_ref().unwrap())
                                                 .unwrap());

        match parser.parse_crate_mod() {
            Ok(_) => println!("Success"),
            Err(mut e) => {
                e.emit();
                e.cancel();
            }
        }
    }

    pub fn new() -> Session {
        let file_loader = Box::new(SessionFileLoader::new());

        let file_manager = file_loader.as_ref().file_manager.clone();

        let codemap = Rc::new(syntax::codemap::CodeMap::with_file_loader(file_loader));

        let handler = syntax_errors::Handler::with_tty_emitter(syntax_errors::ColorConfig::Auto,
                                                               true,
                                                               false,
                                                               Some(codemap.clone()));

        Session {
            parse_sess: syntax::parse::ParseSess::with_span_handler(handler, codemap),
            crate_root: None,
            file_manager: file_manager,
        }
    }

    pub fn new_from_crate_root(name: String, source: String) -> Session {
        let mut session = Session::new();
        session.crate_root = Some(name.clone());
        session.add_file(name, source);
        session
    }
}
