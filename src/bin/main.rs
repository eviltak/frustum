
// Copyright (c) 2016 Arav Singhal.

extern crate frustum;
extern crate syntex_syntax as syntax;
extern crate syntex_pos as syntax_pos;

fn main() {
    println!("Hello, frustum!");
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
