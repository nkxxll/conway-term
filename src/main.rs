use clap::Parser;
use core::panic;
use std::process::exit;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

mod args;
mod database;
mod game;

fn main() {
    // get cli args
    let args = args::Cli::parse();
    match args.command {
        args::Command::Database(args) => {
            if args.list {
                let dbconn = database::DatabaseConnection::new("games.sqlite");
                let list = dbconn.list_games();
                println!("id width height rounds peek");
                for entry in list {
                    let first_line: String =
                        entry.data.into_iter().take(20).map(|x| x as char).collect();
                    println!(
                        "{} {} {} {} |{}|",
                        entry.id.unwrap(),
                        entry.width,
                        entry.height,
                        entry.rounds,
                        first_line
                    );
                }
                exit(1)
            }
            match args.get {
                Some(_) => todo!(),
                None => todo!(),
            }
        }
        args::Command::Play(args) => {
            // choose a random state
            let state = game::State::random();
            if args.based {
                let dbgs = database::DatabaseGameState::new(
                    game::WIDTH as usize,
                    game::HEIGHT as usize,
                    args.rounds.unwrap(),
                    state
                        .state
                        .iter()
                        .copied()
                        .flatten()
                        .map(|x| x as u8)
                        .collect::<Vec<u8>>(),
                );
                let dbconn = database::DatabaseConnection::new("games.sqlite");
                dbconn.create_tables();
                // todo: catch the error here
                let res = dbconn.insert_game(dbgs);
                match res {
                    Ok(_) => println!("saved game"),
                    Err(e) => {
                        panic!("error {:?}", e)
                    }
                }
            }
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
    }
}
