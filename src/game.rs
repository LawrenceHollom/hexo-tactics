use serde_json::Value;

use crate::board::Board;
use crate::imageio;
use crate::player::*;
use crate::position::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    player: Player,
    position: Position,
}

pub struct Game {
    moves: Vec<Move>,
    boards: Vec<Board>,
}

fn get_player(players: &Vec<Value>, index: usize) -> String {
    players[index].get("playerId").unwrap().as_str().unwrap().to_owned()
}

impl Move {
    pub fn new(player: Player, x: i32, y: i32) -> Move {
        Move { 
            player, 
            position: Position::new(x, y),
        }
    }

    pub fn get_u(&self) -> i32 {
        self.position.u
    }

    pub fn get_v(&self) -> i32 {
        self.position.v
    }

    pub fn get_x(&self, mid: i32) -> usize {
        (2 * self.position.u + self.position.v + mid) as usize
    }

    pub fn get_y(&self, mid: i32) -> usize {
        (self.position.v + mid) as usize
    }

    pub fn get_player(&self) -> Player {
        self.player
    }
}

impl Game {
    pub fn from_json(json: Value) -> Game {
        println!("{:?}", json.get("moves"));
        let players = json.get("players").unwrap().as_array().unwrap();
        let yellow_player = get_player(players, 0);
        let blue_player = get_player(players, 1);
        let json_moves = json.get("moves").unwrap().as_array().unwrap();
        let mut moves = vec![];
        let mut boards = vec![];
        let mut board = Board::new();
        for m in json_moves {
            let player = m.get("playerId").unwrap().as_str().unwrap();
            let player = if player == blue_player {
                Player::Blue
            } else if player == yellow_player {
                Player::Yellow
            } else {
                panic!("Unknown player {}", player);
            };
            let x = m.get("x").unwrap().as_number().unwrap().as_i64().unwrap() as i32;
            let y = m.get("y").unwrap().as_number().unwrap().as_i64().unwrap() as i32;
            let m = Move::new(player, x, y);
            board.make_move(&m);
            moves.push(m);
            boards.push(board.to_owned());
        }
        Game { moves, boards }
    }

    pub fn print(&self) {
        for m in &self.moves {
            println!("{:?} at ({}, {})", m.player, m.position.u, m.position.v);
        }
        self.boards.last().map(|b| imageio::print_board(b));
    }
}