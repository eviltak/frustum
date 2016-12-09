
// Copyright (c) 2016 Arav Singhal.

use items::*;
use ::internal;

use syntax;

pub fn crate_from_ast_crate(krate: &syntax::ast::Crate, sess: &syntax::parse::ParseSess) -> Crate {
    Crate {
        span: internal::pos::span_from_ast_span(&krate.span, sess),
        module: mod_from_ast_mod(&krate.module, sess),
        .. Default::default()
    }
}

pub fn mod_from_ast_mod(module: &syntax::ast::Mod, sess: &syntax::parse::ParseSess) -> Module {
    let mut conv_items = Vec::new();

    for item in &module.items {
        if let Some(converted) = item_from_ast_item(item, sess) {
            conv_items.push(converted);
        }
    }

    Module {
        span: internal::pos::span_from_ast_span(&module.inner, sess),
        items: conv_items,
        .. Default::default()
    }
}

pub fn item_from_ast_item(item: &syntax::ast::Item, sess: &syntax::parse::ParseSess) -> Option<Item> {
    // TODO: Find a way to simplify setting the name and span
    let name = (&item.ident.name.as_str()).to_string();
    let span = internal::pos::span_from_ast_span(&item.span, sess);

    match item.node {
        // TODO: Complete definition
        syntax::ast::ItemKind::Fn(ref decl, _, _, _, _, ref block) => {
            let mut func = fn_from_ast_fn(decl, block, sess);
            func.name = name; func.span = span;
            Some(Item::Fn(func))
        }
        syntax::ast::ItemKind::Mod(ref module) => {
            let mut conv_module = mod_from_ast_mod(module, sess);
            conv_module.name = name; conv_module.span = span;
            Some(Item::Mod(conv_module))
        }
        _ => None,
    }
}

pub fn fn_from_ast_fn(decl: &syntax::ast::FnDecl, block: &syntax::ast::Block, sess: &syntax::parse::ParseSess) -> Function {
    // TODO: Complete definition
    Function {
        body: block_from_ast_block(block, sess),
        .. Default::default()
    }
}

pub fn block_from_ast_block(block: &syntax::ast::Block, sess: &syntax::parse::ParseSess) -> Block {
    Block {
        span: internal::pos::span_from_ast_span(&block.span, sess),
        statements: block.stmts.iter().map(|stmt| statement_from_ast_stmt(&stmt, sess)).collect(),
    }
}

pub fn statement_from_ast_stmt(stmt: &syntax::ast::Stmt, sess: &syntax::parse::ParseSess) -> Statement {
    // TODO: Complete definition
    Statement {
        span: internal::pos::span_from_ast_span(&stmt.span, sess),
    }
}
