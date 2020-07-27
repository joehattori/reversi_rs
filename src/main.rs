extern crate clap;

mod cli;
mod game;
mod message;

use clap::Arg;

use crate::game::base::Game;
use crate::game::opening_db::load_from_file;

const DEFAULT_PORT: &str = "3000";
const DEFAULT_HOST: &str = "localhost";
const DEFAULT_NAME: &str = "Joe";

fn main() {
    let matches = clap::App::new("Let's Reversi")
        .arg(
            Arg::with_name("host")
                .short("H")
                .value_name("HOST")
                .help("Sets a host for this reversi match")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .value_name("Port")
                .help("Sets a port for this reversi match")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .value_name("Name")
                .help("Sets a player's name for this reversi match")
                .takes_value(true),
        )
        .get_matches();

    let host = matches.value_of("host").unwrap_or(DEFAULT_HOST);
    let port: u32 = matches
        .value_of("port")
        .unwrap_or(DEFAULT_PORT)
        .parse()
        .expect("Invalid port specified.");
    let name = matches.value_of("name").unwrap_or(DEFAULT_NAME);
    println!("Loading opening db...");
    load_from_file();
    let game = Game::launch(host, port, name).unwrap();
    game.main_loop();
    println!("Game Ended!");
}
