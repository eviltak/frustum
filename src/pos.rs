
// Copyright (c) 2016 Arav Singhal.

use syntax;
use syntax_pos;

use syntax_pos::Pos;

// Span

pub struct Span {
    pub start: Position,
    pub end: Position,
    pub file: String
}

impl Span {
    fn from_ast_span(span: &syntax_pos::Span, sess: &syntax::parse::ParseSess) -> Span {
        let code_map = sess.codemap();

        Span {
            start: Position::from_usize(code_map.bytepos_to_file_charpos(span.lo).to_usize()),
            end: Position::from_usize(code_map.bytepos_to_file_charpos(span.hi).to_usize()),
            file: code_map.span_to_filename(*span)
        }
    }
}

// Copyright 2012-2013 The Rust Project Developers.
// Position

use std::ops::{Add, Sub};

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Position(pub usize);

impl Position {
    pub fn from_usize(n: usize) -> Position { Position(n) }
    pub fn to_usize(&self) -> usize { let Position(n) = *self; n }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position(self.to_usize() + rhs.to_usize())
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Position {
        Position(self.to_usize() - rhs.to_usize())
    }
}
