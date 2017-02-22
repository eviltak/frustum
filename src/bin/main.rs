
// Copyright (c) 2016 Arav Singhal.

extern crate frustum;
extern crate syntex_syntax as syntax;
extern crate syntex_pos as syntax_pos;

struct ItemVisitor;

impl<'a> syntax::visit::Visitor<'a> for ItemVisitor {
    fn visit_item(&mut self, b: &'a syntax::ast::Item) {
        println!("Item visited");
    }
}

fn main() {
    //println!("Hello, frustum!");

    let sess = syntax::parse::ParseSess::new();

    let mut visitor = ItemVisitor;

    let result = syntax::parse::parse_crate_from_source_str("some_crate".to_string(), "
    use something;
    use something_else;

    //mod some_mod;

    fn main() {
        println!(\"Hello world\");
    }".to_string(), &sess);

    let krate = result.ok().unwrap();

    syntax::visit::walk_crate(&mut visitor, &krate);

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

}
