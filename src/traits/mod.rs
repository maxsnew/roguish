extern crate tcod;
use util::Bound;

pub trait Updates {
    fn update(&mut self, tcod::KeyState, Bound);
    fn render(&self, &mut tcod::Console);
}
