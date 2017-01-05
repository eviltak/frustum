
// Copyright (c) 2017 Arav Singhal.

use syntax;


use session::{self, Session};
use ::internal::items;
use items::Crate;

pub fn parse_as_crate(session: &Session) -> Option<Crate> {
    if session.crate_root.is_none() {
        // TODO: Error handling
        panic!("No crate root defined!");
    }

    let parse_sess = session::parse_sess(session);

    let mut parser =
        syntax::parse::filemap_to_parser(&parse_sess,
                                         parse_sess
                                             .codemap()
                                             .get_filemap(&session.crate_root.as_ref().unwrap())
                                             .unwrap());

    match parser.parse_crate_mod() {
        Ok(krate) => Some(items::crate_from_ast_crate(&krate, &parse_sess)),
        Err(mut e) => {
            e.emit();
            println!("Code: {}",
                     e.code.as_ref().unwrap_or(&"No code".to_string()));
            e.cancel();
            None
        }
    }
}
