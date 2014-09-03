extern crate tcod;
use tcod::{Console, background_flag, key_code, Special};
use std::cmp;

struct State {
    player: Position,
    friend: Position
}

struct Position {
    x: int,
    y: int
}

static WINDOW_WIDTH:  int = 80;
static WINDOW_HEIGHT: int = 50;

fn main() {
    let mut con   = Console::init_root(WINDOW_WIDTH, WINDOW_HEIGHT, "libtcod Rust tutorial", false);
    let mut exit  = false;
    let mut state = 
        State { player: Position { x: WINDOW_WIDTH  / 2,
                                   y: WINDOW_HEIGHT / 2 } 
              , friend: Position { x: WINDOW_WIDTH  / 2 + 1,
                                   y: WINDOW_HEIGHT / 2 + 1}};
    // Initial render
    render(&mut con, state);
    while !(Console::window_closed() || exit) {
        // wait for user input
        let keypress = con.wait_for_keypress(true);

        // update game state
        match keypress.key {
            Special(key_code::Escape) => exit = true,
            Special(key_code::Up)     => state.player.y = cmp::max(0, state.player.y - 1),
            Special(key_code::Down)   => state.player.y = cmp::min(WINDOW_HEIGHT - 1, state.player.y + 1),
            Special(key_code::Left)   => state.player.x = cmp::max(0, state.player.x - 1),
            Special(key_code::Right)  => state.player.x = cmp::min(WINDOW_WIDTH - 1, state.player.x + 1),
            _                         => {}
        }

        // render
        render(&mut con, state);
    }
}

fn render(con: &mut Console, state: State) {
    con.clear();
    match state {
        State { player: Position {x: px, y: py}
              , friend: Position {x: fx, y: fy}} => {
            con.put_char(px, py, '@', background_flag::Set);
            if !(px == fx && py == fy) {
                con.put_char(fx, fy, 'd', background_flag::Set)
            }
        }
    };
    con.flush();
}
