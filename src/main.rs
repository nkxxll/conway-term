use clap::Parser;
use rand::random;
use std::{thread, time::Duration};

const HEIGHT: isize = 50;
const WIDTH: isize = 50;
const CLEAR_OCT: &str = "\033c";
const CLEAR_HEX: &str = "\x1Bc";
const SLEEP_TIME: u64 = 100;
const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 1),
    (1, 0),
    (1, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn clear_hex() {
    print!("{}", CLEAR_HEX);
}
fn clear_oct() {
    print!("{}", CLEAR_OCT);
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 0, help = "number of rounds")]
    rounds: usize,
    #[arg(
        short,
        long,
        default_value_t = false,
        requires = "rounds",
        action,
        help = "run silent"
    )]
    silent: bool,
}

#[derive(Debug)]
struct State {
    state: [[char; WIDTH as usize]; HEIGHT as usize],
}

impl Default for State {
    fn default() -> Self {
        State {
            state: [[' '; WIDTH as usize]; HEIGHT as usize],
        }
    }
}

impl State {
    fn random() -> Self {
        let mut res = State::default();
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if random::<bool>() {
                    res.state[row as usize][col as usize] = '#';
                } else {
                    res.state[row as usize][col as usize] = ' ';
                }
            }
        }
        res
    }
}

#[derive(Debug, Default)]
struct Game {
    state: State,
}

impl Game {
    fn evaluate(&mut self) {
        let mut new_state: State = State::default();
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let current = self.state.state[row as usize][col as usize];
                let mut count = 0;
                for dir in DIRECTIONS {
                    let (pr, pc) = dir;
                    if row + pr < 0 || row + pr >= HEIGHT || col + pc < 0 || col + pc >= WIDTH {
                        continue;
                    }
                    if self.state.state[(row + pr) as usize][(col + pc) as usize] == '#' {
                        count += 1;
                    }
                }
                if (count == 2 || count == 3) && current == '#' {
                    new_state.state[row as usize][col as usize] = '#';
                }
                if count > 3 && current == '#' {
                    new_state.state[row as usize][col as usize] = ' ';
                }
                if count == 3 {
                    new_state.state[row as usize][col as usize] = '#';
                }
                if count < 2 && current == '#' {
                    new_state.state[row as usize][col as usize] = ' ';
                }
            }
        }
        self.state = new_state;
    }

    fn print(&self) {
        println!(
            "{}",
            self.state
                .state
                .iter()
                .map(|line| {
                    let mut res = line.iter().collect::<String>();
                    res.push('\n');
                    res
                })
                .collect::<String>()
        );
    }
    fn new(state: State) -> Self {
        Game { state }
    }
}

fn sleep_time() {
    thread::sleep(Duration::from_millis(SLEEP_TIME));
}

fn main() {
    let args = Cli::parse();
    let state = State::random();
    let mut game = Game::new(state);
    if args.rounds == 0 {
        loop {
            clear_hex();
            game.evaluate();
            game.print();
            sleep_time();
        }
    } else {
        for r in 0..args.rounds {
            game.evaluate();
            if !args.silent || r == (args.rounds - 1) {
                clear_hex();
                game.print();
                if !args.silent {
                    sleep_time();
                }
            }
        }
    }
}
