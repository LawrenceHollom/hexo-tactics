use serde_json::Value;

use crate::board::Board;
use crate::imageio;
use crate::player::*;
use crate::moves::*;
use crate::tactic::Tactic;

pub struct Game {
    moves: Vec<Move>,
    boards: Vec<Board>,
}

fn get_player(players: &Vec<Value>, index: usize) -> String {
    players[index].get("playerId").unwrap().as_str().unwrap().to_owned()
}

impl Game {
    pub fn from_json(json: Value) -> Game {
        // println!("{:?}", json.get("moves"));
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
            let u = m.get("x").unwrap().as_number().unwrap().as_i64().unwrap() as i32;
            let v = m.get("y").unwrap().as_number().unwrap().as_i64().unwrap() as i32;
            let m = Move::new(player, u, v);
            board.make_move(&m);
            moves.push(m);
            boards.push(board.to_owned());
        }
        Game { moves, boards }
    }

    /**
     * Go through all the boards, and return those where the current
     * player has two moves remaining and, and can win this turn.
     */
    pub fn get_forced_wins(&self) -> Vec<Board> {
        let mut out = vec![];

        for board in self.boards.iter() {
            if board.moves_remaining == 2 && !board.is_won {
                if board.can_current_player_win() {
                    out.push(board.to_owned());
                }
            }
        }
        
        out
    }

    /**
     * Go through all the boards, and return those where the current
     * player has two moves remaining and, and can force a win next turn.
     */
    pub fn get_two_step_wins(&self, print_debug: bool) -> Vec<Board> {
        let mut out = vec![];

        for board in self.boards.iter() {
            if board.moves_remaining == 2 && !board.is_won {
                if board.can_current_player_win() {
                    // We don't care about these.
                    continue;
                }
                
                if board.can_current_player_force_two_step_win(false) {
                    out.push(board.to_owned());
                }
            }
        }
        
        out
    }

    pub fn get_three_step_wins(&self) -> Vec<Board> {
        let mut out = vec![];

        for board in self.boards.iter() {
            if board.moves_remaining == 2 && !board.is_won {
                if board.can_current_player_win() {
                    // We don't care about these.
                    continue;
                }
                if board.can_current_player_force_three_step_win() {
                    out.push(board.to_owned());
                }
            }
        }
        
        out
    }

    pub fn print(&self) {
        for m in &self.moves {
            m.print();
        }
        self.boards.last().map(|b| imageio::print_board(b, Tactic::Test, "test"));
    }
}