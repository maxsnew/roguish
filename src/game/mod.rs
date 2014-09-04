use util::Bound;

pub struct Game {
    pub bound:  Bound
}

impl Game {
    pub fn new(b: Bound) -> Game {
        Game { bound: b }
    }
}
