use serde_json::Value;

use crate::player::*;
use crate::position::*;

pub struct Move {
    player: Player,
    position: Position,
}

pub struct Game {
    moves: Vec<Move>,
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
}

impl Game {
    pub fn from_json(json: Value) -> Game {
        println!("{:?}", json.get("moves"));
        let players = json.get("players").unwrap().as_array().unwrap();
        let yellow_player = get_player(players, 0);
        let blue_player = get_player(players, 1);
        let json_moves = json.get("moves").unwrap().as_array().unwrap();
        let mut moves = vec![];
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
            moves.push(Move::new(player, x, y));
        }
        Game { moves }
    }

    pub fn print(&self) {
        for m in &self.moves {
            println!("{:?} at ({}, {})", m.player, m.position.x, m.position.y);
        }
    }
}