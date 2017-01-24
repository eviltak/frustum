
// Copyright (c) 2016 Arav Singhal.

use pos::Span;

pub enum Item {
    Type(Type),
    Typed(TypedItem),
    Fn(Function),
    Mod(Module),
    Use,
}

#[derive(Default)]
pub struct Crate {
    pub name: String,
    pub span: Span,

    pub module: Module,
}

#[derive(Default)]
pub struct Module {
    pub name: String,
    pub span: Span,

    pub items: Vec<Item>,
}

#[derive(Default)]
pub struct Type {
    pub name: String,
    pub span: Span,
}

#[derive(Default)]
pub struct TypedItem {
    pub name: String,
    pub span: Span,

    pub ty: Type,
}

#[derive(Default)]
pub struct Function {
    pub name: String,
    pub span: Span,

    pub args: Vec<TypedItem>,
    pub return_type: Type,
    pub body: Block,
}

#[derive(Default)]
pub struct Block {
    pub span: Span,

    pub statements: Vec<Statement>,
}

pub enum Statement {
    Local(Local),
    // TODO: Remove
    None,
}

pub struct Local {
    pub span: Span,

    pub pattern: Pattern,

    // TODO: Complete definition
}

pub enum Pattern {
    Ident(String),
    // TODO: Remove
    None,
}
