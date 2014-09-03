extern crate tcod;
extern crate roguish;

use tcod::{Console, key_code, Special};
use std::rand::Rng;
use roguish::util::{Bound, Position};
use roguish::character::Character;

struct Game {
    player: Character,
    friend: Character,
    bound:  Bound
}

static WINDOW_WIDTH:  int = 80i;
static WINDOW_HEIGHT: int = 50i;
static DOG_START: int = 10i;

fn main() {
    let mut con   = Console::init_root(WINDOW_WIDTH, WINDOW_HEIGHT, "libtcod Rust tutorial", false);
    let mut exit  = false;
    let mut game = Game::new( Character::new( Position::new(WINDOW_WIDTH  / 2, WINDOW_HEIGHT / 2)
                                            , '@')
                            , Character::new( Position::new(DOG_START, DOG_START)
                                            , 'd')
                            , Bound::new( Position::new(0, 0)
                                        , Position::new(WINDOW_WIDTH, WINDOW_HEIGHT)));
    // Initial render
    render(&mut con, game);
    while !(Console::window_closed() || exit) {
        // wait for user input
        let keypress = con.wait_for_keypress(true);

        // update game game
        let moveFX = std::rand::task_rng().gen_range(-1i, 2i);
        let moveFY = std::rand::task_rng().gen_range(-1i, 2i);
        game.friend.update(game.bound, moveFX, moveFY);
        match keypress.key {
            Special(key_code::Escape) => exit = true,
            Special(key_code::Up)     => game.player.update(game.bound, 0, -1),
            Special(key_code::Down)   => game.player.update(game.bound, 0, 1),
            Special(key_code::Left)   => game.player.update(game.bound, -1, 0),
            Special(key_code::Right)  => game.player.update(game.bound, 1, 0),
            _                         => {}
        }

        // render
        render(&mut con, game);
    }
}

fn render(con: &mut Console, game: Game) {
    con.clear();
    match game {
        Game { player: playerC
             , friend: friendC 
             , bound: _} => {
            friendC.render(con);
            playerC.render(con);
        }
    };
    con.flush();
}

impl Game {
    fn new(p: Character, f: Character, b: Bound) -> Game {
        Game { player: p, friend: f, bound: b}
    }
}
