
// Copyright (c) 2016 Arav Singhal.

use items::*;
use ::internal::*;

use syntax;

pub fn item_from_ast_item(item: &syntax::ast::Item, sess: &syntax::parse::ParseSess) -> Option<Item> {
    let name = (&item.ident.name.as_str()).to_string();
    let span = pos::span_from_ast_span(&item.span, sess);

    match item.node {
        syntax::ast::ItemKind::Fn(ref decl, _, _, _, _, ref block) => {
            // TODO: Complete definition
            unimplemented!()
        }
        _ => None,
    }
}

pub fn fn_from_ast_fn(decl: &syntax::ast::FnDecl, block: &syntax::ast::Block) -> Option<Function> {
    // TODO: Complete definition
    unimplemented!()
}

pub fn block_from_ast_block(block: &syntax::ast::Block, sess: &syntax::parse::ParseSess) -> Block {
    Block {
        span: pos::span_from_ast_span(&block.span, sess),
        statements: block.stmts.iter().map(|stmt| statement_from_ast_stmt(&stmt, sess)).collect(),
    }
}

pub fn statement_from_ast_stmt(stmt: &syntax::ast::Stmt, sess: &syntax::parse::ParseSess) -> Statement {
    Statement {
        span: pos::span_from_ast_span(&stmt.span, sess),
    }
}
