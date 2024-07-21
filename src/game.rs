use rand::random;
use std::{thread, time::Duration};

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
    pub(crate) state: [[char; WIDTH as usize]; HEIGHT as usize],
}

impl Default for State {
    fn default() -> Self {
        State {
            state: [[' '; WIDTH as usize]; HEIGHT as usize],
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
}

#[derive(Debug, Default)]
pub(crate) struct Game {
    pub(crate) state: State,
}

impl Game {
    pub(crate) fn evaluate(&mut self) {
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
}

pub(crate) fn sleep_time() {
    thread::sleep(Duration::from_millis(SLEEP_TIME));
}
