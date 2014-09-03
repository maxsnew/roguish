extern crate tcod;

use tcod::{Console, background_flag, key_code, Special};
use std::cmp;
use std::rand::Rng;

struct State {
    player: Position,
    friend: Position,
}

struct Position {
    x: int,
    y: int
}

static WINDOW_WIDTH:  int = 80i;
static WINDOW_HEIGHT: int = 50i;
static DOG_START: int = 10i;

fn main() {
    let mut con   = Console::init_root(WINDOW_WIDTH, WINDOW_HEIGHT, "libtcod Rust tutorial", false);
    let mut exit  = false;
    let mut state =
        State { player: Position { x: WINDOW_WIDTH  / 2
                                 , y: WINDOW_HEIGHT / 2 }
              , friend: Position { x: DOG_START
                                 , y: DOG_START } 
        };
    // Initial render
    render(&mut con, state);
    while !(Console::window_closed() || exit) {
        // wait for user input
        let keypress = con.wait_for_keypress(true);

        // update game state
        let moveFX = std::rand::task_rng().gen_range(-1i, 2i);
        let moveFY = std::rand::task_rng().gen_range(-1i, 2i);
        update_position(&mut state.friend, moveFX, moveFY);
        match keypress.key {
            Special(key_code::Escape) => exit = true,
            Special(key_code::Up)     => update_position(&mut state.player, 0, -1),
            Special(key_code::Down)   => update_position(&mut state.player, 0, 1),
            Special(key_code::Left)   => update_position(&mut state.player, -1, 0),
            Special(key_code::Right)  => update_position(&mut state.player, 1, 0),
            _                         => {}
        }

        // render
        render(&mut con, state);
    }
}

fn update_position(pos: &mut Position, moveX: int, moveY: int) {
    pos.x = std::cmp::max(0, cmp::min(WINDOW_WIDTH  - 1, pos.x + moveX));
    pos.y = std::cmp::max(0, cmp::min(WINDOW_HEIGHT - 1, pos.y + moveY));
}

fn render(con: &mut Console, state: State) {
    con.clear();
    match state {
        State { player: Position {x: px, y: py}
              , friend: Position {x: fx, y: fy} } => {
            con.put_char(px, py, '@', background_flag::Set);
            if !(px == fx && py == fy) {
                con.put_char(fx, fy, 'd', background_flag::Set)
            }
        }
    };
    con.flush();
}
