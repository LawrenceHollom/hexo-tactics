use crate::player::*;
use crate::position::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    player: Player,
    position: Position,
}

impl Move {
    pub fn new(player: Player, u: i32, v: i32) -> Move {
        Move { 
            player, 
            position: Position::new(u, v),
        }
    }

    pub fn new_unnormalised(player: Player, u: usize, v: usize, mid: i32) -> Move {
        Move { 
            player, 
            position: Position::new(u as i32 - mid, v as i32 - mid),
        }
    }

    pub fn get_u(&self) -> i32 {
        self.position.u
    }

    pub fn get_v(&self) -> i32 {
        self.position.v
    }

    pub fn get_player(&self) -> Player {
        self.player
    }

    pub fn print(&self) {
        println!("{:?} at ({}, {})", self.player, self.position.u, self.position.v);
    }
}