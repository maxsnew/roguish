extern crate std;
extern crate tcod;

use self::tcod::{Console};
use std::rand::Rng;

use character::Character;
use traits::Updates;
use util::{Bound, Position};

pub struct NPC {
    pub character: Character
}

impl NPC { 
    pub fn new(x: int, y: int, c: char) -> NPC {
        NPC { character: Character::new(Position::new(x, y), c) }
    }
}

impl Updates for NPC {
    fn update(&mut self, _keypress: tcod::KeyState, bound: Bound) {
        let moveFX = std::rand::task_rng().gen_range(-1i, 2i);
        let moveFY = std::rand::task_rng().gen_range(-1i, 2i);
        self.character.update(bound, moveFX, moveFY);
    }
    fn render(&self, con: &mut tcod::Console) {
        self.character.render(con);
    }
}

