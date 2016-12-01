
// Copyright (c) 2016 Arav Singhal.

extern crate frustum;
extern crate syntex_pos as syntax_pos;

use std::path::PathBuf;

fn main() {
    println!("Hello, frustum!");

    // TODO: Move tests to tests/*.rs
    let session = frustum::Session::new_from_crate_root("src/lib.rs".to_string(),
                                                        "mod somemod;".to_string());
    session.add_file("src/somemod.rs".to_string(), "fn some() {}".to_string());

    session.parse_as_crate();

    let mut path = PathBuf::from(session.parse_sess.codemap().span_to_filename(syntax_pos::Span {
        lo: syntax_pos::BytePos(25),
        hi: syntax_pos::BytePos(26),
        expn_id: syntax_pos::ExpnId::from_u32(0),
    }));
    // path.pop();

    println!("{:?}", path);

    println!("{:?}", session.parse_sess.codemap().file_exists(&path));
}
