use rand::random;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

pub(crate) const HEIGHT: isize = 30;
pub(crate) const WIDTH: isize = 100;
pub(crate) const CLEAR_HEX: &str = "\x1Bc";
pub(crate) const ENTER_ALT: &str = "\x1B[?1049h";
pub(crate) const LEAVE_ALT: &str = "\x1B[?1049l";
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

pub(crate) fn print_esc_code(code: &str) {
    print!("{}", code);
}

#[derive(Debug)]
pub(crate) struct State {
    pub(crate) state: Vec<Vec<char>>,
    pub(crate) width: isize,
    pub(crate) height: isize,
}

impl Default for State {
    fn default() -> Self {
        State {
            state: vec![vec![' '; WIDTH.try_into().unwrap()]; HEIGHT.try_into().unwrap()],
            width: WIDTH,
            height: HEIGHT,
        }
    }
}

impl State {
    pub(crate) fn random() -> Self {
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

    pub(crate) fn new(state: Vec<Vec<char>>, width: isize, height: isize) -> Self {
        Self {
            state,
            width,
            height,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Game {
    pub(crate) state: State,
}

impl Game {
    pub(crate) fn evaluate(&mut self) {
        let mut new_state: State = State::default();
        for row in 0..self.state.height {
            for col in 0..self.state.width {
                let current = self.state.state[row as usize][col as usize];
                let mut count = 0;
                for dir in DIRECTIONS {
                    let (pr, pc) = dir;
                    if row + pr < 0
                        || row + pr >= self.state.height
                        || col + pc < 0
                        || col + pc >= self.state.width
                    {
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

    pub(crate) fn print(&self) {
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
    pub(crate) fn new(state: State) -> Self {
        Game { state }
    }

    pub(crate) fn run(state: State, silent: bool, rounds: Option<usize>) {
        // create a game instance
        let mut game_instance = Game::new(state);
        print_esc_code(ENTER_ALT);
        match rounds {
            None => {
                let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
                ctrlc::set_handler(move || {
                    // stopping the endless game with ctrl-c is expected so we exit with 0
                    tx.send(false).unwrap();
                })
                .expect("Error setting Ctrl-C handler");

                // Following code does the actual work, and can be interrupted by pressing
                // Ctrl-C. As an example: Let's wait a few seconds.
                while rx.try_recv().unwrap_or(true) {
                    print_esc_code(CLEAR_HEX);
                    game_instance.evaluate();
                    game_instance.print();
                    sleep_time();
                }
                print_esc_code(LEAVE_ALT);
                game_instance.print();
            }
            Some(rounds) => {
                match silent {
                    // if silent we can evaluate the game state after n rounds without printing
                    true => {
                        for _ in 0..rounds {
                            game_instance.evaluate();
                        }
                        game_instance.print()
                    }
                    false => {
                        let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
                        ctrlc::set_handler(move || {
                            tx.send(false).unwrap();
                        })
                        .expect("Error setting Ctrl-C handler");

                        // Following code does the actual work, and can be interrupted by pressing
                        // Ctrl-C. As an example: Let's wait a few seconds.
                        print_esc_code(ENTER_ALT);
                        for _ in 0..rounds {
                            game_instance.evaluate();
                            game_instance.print();
                            sleep_time();
                            print_esc_code(CLEAR_HEX);
                            if !rx.try_recv().unwrap_or(true) {
                                break;
                            }
                        }
                        print_esc_code(LEAVE_ALT);
                        // print the final state again
                        game_instance.print();
                    }
                }
            }
        }
    }
}

pub(crate) fn sleep_time() {
    thread::sleep(Duration::from_millis(SLEEP_TIME));
}
