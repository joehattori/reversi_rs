use crate::cli::Client;
use crate::game::board::{Board, Position};
use crate::message::{move_message, open_message, pass_message, ServerMessage};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    fn other_color(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
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
    Wait,
    PlayerTurn,
    OpponentTurn,
}

pub struct Game<'a> {
    client: &'a mut Client,
    state: State,
    player: Player,
    opponent: Player,
    board: Board,
    time: u32,
}

impl<'a> Game<'a> {
    fn empty(client: &'a mut Client, player: Player, opponent: Player, time: u32) -> Game {
        let board = Board::initial();
        Game {
            client: client,
            state: State::Wait,
            player: player,
            opponent: opponent,
            board: board,
            time: time,
        }
    }

    pub fn launch(client: &'a mut Client, name: &str) -> Result<Game<'a>, String> {
        match client.send_message(&open_message(name)) {
            Ok(_) => (),
            Err(_) => return Err("Couldn't start game.".to_string()),
        };

        // initialize players with dummy information.
        let player = Player {
            name: name.to_string(),
            color: Color::Black,
        };
        let opponent = Player {
            name: String::new(),
            color: Color::White,
        };
        Ok(Game::empty(client, player, opponent, 0))
    }

    pub fn main_loop(mut self) {
        loop {
            match self.state {
                State::Wait => {
                    match self.client.poll_message() {
                        Ok(msg) => match msg {
                            ServerMessage::Start {
                                color,
                                op_name,
                                remaining_time_ms,
                            } => {
                                self.handle_start(color, &op_name, remaining_time_ms);
                            }
                            ServerMessage::Bye { stat } => {
                                println!("{}", stat);
                                return;
                            }
                            _ => panic!("Unexpected message: expected Start or Bye"),
                        },
                        Err(s) => panic!("{}", s),
                    };
                }
                State::PlayerTurn => {
                    let (pos, next_board) = self.make_naive_move(self.player.color);
                    self.board = next_board;
                    self.state = State::OpponentTurn;
                    let msg = match pos {
                        Some(p) => move_message(p),
                        None => pass_message(),
                    };
                    self.client.send_message(&msg).unwrap();
                    self.board.print();
                    match self.client.poll_message() {
                        Ok(msg) => match msg {
                            ServerMessage::Ack { remaining_time_ms } => {
                                self.time = remaining_time_ms
                            }
                            ServerMessage::End {
                                result,
                                player_count,
                                op_count,
                                reason,
                            } => {
                                self.handle_end(result, player_count, op_count, &reason);
                            }
                            _ => panic!("Unexpected message: expected Ack"),
                        },
                        Err(s) => panic!("{}", s),
                    };
                }
                State::OpponentTurn => match self.client.poll_message() {
                    Ok(msg) => match msg {
                        ServerMessage::Move { pos } => {
                            self.handle_move(pos);
                        }
                        ServerMessage::End {
                            result,
                            player_count,
                            op_count,
                            reason,
                        } => {
                            self.handle_end(result, player_count, op_count, &reason);
                        }
                        _ => panic!("Unexpected message: expected Move or End"),
                    },
                    Err(s) => panic!("{}", s),
                },
            }
        }
    }

    fn handle_start(&mut self, color: Color, op_name: &str, time: u32) {
        self.board = Board::initial();
        self.player.color = color;
        self.opponent.color = color.other_color();
        self.opponent.name = op_name.to_string();
        self.time = time;
        self.state = match self.player.color {
            Color::Black => State::PlayerTurn,
            Color::White => State::OpponentTurn,
        };
    }

    fn handle_move(&mut self, pos: Option<Position>) {
        match pos {
            Some(p) => {
                self.board = self.board.flip(&p, self.opponent.color);
            }
            None => (),
        };
        self.board.print();
        self.state = State::PlayerTurn;
    }

    fn handle_end(&mut self, result: GameResult, player_count: u8, op_count: u8, reason: &str) {
        let result_str = match result {
            GameResult::Win => "You win!",
            GameResult::Lose => "You lose!",
            GameResult::Tie => "Tie game!",
        };
        println!(
            "{}. {}'s count: {}, {}'s count: {}, reason: {}",
            result_str, self.player.name, player_count, self.opponent.name, op_count, reason
        );
        self.state = State::Wait;
    }

    fn make_naive_move(&self, color: Color) -> (Option<Position>, Board) {
        let flippables = self.board.flippable_cells(color);
        for i in 0..64u8 {
            if flippables & 1 << i != 0 {
                let pos = Position { x: i % 8, y: i / 8 };
                let board = self.board.flip(&pos, color);
                return (Some(pos), board);
            }
        }
        (None, self.board)
    }
}
