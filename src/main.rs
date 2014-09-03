extern crate tcod;

use tcod::{Console, background_flag, key_code, Special};
use std::cmp;
use std::rand::Rng;

struct State {
    player: Character,
    friend: Character
}

struct Character {
    position: Position,
    display:  char
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
        State { player: Character {
                          position: 
                            Position { x: WINDOW_WIDTH  / 2
                                     , y: WINDOW_HEIGHT / 2 
                            }
                        , display: '@'
                }
              , friend: Character { 
                          position:
                            Position { x: DOG_START
                                     , y: DOG_START 
                            }
                        , display: 'd'
              }
        };
    // Initial render
    render(&mut con, state);
    while !(Console::window_closed() || exit) {
        // wait for user input
        let keypress = con.wait_for_keypress(true);

        // update game state
        let moveFX = std::rand::task_rng().gen_range(-1i, 2i);
        let moveFY = std::rand::task_rng().gen_range(-1i, 2i);
        state.friend.update(moveFX, moveFY);
        match keypress.key {
            Special(key_code::Escape) => exit = true,
            Special(key_code::Up)     => state.player.update(0, -1),
            Special(key_code::Down)   => state.player.update(0, 1),
            Special(key_code::Left)   => state.player.update(-1, 0),
            Special(key_code::Right)  => state.player.update(1, 0),
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
        State { player: playerC
              , friend: friendC } => {
            friendC.render(con);
            playerC.render(con);
        }
    };
    con.flush();
}

impl Character { 
    fn render(&self, con: &mut Console) {
        con.put_char(self.position.x, self.position.y, self.display, background_flag::Set);
    }

    fn update(&mut self, moveX: int, moveY: int) {
        update_position(&mut self.position, moveX, moveY)
    }
}
