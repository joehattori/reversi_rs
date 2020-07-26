use crate::cli::Client;
use crate::game::board::Board;
use crate::game::square::Square;
use crate::game::strategy;
use crate::game::strategy::Strategy;
use crate::message::{move_message, open_message, pass_message, ServerMessage};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Dark,
    Light,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::Dark => Color::Light,
            Color::Light => Color::Dark,
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

pub struct Game {
    client: Client,
    state: State,
    player: Player,
    opponent: Player,
    pub board: Board,
    time: u32,
    strategy: Box<dyn Strategy>,
}

impl Game {
    const ENDGAME_BORDER: u8 = 20;

    fn empty(client: Client, player: Player, opponent: Player, time: u32) -> Game {
        let board = Board::initial();
        Game {
            client: client,
            state: State::Wait,
            player: player,
            opponent: opponent,
            board: board,
            time: time,
            strategy: Box::new(Strategy::default()),
        }
    }

    pub fn launch(host: &str, port: u32, name: &str) -> Result<Game, String> {
        let mut client = Client::new(host, port);
        match client.send_message(&open_message(name)) {
            Ok(_) => (),
            Err(_) => return Err("Couldn't start game.".to_string()),
        };

        // initialize players with dummy information.
        let player = Player {
            name: name.to_string(),
            color: Color::Dark,
        };
        let opponent = Player {
            name: String::new(),
            color: Color::Light,
        };
        Ok(Game::empty(client, player, opponent, 0))
    }

    pub fn main_loop(mut self) {
        loop {
            match self.state {
                State::Wait => {
                    let is_bye = self.handle_wait();
                    if is_bye {
                        return;
                    }
                }
                State::PlayerTurn => self.handle_player_turn(),
                State::OpponentTurn => self.handle_opponent_turn(),
            }
        }
    }

    fn handle_wait(&mut self) -> bool {
        match self.client.poll_message() {
            Ok(msg) => match msg {
                ServerMessage::Start {
                    color,
                    op_name,
                    remaining_time_ms,
                } => {
                    self.reset();
                    self.on_start_message(color, &op_name, remaining_time_ms);
                    false
                }
                ServerMessage::Bye { stat } => {
                    println!("{}", stat);
                    true
                }
                _ => panic!("Unexpected message: expected Start or Bye"),
            },
            Err(s) => panic!("{}", s),
        }
    }

    fn handle_player_turn(&mut self) {
        let msg = self.perform_player_move();
        self.state = State::OpponentTurn;
        self.client.send_message(&msg).unwrap();
        self.board.print();
        match self.client.poll_message() {
            Ok(msg) => match msg {
                ServerMessage::Ack { remaining_time_ms } => self.time = remaining_time_ms,
                ServerMessage::End {
                    result,
                    player_count,
                    op_count,
                    reason,
                } => {
                    self.on_end_message(result, player_count, op_count, &reason);
                }
                _ => panic!("Unexpected message: expected Ack"),
            },
            Err(s) => panic!("{}", s),
        };
    }

    fn handle_opponent_turn(&mut self) {
        match self.client.poll_message() {
            Ok(msg) => match msg {
                ServerMessage::Move { pos } => {
                    self.on_move_message(pos);
                }
                ServerMessage::End {
                    result,
                    player_count,
                    op_count,
                    reason,
                } => {
                    self.on_end_message(result, player_count, op_count, &reason);
                }
                _ => panic!("Unexpected message: expected Move or End"),
            },
            Err(s) => panic!("{}", s),
        }
    }

    fn reset(&mut self) {
        self.board = Board::initial();
        self.strategy = Box::new(Strategy::default());
    }

    fn on_start_message(&mut self, color: Color, op_name: &str, time: u32) {
        self.player.color = color;
        self.opponent.color = color.opposite();
        self.opponent.name = op_name.to_string();
        self.time = time;
        self.state = match self.player.color {
            Color::Dark => State::PlayerTurn,
            Color::Light => State::OpponentTurn,
        };
    }

    fn on_move_message(&mut self, pos: Option<Square>) {
        match pos {
            Some(p) => {
                self.board = self.board.flip(&p, self.opponent.color);
            }
            None => (),
        };
        self.board.print();
        self.state = State::PlayerTurn;
    }

    fn on_end_message(&mut self, result: GameResult, player_count: u8, op_count: u8, reason: &str) {
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

    fn perform_player_move(&mut self) -> String {
        self.set_strategy();
        match self.strategy.next_move(&self.board, self.player.color) {
            Some(square) => {
                self.board = self.board.flip(&square, self.player.color);
                move_message(&square)
            }
            None => pass_message(),
        }
    }

    fn set_strategy(&mut self) {
        let count = self.board.empty_squares_count();
        self.strategy = if count < Game::ENDGAME_BORDER {
            Box::new(strategy::Exhausive {})
        } else {
            Box::new(strategy::Naive {})
        };
    }
}
