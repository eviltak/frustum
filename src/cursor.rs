
// Copyright (c) 2017 Arav Singhal.

use pos::Position;

pub struct Cursor {
    position: Position,
}

impl Cursor {
    pub fn new(position: Position) -> Cursor {
        Cursor {
            position: position,
        }
    }
}
