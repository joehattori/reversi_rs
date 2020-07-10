use crate::cli::Client;
use crate::game::board::Board;
use crate::message::client::{move_message, open_message};
use crate::message::server::ServerMessage;

#[derive(Copy, Clone, PartialEq, Eq)]
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
    pub color: Color,
}

enum State {
    Start,
    Wait,
    PlayerTurn,
    OpponentTurn,
    End,
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
        let board = Board::initial(&player);
        Game {
            client: client,
            state: State::Start,
            player: player,
            opponent: opponent,
            board: board,
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
                State::Start => {
                    if self.player.color == Color::Black {
                        self.make_move(&self.player);
                    }
                }
                State::Wait => (),
                State::PlayerTurn => (),
                State::OpponentTurn => (),
                State::End => return,
            }
        }
    }

    fn make_move(&self, hand: &Player) {
        //TODO: select a valid cell
    }
}
