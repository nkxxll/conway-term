use clap::Parser;
use std::process::exit;

mod args;
mod database;
mod game;
mod rle;

fn main() {
    // get cli args
    let args = args::Cli::parse();
    match args.command {
        args::Command::Database(args) => {
            match (args.list, args.get) {
                (true, None) => {
                    // todo this is a task for the database not the main file
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
                // get game from database
                (false, Some(index)) => {
                    let dbconn = database::DatabaseConnection::new("games.sqlite");
                    let game = dbconn.get_game_by_idx(index);
                    let state: game::State = rle::state_from_rle(
                        game.data,
                        game.width.try_into().unwrap(),
                        game.height.try_into().unwrap(),
                    );
                    game::Game::run(state, false, Some(game.rounds));
                }
                _ => println!("Error: exactly one of --list or --get must be provided."),
            }
        }
        args::Command::Play(args) => {
            // choose a random state
            let state = game::State::random();
            // save a game
            if args.based {
                let dbgs = database::DatabaseGameState::new(
                    game::WIDTH as usize,
                    game::HEIGHT as usize,
                    args.rounds.unwrap(),
                    rle::state_to_rle(&state),
                );
                let dbconn = database::DatabaseConnection::new("games.sqlite");
                dbconn.create_tables();
                // todo: catch the error here
                let res = dbconn.insert_game(dbgs);
                match res {
                    Ok(_) => (),
                    Err(e) => {
                        panic!("error {:?}", e)
                    }
                }
            }
            // create a game instance
            game::Game::run(state, args.silent, args.rounds);
        }
    }
}
