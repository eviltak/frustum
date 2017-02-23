
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
pub struct Position(pub u32);

impl Position {
    pub fn from_u32(n: u32) -> Position { Position(n) }
    pub fn to_u32(&self) -> u32 { let Position(n) = *self; n }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position(self.to_u32() + rhs.to_u32())
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Position {
        Position(self.to_u32() - rhs.to_u32())
    }
}
