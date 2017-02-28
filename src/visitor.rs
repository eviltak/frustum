
// Copyright (c) 2017 Arav Singhal.

use syntax;
use syntax_pos;

pub struct AstVisitor {
    // TODO: Convert bools to a builder type struct instead
    visiting_local: bool,
    visiting_pat_in_local: bool,
}

impl AstVisitor {
    pub fn new() -> AstVisitor {
        AstVisitor {
            visiting_local: false,
            visiting_pat_in_local: false,
        }
    }
}

impl<'a> syntax::visit::Visitor<'a> for AstVisitor {
    fn visit_local(&mut self, ast_local: &'a syntax::ast::Local) {
        self.visiting_local = true;

        syntax::visit::walk_local(self, ast_local);

        self.visiting_local = false;
    }

    fn visit_pat(&mut self, ast_pat: &'a syntax::ast::Pat) {
        if !self.visiting_local {
            return
        }

        self.visiting_pat_in_local = true;

        syntax::visit::walk_pat(self, ast_pat);

        self.visiting_pat_in_local = false;
    }

    fn visit_ident(&mut self, ast_span: syntax_pos::Span, ast_ident: syntax::ast::Ident) {
        if !self.visiting_pat_in_local {
            return
        }

        println!("Found local \"{}\" at {:?}", ast_ident.name.as_str(), ast_span);

        syntax::visit::walk_ident(self, ast_span, ast_ident);
    }
}
