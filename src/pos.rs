
// Copyright (c) 2016 Arav Singhal.

// Span

#[derive(Default)]
pub struct Span {
    pub start: Position,
    pub end: Position,
    pub file: String
}

// Position
// Copyright 2012-2013 The Rust Project Developers.

use std::ops::{Add, Sub};

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
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
