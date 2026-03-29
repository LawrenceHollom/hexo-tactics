use std::collections::HashSet;

use crate::player::*;
use crate::moves::*;

const SIZE: usize = 101;

const WIDTH_SCALE: i32 = 10;
const HEIGHT_SCALE: i32 = 9;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    board: [[Option<Player>; SIZE]; SIZE],
    pub to_move: Player,
    pub moves_remaining: usize,
    mid: i32,
    pub is_won: bool,
}

/**
 * This stores values for screen-printing
 */
pub struct Hexagon {
    pub centre_x: i32,
    pub centre_y: i32,
    pub player: Option<Player>,
}

impl Board {
    pub fn new() -> Board {
        Board { 
            board: [[None; SIZE]; SIZE], 
            to_move: Player::Yellow,
            moves_remaining: 1,
            mid: (SIZE / 2) as i32,
            is_won: false,
        }
    }

    pub fn make_move(&mut self, m: &Move) {
        let u = (m.get_u() + self.mid) as usize;
        let v = (m.get_v() + self.mid) as usize;
        
        self.board[v][u] = Some(m.get_player());

        if self.to_move != m.get_player() {
            panic!("Player {:?} made a move when it was {:?}'s turn", m.get_player(), self.to_move);
        }
        self.moves_remaining -= 1;
        if self.moves_remaining == 0 {
            if self.has_player_won(self.to_move) {
                self.is_won = true;
            }
            self.to_move = self.to_move.other();
            self.moves_remaining = 2;
        }
    }

    pub fn get_hexagons(&self) -> Vec<Hexagon> {
        let mut out = vec![];

        for u in 0..SIZE {
            for v in 0..SIZE {
                if let Some(player) = self.board[v][u] {
                    let u = u as i32 - self.mid as i32;
                    let v = v as i32 - self.mid as i32;
                    out.push(Hexagon::new(u, v, Some(player)))
                }
            }
        }

        out
    }

    /**
     * Scores how good for winning the given 6-line is:
     * +1 point for each friendly piece.
     * 0 if there is any enemy piece
     */
    fn score_line(&self, u: usize, v: usize, du: i32, dv: i32, player: Player) -> u8 {
        let mut score = 0;
        for i in 0..6 {
            let u = (u as i32 + du * i) as usize;
            let v = (v as i32 + dv * i) as usize;
            if u >= SIZE || v >= SIZE {
                return 0
            }
            match self.board[v][u] {
                Some(p) if p == player => score += 1,
                Some(_other_player) => return 0,
                None => (),
            }
        }
        score
    }

    fn get_max_score_line(&self, u: usize, v: usize, player: Player) -> u8 {
        let mut max_score = 0;
        for (du, dv) in [(1, 0), (0, 1), (-1, 1)] {
            let score = self.score_line(u, v, du, dv, player);
            if score > max_score {
                max_score = score;
            }
        }
        max_score
    }

    fn get_max_score_line_omnidirectional(&self, u: usize, v: usize, player: Player) -> u8 {
        let mut max_score = 0;
        for (du, dv) in [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)] {
            let score = self.score_line(u, v, du, dv, player);
            if score > max_score {
                max_score = score;
            }
        }
        max_score
    }

    pub fn can_current_player_win(&self) -> bool {
        for (v, row) in self.board.iter().enumerate() {
            for (u, piece) in row.iter().enumerate() {
                if piece.map_or(false, |x| x.eq(&self.to_move)) {
                    if self.get_max_score_line(u, v, self.to_move) >= 4 {
                        return true
                    }
                }
            }
        }
        false
    }

    pub fn has_player_won(&self, player: Player) -> bool {
        for (v, row) in self.board.iter().enumerate() {
            for (u, piece) in row.iter().enumerate() {
                if piece.map_or(false, |x| x.eq(&player)) {
                    if self.get_max_score_line(u, v, player) >= 6 {
                        return true
                    }
                }
            }
        }
        false
    }

    /**
     * This could be heavily optimised to just track the threats, and the
     * cells with which the threats could be blocked, without needing to
     * make board copies etc.
     */
    pub fn can_current_player_block_all_threats(&self) -> bool {
        let mut blocking_moves: HashSet<Move> = HashSet::new();

        for v in 0..SIZE {
            for u in 0..SIZE {
                for (du, dv) in [(1, 0), (0, 1), (-1, 1)] {
                    if self.score_line(u, v, du, dv, self.to_move.other()) >= 4 {
                        for i in 0..6 {
                            let su = (u as i32 + i * du) as usize;
                            let sv = (v as i32 + i * dv) as usize;
                            if self.board[sv][su].is_none() {
                                let m = Move::new_unnormalised(self.to_move, su, sv, self.mid);
                                blocking_moves.insert(m);
                            }
                        }
                    }
                }
            }
        }

        let blocking_moves = blocking_moves.iter().collect::<Vec<_>>();

        // println!("Blocking moves:");
        // for m in blocking_moves.iter() {
        //     print!(" [{}, {}]", m.get_u(), m.get_v());
        // }
        // println!();

        if blocking_moves.len() <= 2 {
            // If there are at most two blocking moves, then we play them.
            return true
        }

        
        // If *any* pair of moves blocks, then the player can block.
        for (i, m1) in blocking_moves.iter().enumerate() {
            for m2 in blocking_moves.iter().skip(i + 1) {
                // Play these two moves and then see if opponent can win
                let mut board_copy = self.to_owned();
                board_copy.make_move(m1);
                board_copy.make_move(m2);
                if !board_copy.can_current_player_win() {
                    return true
                }
            }
        }
        false
    }

    /**
     * We can't just interate over everything as that would be too slow.
     * We check for preemptives, and start cashing them in.
     */
    pub fn can_current_player_force_two_step_win(&self) -> bool {
        let mut preemptive_gaps = vec![];
        for (v, row) in self.board.iter().enumerate() {
            for (u, piece) in row.iter().enumerate() {
                if piece.is_none() {
                    if self.get_max_score_line_omnidirectional(u, v, self.to_move) >= 2 {
                        let m = Move::new_unnormalised(self.to_move, u, v, self.mid);
                        // println!("Found preemptive gap at {}, {}", m.get_u(), m.get_v());
                        preemptive_gaps.push(m);
                    }
                }
            }
        }
        // crate::imageio::print_board(self, crate::tactic::Tactic::TwoMoves, "debug1");
        for (i, m1) in preemptive_gaps.iter().enumerate() {
            for m2 in preemptive_gaps.iter().skip(i + 1) {
                // Play these two moves and then see if opponent can block all threats
                let mut board_copy = self.to_owned();
                board_copy.make_move(m1);
                board_copy.make_move(m2);
                if !board_copy.can_current_player_block_all_threats() {
                    // println!("Found winning pair: ({}, {}), ({}, {})", m1.get_u(), m1.get_v(), m2.get_u(), m2.get_v());
                    // crate::imageio::print_board(&board_copy, crate::tactic::Tactic::TwoMoves, "debug2");
                    return true
                }
            }
        }
        false
    }
}

impl Hexagon {
    pub fn new(u: i32, v: i32, player: Option<Player>) -> Hexagon {
        let centre_x = u * WIDTH_SCALE + v * (WIDTH_SCALE / 2);
        let centre_y = v * HEIGHT_SCALE;
        Hexagon { centre_x, centre_y, player }
    }
}