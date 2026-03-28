use crate::player::*;
use crate::game::*;

const SIZE: usize = 101;

const WIDTH_SCALE: i32 = 10;
const HEIGHT_SCALE: i32 = 9;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    board: [[Option<Player>; SIZE]; SIZE],
    mid: i32,
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
            mid: (SIZE / 2) as i32,
        }
    }

    pub fn make_move(&mut self, m: &Move) {
        let u = (m.get_u() + self.mid) as usize;
        let v = (m.get_v() + self.mid) as usize;
        
        self.board[v][u] = Some(m.get_player());
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
}

impl Hexagon {
    pub fn new(u: i32, v: i32, player: Option<Player>) -> Hexagon {
        let centre_x = u * WIDTH_SCALE + v * (WIDTH_SCALE / 2);
        let centre_y = v * HEIGHT_SCALE;
        Hexagon { centre_x, centre_y, player }
    }
}