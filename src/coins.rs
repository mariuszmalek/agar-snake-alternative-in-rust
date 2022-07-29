use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::colors;
use crate::draw::*;
use crate::physics::{Direction, Position};

pub struct Coin {
    position: Position
}

impl Coin {
    pub fn new(position: Position) -> Self {
        let (x, y) = (position.x, position.y);
        
        Self {
            position: Position { x, y }
        }
    }
}
