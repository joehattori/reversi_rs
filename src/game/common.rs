use crate::cli::Client;
use crate::message::client::{move_message, open_message};
use crate::message::server::ServerMessage;

#[derive(Copy, Clone)]
pub enum Color {
    Black,
    White,
}

impl Color {
    fn other_color(&self) -> Color {
        match self {
            Black => Color::White,
            White => Color::Black,
        }
    }
}

pub enum GameResult {
    Win,
    Lose,
    Tie,
}

pub struct Player {
    name: String,
    color: Color,
}

pub enum State {
    Wait,
    PlayerTurn,
    OpponentTurn,
    End,
}

pub struct Board {
    player: u64,
    opponent: u64,
}

impl Board {
    pub fn new() -> Board {
        Board {
            player: 0u64,
            opponent: 0u64,
        }
    }
}

pub struct Game<'a> {
    client: &'a mut Client,
    state: State,
    player: Player,
    opponent: Player,
    board: Board,
    time: u16,
}

impl<'a> Game<'a> {
    fn new(client: &'a mut Client, player: Player, opponent: Player, time: u16) -> Game {
        Game {
            client: client,
            state: State::Wait,
            player: player,
            opponent: opponent,
            board: Board::new(),
            time: time,
        }
    }

    pub fn launch<'b>(client: &'a mut Client, name: &str) -> Result<Game<'a>, &'b str> {
        match client.send_message(open_message(name)) {
            Ok(_) => (),
            Err(_) => return Err("Couldn't start game."),
        };
        match client.poll_message() {
            Ok(ServerMessage::Start {
                color,
                op_name,
                remaining_time_ms,
            }) => {
                let player = Player {
                    name: name.to_string(),
                    color: color,
                };
                let opponent = Player {
                    name: op_name,
                    color: color.other_color(),
                };
                Ok(Game::new(client, player, opponent, remaining_time_ms))
            }
            _ => return Err("Unexpected message: Start messgae expected."),
        }
    }

    pub fn main_loop(&self) {
        loop {
            match self.state {
                State::Wait => (),
                State::PlayerTurn => (),
                State::OpponentTurn => (),
                State::End => return,
            }
        }
    }
}
