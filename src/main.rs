extern crate tcod;
extern crate roguish;

use tcod::{Console, key_code, Special};

use roguish::game::Game;
use roguish::npc::NPC;
use roguish::player::Player;
use roguish::traits::Updates;
use roguish::util::{Bound, Position};

static WINDOW_WIDTH:  int = 80i;
static WINDOW_HEIGHT: int = 50i;
static DOG_START: int = 10i;

fn main() {
    let mut con  = Console::init_root(WINDOW_WIDTH, WINDOW_HEIGHT, "libtcod Rust tutorial", false);
    let mut exit = false;
    let c        = box Player::new(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2, '@') as Box<Updates>;
    let d        = box NPC::new(DOG_START, DOG_START, 'd') as Box<Updates>;
    let bound    = Bound::new( Position::new(0, 0), Position::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    let mut objs: Vec<Box<Updates>> = vec![
        d, c
    ];
    let game = Game::new(bound);
    // Initial render
    render(&mut con, &objs);
    while !(Console::window_closed() || exit) {
        // wait for user input
        let keypress = con.wait_for_keypress(true);

        // update game game
        match keypress.key {
            Special(key_code::Escape) => exit = true,
            _                         => {}
        }
        update(game, &mut objs, keypress);

        // render
        render(&mut con, &objs);
    }
}

fn update(game: Game, objs: &mut Vec<Box<Updates>>, keypress: tcod::KeyState) {
    for i in objs.mut_iter() {
        i.update(keypress, game.bound);
    }
}
fn render(con: &mut Console, objs: &Vec<Box<Updates>>) {
    con.clear();
    for i in objs.iter() {
        i.render(con);
    }
    con.flush();
}
