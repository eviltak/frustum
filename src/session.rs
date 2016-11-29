
// Copyright (c) 2016 Arav Singhal.

use syntax;

pub struct Session {
    // TODO: Complete definition
    // TODO: Make private
    pub parse_sess: syntax::parse::ParseSess,

    crate_root: Option<String>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            parse_sess: syntax::parse::ParseSess::new(),
            crate_root: None,
        }
    }

    pub fn new_from_crate_root(name: String, source: String) -> Session {
        let mut session = Session::new();
        session.crate_root = Some(name.clone());
        session.add_file(name, source);
        session
    }

    pub fn add_file(&self, name: String, source: String) {
        self.parse_sess.codemap().new_filemap(name, None, source);
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
}
