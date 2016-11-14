
use items::*;

use syntax;

pub fn item_from_ast_item(item: &syntax::ast::Item, sess: &syntax::parse::ParseSess) -> Option<Item> {
    let name: String = (&item.ident.name.as_str()).to_string();
    let span = ::internal::pos::span_from_ast_span(&item.span, sess);

    match item.node {
        syntax::ast::ItemKind::Fn(ref decl, _, _, _, _, ref block) => {
            unimplemented!()
        }
        _ => None,
    }
}
