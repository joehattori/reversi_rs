extern crate clap;

pub mod cli;

use clap::{App, Arg};
use cli::Client;

const DEFAULT_PORT: &str = "3000";
const DEFAULT_HOST: &str = "localhost";

fn main() {
    let matches = App::new("Let's Reversi")
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
    let port = match matches.value_of("port").unwrap_or(DEFAULT_PORT).parse() {
        Ok(n) => n,
        Err(_) => panic!("Invalid port specified."),
    };
    let client = Client::new(host, port);
}
