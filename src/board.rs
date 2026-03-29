use crate::direction::Direction;
use crate::player::*;
use crate::moves::*;
use crate::position::Position;
use crate::threats::*;

const SIZE: usize = 101;

const WIDTH_SCALE: i32 = 10;
const HEIGHT_SCALE: i32 = 9;

#[derive(Debug, Clone)]
pub struct Board {
    board: [[Option<Player>; SIZE]; SIZE],
    pub to_move: Player,
    pub moves_remaining: usize,
    mid: i32,
    yellow_threats: ThreatSet,
    blue_threats: ThreatSet,
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
            yellow_threats: ThreatSet::new(),
            blue_threats: ThreatSet::new(),
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

        // Update threats.
        self.get_threat_set_mut(self.to_move).on_friendly_move(m.get_position());
        self.get_threat_set_mut(self.to_move.other()).on_enemy_move(m.get_position());

        for dir in Direction::ALL {
            for offset in -5..=0 {
                let start = m.get_position().offset(dir, offset);
                let su = start.get_normalised_u(self.mid);
                let sv = start.get_normalised_v(self.mid);
                let score = self.score_line(su, sv, dir, self.to_move);
                if score == 1 {
                    // This threat wouldn't have been present before.
                    self.get_threat_set_mut(self.to_move).add_singleton(start, dir, m.get_position());
                }
            }
        }

        if self.moves_remaining == 0 {
            if self.has_player_won(self.to_move) {
                self.is_won = true;
            }
            self.to_move = self.to_move.other();
            self.moves_remaining = 2;
        }
    }

    fn get_threat_set_mut(&mut self, player: Player) -> &mut ThreatSet {
        match player {
            Player::Yellow => &mut self.yellow_threats,
            Player::Blue => &mut self.blue_threats,
        }
    }

    fn get_threat_set(&self, player: Player) -> &ThreatSet {
        match player {
            Player::Yellow => &self.yellow_threats,
            Player::Blue => &self.blue_threats,
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
    fn score_line(&self, u: usize, v: usize, dir: Direction, player: Player) -> u8 {
        let (du, dv) = dir.to_vector();
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
        for dir in Direction::POSITIVE {
            let score = self.score_line(u, v, dir, player);
            if score > max_score {
                max_score = score;
            }
        }
        max_score
    }

    // fn get_max_score_line_omnidirectional(&self, u: usize, v: usize, player: Player) -> u8 {
    //     let mut max_score = 0;
    //     for dir in Direction::ALL {
    //         let score = self.score_line(u, v, dir, player);
    //         if score > max_score {
    //             max_score = score;
    //         }
    //     }
    //     max_score
    // }

    pub fn can_current_player_win(&self) -> bool {
        self.get_threat_set(self.to_move).get_best_threat_size() <= 2

        // for (v, row) in self.board.iter().enumerate() {
        //     for (u, piece) in row.iter().enumerate() {
        //         if piece.map_or(false, |x| x.eq(&self.to_move)) {
        //             if self.get_max_score_line(u, v, self.to_move) >= 4 {
        //                 return true
        //             }
        //         }
        //     }
        // }
        // false
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
    // pub fn can_current_player_block_all_threats(&self) -> bool {
    //     let mut blocking_moves: HashSet<Move> = HashSet::new();

    //     for v in 0..SIZE {
    //         for u in 0..SIZE {
    //             for dir in Direction::POSITIVE {
    //                 if self.score_line(u, v, dir, self.to_move.other()) >= 4 {
    //                     for i in 0..6 {
    //                         let (du, dv) = dir.to_vector();
    //                         let su = (u as i32 + i * du) as usize;
    //                         let sv = (v as i32 + i * dv) as usize;
    //                         if self.board[sv][su].is_none() {
    //                             let m = Move::new_unnormalised(self.to_move, su, sv, self.mid);
    //                             blocking_moves.insert(m);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     let blocking_moves = blocking_moves.iter().collect::<Vec<_>>();

    //     // println!("Blocking moves:");
    //     // for m in blocking_moves.iter() {
    //     //     print!(" [{}, {}]", m.get_u(), m.get_v());
    //     // }
    //     // println!();

    //     if blocking_moves.len() <= 2 {
    //         // If there are at most two blocking moves, then we play them.
    //         return true
    //     }

        
    //     // If *any* pair of moves blocks, then the player can block.
    //     for (i, m1) in blocking_moves.iter().enumerate() {
    //         for m2 in blocking_moves.iter().skip(i + 1) {
    //             // Play these two moves and then see if opponent can win
    //             let mut board_copy = self.to_owned();
    //             board_copy.make_move(m1);
    //             board_copy.make_move(m2);
    //             if !board_copy.can_current_player_win() {
    //                 return true
    //             }
    //         }
    //     }
    //     false
    // }

    fn can_current_player_block_all_threats(&self, print_debug: bool) -> bool {
        let threats = self.get_threat_set(self.to_move.other()).get_all_immediate_threats();
        if print_debug {
            println!("Immediate threats:");
            threats.print();
        }
        if threats.has_at_least_three_singletons() {
            false
        } else if let Some((p1, p2)) = threats.get_first_doubleton() {
            if print_debug && p1.u == 4 && p1.v == -5 {
                threats.print();
            }
            // Try playing p1.
            let remaining_threats = threats.after_playing(p1);
            if remaining_threats.is_star() {
                return true
            }
            // Try playing p2
            let remaining_threats = threats.after_playing(p2);
            if remaining_threats.is_star() {
                return true
            }
            // If neither worked, then the player can't block.
            false
        } else {
            true
        }
    }

    /**
     * If the current player requires two moves to block all threats, then return those moves.
     * Otherwise, return None.
     */
    fn get_current_player_two_blocks(&self) -> Vec<(Position, Position)> {
        let mut out = vec![];
        let threats = self.get_threat_set(self.to_move.other()).get_all_immediate_threats();
        if threats.has_at_least_three_singletons() {
            return vec![];
        } else if let Some((p1, p2)) = threats.get_the_exact_two_singletons() {
            if threats.after_playing(p1).after_playing(p2).is_empty() {
                return vec![(p1, p2)]
            } else {
                return vec![];
            }
        } else if let Some(p1) = threats.get_the_exact_one_singleton() {
            let new_threats = threats.after_playing(p1);
            if new_threats.is_empty() {
                return vec![]
            }
            for p2 in new_threats.get_all_blocking_points() {
                out.push((p1, p2));
            }
        } else if let Some((p1, p2)) = threats.get_first_doubleton() {
            // There are no singletons, but there are doubletons.
            let new_threats = threats.after_playing(p1);
            if !new_threats.is_empty() {
                for q in new_threats.get_all_blocking_points() {
                    out.push((p1, q));
                }
            }
            let new_threats = threats.after_playing(p2);
            if !new_threats.is_empty() {
                for q in new_threats.get_all_blocking_points() {
                    if q != p1 {
                        out.push((p2, q));
                    }
                }
            }
        } else {
            // There's nothing to block.
            return vec![];
        }
        out
    }

    /**
     * We can't just interate over everything as that would be too slow.
     * We check for preemptives, and start cashing them in.
     */
    // pub fn can_current_player_force_two_step_win(&self) -> bool {
    //     let mut preemptive_gaps = vec![];
    //     for (v, row) in self.board.iter().enumerate() {
    //         for (u, piece) in row.iter().enumerate() {
    //             if piece.is_none() {
    //                 if self.get_max_score_line_omnidirectional(u, v, self.to_move) >= 2 {
    //                     let m = Move::new_unnormalised(self.to_move, u, v, self.mid);
    //                     // println!("Found preemptive gap at {}, {}", m.get_u(), m.get_v());
    //                     preemptive_gaps.push(m);
    //                 }
    //             }
    //         }
    //     }
    //     // crate::imageio::print_board(self, crate::tactic::Tactic::TwoMoves, "debug1");
    //     for (i, m1) in preemptive_gaps.iter().enumerate() {
    //         for m2 in preemptive_gaps.iter().skip(i + 1) {
    //             // Play these two moves and then see if opponent can block all threats
    //             let mut board_copy = self.to_owned();
    //             board_copy.make_move(m1);
    //             board_copy.make_move(m2);
    //             if !board_copy.can_current_player_block_all_threats() {
    //                 // println!("Found winning pair: ({}, {}), ({}, {})", m1.get_u(), m1.get_v(), m2.get_u(), m2.get_v());
    //                 // crate::imageio::print_board(&board_copy, crate::tactic::Tactic::TwoMoves, "debug2");
    //                 return true
    //             }
    //         }
    //     }
    //     false
    // }

    pub fn can_current_player_force_two_step_win(&self, print_debug: bool) -> bool {
        let potential_moves = self.get_threat_set(self.to_move).get_all_preemptive_moves();
        if print_debug {
            println!("Potential moves:");
            for m in potential_moves.iter() {
                println!("  ({}, {})", m.u, m.v);
            }
        }
        for (i, p1) in potential_moves.iter().enumerate() {
            for p2 in potential_moves.iter().skip(i + 1) {
                // Play these two moves and then see if opponent can block all threats
                let mut board_copy = self.to_owned();
                let m1 = Move::of_position(self.to_move, *p1);
                let m2 = Move::of_position(self.to_move, *p2);
                board_copy.make_move(&m1);
                board_copy.make_move(&m2);

                if board_copy.can_current_player_win() {
                    // So much for those threats...
                    continue
                }

                let print_debug = print_debug && ((p1.u == 4 && p1.v == -5) || (p1.u == 6 && p1.v == -7));
                if print_debug {
                    println!("Testing moves ({}, {}) and ({}, {})", m1.get_u(), m1.get_v(), m2.get_u(), m2.get_v());
                }
                if !board_copy.can_current_player_block_all_threats(print_debug) {
                    return true
                }
            }
        }
        false
    }

    pub fn can_current_player_force_three_step_win(&self) -> bool {
        let potential_moves = self.get_threat_set(self.to_move).get_all_preemptive_moves();
        if self.can_current_player_win() || self.can_current_player_force_two_step_win(false) {
            println!("Skipped as there's a 1 or 2-step win!");
            return false
        }
        // let mut num_deeper = 0;
        // println!("to_move = {:?}, moves_remaining = {}", self.to_move, self.moves_remaining);
        for (i, p1) in potential_moves.iter().enumerate() {
            for p2 in potential_moves.iter().skip(i + 1) {
                // Play these two moves and then see if opponent can block all threats
                let mut board_copy = self.to_owned();
                let m1 = Move::of_position(self.to_move, *p1);
                let m2 = Move::of_position(self.to_move, *p2);
                board_copy.make_move(&m1);
                board_copy.make_move(&m2);

                if board_copy.can_current_player_win() {
                    // So much for those threats...
                    continue
                }

                let blocks = board_copy.get_current_player_two_blocks();
                // Iterate over all moves the other player might make in response.
                let mut win_always_forced = blocks.len() >= 1;
                'iter_other: for (p3, p4) in blocks {
                    let mut board_copy_copy = board_copy.to_owned();
                    let m3 = Move::of_position(self.to_move.other(), p3);
                    let m4 = Move::of_position(self.to_move.other(), p4);
                    board_copy_copy.make_move(&m3);
                    board_copy_copy.make_move(&m4);
                    // num_deeper += 1;
                    if !board_copy_copy.can_current_player_force_two_step_win(false) {
                        win_always_forced = false;
                        break 'iter_other
                    }
                }

                if win_always_forced {
                    println!("Moves are ({}, {}) and ({}, {})", m1.get_u(), m1.get_v(), m2.get_u(), m2.get_v());
                    crate::imageio::print_board(&board_copy, crate::tactic::Tactic::Test, "debug");
                    // println!("True! Num deeper = {}", num_deeper);
                    return true
                }
            }
        }
        // println!("False! Num deeper = {}", num_deeper);
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