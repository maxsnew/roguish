use character::Character;
use util::Bound;

pub struct Game {
    pub player: Character,
    pub friend: Character,
    pub bound:  Bound
}

impl Game {
    pub fn new(p: Character, f: Character, b: Bound) -> Game {
        Game { player: p, friend: f, bound: b}
    }
}
