extern crate tcod;

use self::tcod::{key_code, Special};

use character::Character;
use traits::Updates;
use util::{Bound, Position};

pub struct Player {
    pub character: Character
}

impl Player {
    pub fn new(x: int, y: int, c: char) -> Player {
        Player { character: Character::new(Position::new(x, y), c) }
    }
}

impl Updates for Player {
    fn update(&mut self, keypress: tcod::KeyState, bound: Bound) {
        match keypress.key {
            Special(key_code::Up)     => self.character.update(bound, 0, -1),
            Special(key_code::Down)   => self.character.update(bound, 0, 1),
            Special(key_code::Left)   => self.character.update(bound, -1, 0),
            Special(key_code::Right)  => self.character.update(bound, 1, 0),
            _                         => {}
        }
    }

    fn render(&self, con: &mut tcod::Console) {
        self.character.render(con);
    }
}
