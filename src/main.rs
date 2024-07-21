use clap::Parser;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

mod args;
mod game;

fn main() {
    // get cli args
    let args = args::Cli::parse();
    // choose a random state
    let state = game::State::random();
    // create a game instance
    let mut game_instance = game::Game::new(state);
    game::print_esc_code(game::ENTER_ALT);
    match args.rounds {
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
                game::print_esc_code(game::CLEAR_HEX);
                game_instance.evaluate();
                game_instance.print();
                game::sleep_time();
            }
            game::print_esc_code(game::LEAVE_ALT);
            game_instance.print();
        }
        Some(rounds) => {
            match args.silent {
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
                    game::print_esc_code(game::ENTER_ALT);
                    for _ in 0..rounds {
                        game_instance.evaluate();
                        game_instance.print();
                        game::sleep_time();
                        game::print_esc_code(game::CLEAR_HEX);
                        if !rx.try_recv().unwrap_or(true) {
                            break;
                        }
                    }
                    game::print_esc_code(game::LEAVE_ALT);
                    // print the final state again
                    game_instance.print();
                }
            }
        }
    }
}
