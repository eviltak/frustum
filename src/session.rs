
// Copyright (c) 2016 Arav Singhal.

use syntax;

pub struct Session {
    // TODO: Complete definition
    pub parse_sess: syntax::parse::ParseSess,
}

impl Session {
    pub fn new() -> Session {
        Session { parse_sess: syntax::parse::ParseSess::new() }
    }

    pub fn new_from_file(name: String, source: String) -> Session {
        let session = Session::new();
        session.add_file(name, source);
        session
    }

    pub fn add_file(&self, name: String, source: String) {
        self.parse_sess.codemap().new_filemap(name, None, source);
    }
}
