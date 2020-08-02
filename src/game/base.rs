use std::cmp;

use crate::cli::Client;
use crate::game::board::Board;
use crate::game::square::Square;
use crate::game::strategy::exhausive::WINNABLE_COLOR_HISTORY;
use crate::game::strategy::{Exhausive, Naive, NegaScout, Strategy};
use crate::message::{move_message, open_message, pass_message, ServerMessage};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
    time: i32,
    strategy: Box<dyn Strategy>,
    win_game_count: u16,
    lose_game_count: u16,
    tie_game_count: u16,
}

impl Game {
    const ENDGAME_BORDER: u8 = 24;

    fn initialize(client: Client, player: Player, opponent: Player, time: i32) -> Self {
        Self {
            client: client,
            state: State::Wait,
            player: player,
            opponent: opponent,
            board: Board::initial(),
            time: time,
            strategy: Box::new(Naive()),
            win_game_count: 0,
            lose_game_count: 0,
            tie_game_count: 0,
        }
    }

    pub fn launch(host: &str, port: u32, name: &str) -> Result<Self, String> {
        let mut client = Client::new(host, port);
        if client.send_message(&open_message(name)).is_err() {
            return Err("Couldn't start game.".to_string());
        }

        // initialize players with dummy information.
        let player = Player {
            name: name.to_string(),
            color: Color::Dark,
        };
        let opponent = Player {
            name: String::new(),
            color: Color::Light,
        };
        Ok(Game::initialize(client, player, opponent, 0))
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
        match self.client.poll_message().unwrap() {
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
        }
    }

    fn handle_player_turn(&mut self) {
        let msg = self.perform_player_move();
        self.state = State::OpponentTurn;
        self.client.send_message(&msg).unwrap();
        //self.board.print();
        match self.client.poll_message().unwrap() {
            ServerMessage::Ack { remaining_time_ms } => self.time = remaining_time_ms,
            ServerMessage::End {
                result,
                player_count,
                op_count,
                reason,
            } => self.on_end_message(result, player_count, op_count, &reason),
            _ => panic!("Unexpected message: expected Ack"),
        };
    }

    fn handle_opponent_turn(&mut self) {
        match self.client.poll_message().unwrap() {
            ServerMessage::Move { pos } => self.on_move_message(pos),
            ServerMessage::End {
                result,
                player_count,
                op_count,
                reason,
            } => self.on_end_message(result, player_count, op_count, &reason),
            _ => panic!("Unexpected message: expected Move or End"),
        };
    }

    fn reset(&mut self) {
        self.board = Board::initial();
        self.strategy = Box::new(Naive());
        // due to memory issue
        if self.total_game_count() % 5 == 0 {
            let mut write = WINNABLE_COLOR_HISTORY.write().unwrap();
            write.clear();
        }
    }

    fn on_start_message(&mut self, color: Color, op_name: &str, time: i32) {
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
        if let Some(square) = pos {
            self.board = self.board.flip(square.to_uint(), self.opponent.color);
        }
        //self.board.print();
        self.state = State::PlayerTurn;
    }

    fn on_end_message(&mut self, result: GameResult, player_count: u8, op_count: u8, reason: &str) {
        let result_str = match result {
            GameResult::Win => {
                self.win_game_count += 1;
                "You win!"
            }
            GameResult::Lose => {
                self.lose_game_count += 1;
                "You lose!"
            }
            GameResult::Tie => {
                self.tie_game_count += 1;
                "Tie game!"
            }
        };
        println!(
            "{}. {}'s count: {}, {}'s count: {}, reason: {}",
            result_str, self.player.name, player_count, self.opponent.name, op_count, reason
        );
        println!(
            "Results so far:\n\twin:  {}\n\tlose: {}\n\ttie:  {}",
            self.win_game_count, self.lose_game_count, self.tie_game_count
        );
        self.state = State::Wait;
    }

    fn perform_player_move(&mut self) -> String {
        self.set_strategy();
        match self.strategy.next_move(self.board, self.player.color) {
            Some(square) => {
                self.board = self.board.flip(square.to_uint(), self.player.color);
                move_message(square)
            }
            None => pass_message(),
        }
    }

    fn set_strategy(&mut self) {
        let count = self.board.empty_squares_count();
        self.strategy = if count < Game::ENDGAME_BORDER {
            Box::new(Exhausive::new(self.time as u64 / 3))
        } else {
            // need some time to execute exhausive search at the end.
            Box::new(NegaScout::new(
                cmp::max((self.time - 30000) / 2, 0) as u64,
                NegaScout::emergency_move(self.board, self.player.color),
            ))
        };
    }

    fn total_game_count(&self) -> u16 {
        self.win_game_count + self.lose_game_count + self.tie_game_count
    }
}
