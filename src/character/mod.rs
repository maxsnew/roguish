extern crate std;
extern crate tcod;

use self::tcod::{Console, background_flag};
use self::std::cmp;
use util::{Bound, Position};

pub struct Character {
    pub position: Position,
    pub display:  char
}

impl Character {
    pub fn new(pos: Position, c: char) -> Character {
        Character { position: pos, display: c}
    }

    pub fn render(&self, con: &mut Console) {
        con.put_char(self.position.x, self.position.y, self.display, background_flag::Set);
    }

    pub fn update(&mut self, bound: Bound, moveX: int, moveY: int) {
        update_position(&mut self.position, moveX, moveY, bound)
    }
}

fn update_position(pos: &mut Position, moveX: int, moveY: int, bound: Bound) {
    pos.x = std::cmp::max(bound.min.x, cmp::min(bound.max.x - 1, pos.x + moveX));
    pos.y = std::cmp::max(bound.min.y, cmp::min(bound.max.y - 1, pos.y + moveY));
}
