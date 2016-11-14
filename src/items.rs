
// Copyright (c) 2016 Arav Singhal.

use pos::Span;

pub enum Item {
    Type(Type),
    Typed(TypedItem),
    Fn(Function),
}

pub struct Type {
    pub name: String,
    pub span: Span,
}

pub struct TypedItem {
    pub name: String,
    pub span: Span,

    pub ty: Type,
}

pub struct Function {
    pub name: String,
    pub span: Span,

    pub args: Vec<TypedItem>,
    pub return_type: Type,
}
