
// Copyright (c) 2016 Arav Singhal.

extern crate frustum;
extern crate syntex_syntax as syntax;
extern crate syntex_pos as syntax_pos;

fn main() {
    println!("Hello, frustum!");

    // TODO: Move tests to tests/*.rs
    let session = frustum::Session::new_from_crate_root("src/lib.rs".to_string(),
                                                        "mod somemod;".to_string());
    session.add_file("src/somemod.rs".to_string(), "fn some() {} fn some_other() {".to_string());

    let krate = session.parse_as_crate().unwrap();

    println!("{:?}", krate.module.items.len());
/*
    for item in krate.module.items {
        if let syntax::ast::ItemKind::Mod(ref module) = item.node {
            println!("Found module \"{0}\" with {1} items", item.ident, module.items.len());
            for inner_item in &module.items {
                match inner_item.node {
                    syntax::ast::ItemKind::Fn(_, _, _, _, _, _) => {
                        println!("Found fn \"{0}\" in module \"{1}\"", inner_item.ident, item.ident);
                    }
                    _ => println!("Some other item"),
                }
            }
        }
    }
    */
}
