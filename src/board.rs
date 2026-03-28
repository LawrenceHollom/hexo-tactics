use crate::player::*;
use crate::game::*;

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

    pub fn can_current_player_win(&self) -> bool {
        for (v, row) in self.board.iter().enumerate() {
            for (u, piece) in row.iter().enumerate() {
                if piece.map_or(false, |x| x.eq(&self.to_move)) {
                    if self.score_line(u, v, 1, 0, self.to_move) >= 4 {
                        return true
                    }
                    if self.score_line(u, v, 0, 1, self.to_move) >= 4 {
                        return true
                    }
                    if self.score_line(u, v, -1, 1, self.to_move) >= 4 {
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
                if piece.map_or(false, |x| x.eq(&self.to_move)) {
                    if self.score_line(u, v, 1, 0, player) >= 6 {
                        return true
                    }
                    if self.score_line(u, v, 0, 1, player) >= 6 {
                        return true
                    }
                    if self.score_line(u, v, -1, 1, player) >= 6 {
                        return true
                    } 
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