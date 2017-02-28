
// Copyright (c) 2016 Arav Singhal.

extern crate frustum;
extern crate syntex_syntax as syntax;
extern crate syntex_pos as syntax_pos;

fn parse_crate(session: &frustum::Session) {
    let mut parser =
        syntax::parse::filemap_to_parser(&session.parse_sess,
                                         session.parse_sess
                                             .codemap()
                                             .get_filemap(&session.crate_root.as_ref().unwrap())
                                             .unwrap());

    match parser.parse_crate_mod() {
        Ok(krate) => {
            let mut ast_visitor = frustum::visitor::AstVisitor::new();
            syntax::visit::walk_crate(&mut ast_visitor, &krate);
        }
        Err(mut e) => {
            e.emit();
            println!("Code: {}",
                     e.code.as_ref().unwrap_or(&"No code".to_string()));
            e.cancel();
        }
    }
}

fn main() {
    println!("Hello, frustum!\n");

    let session = frustum::Session::new_from_crate_root("some_crate".to_string(),
                                                        "fn main() {
                                                             let local_x = 3;
                                                             let local_y = 4;
                                                         }"
                                                            .to_string());

    parse_crate(&session)
}
