use std::collections::HashSet;

use crate::position::Position;
use crate::direction::Direction;

#[derive(Debug, Clone)]
pub struct ThreatSet {
    threats: Vec<Threat>,
    pub contains_win: bool,
}

#[derive(Debug, Clone)]
pub struct Threat {
    size: usize,
    holes: Vec<Position>,
}

pub struct ImmediateThreatSet {
    singletons: Vec<Position>,
    doubletons: Vec<(Position, Position)>,
}

impl ThreatSet {
    pub fn new() -> ThreatSet {
        ThreatSet { 
            threats: vec![],
            contains_win: false,
         }
    }

    /**
     * A friendly move will make threats bigger.
     */
    pub fn on_friendly_move(&mut self, pos: Position) {
        for threat in self.threats.iter_mut() {
            threat.remove_point(pos);
            if threat.size == 0 {
                self.contains_win = true;
            }
        }
    }

    pub fn add_singleton(&mut self, start: Position, dir: Direction, new_piece: Position) {
        let mut holes = vec![];
        for i in 0..6 {
            let hole = start.offset(dir, i);
            if hole != new_piece {
                holes.push(hole);
            }
        }
        self.threats.push(Threat::new(holes));
    }

    /**
     * An enemy move might remove some threats.
     */
    pub fn on_enemy_move(&mut self, pos: Position) {
        let mut i = 0;
        while i < self.threats.len() {
            if self.threats[i].contains_point(pos) {
                self.threats.remove(i);
            } else {
                i += 1;
            }
        }
    }

    /**
     * Good threats are small as they are easy to complete.
     */
    pub fn get_best_threat_size(&self) -> usize {
        if self.threats.is_empty() {
            return usize::MAX;
        }
        self.threats.iter().map(|threat| threat.size).min().unwrap_or(0)
    }

    /**
     * Get all cells contained in threats of at size at most 4.
     */
    pub fn get_all_preemptive_moves(&self) -> Vec<Position> {
        let mut set = HashSet::new();
        for threat in self.threats.iter() {
            if threat.size <= 4 {
                set.extend(threat.holes.iter());
            }
        }
        set.into_iter().collect()
    }

    pub fn get_all_immediate_threats(&self) -> ImmediateThreatSet {
        let mut singletons = HashSet::new();
        let mut doubletons = HashSet::new();
        for threat in self.threats.iter() {
            if threat.size == 1 {
                singletons.insert(threat.holes[0]);
            } else if threat.size == 2 {
                if threat.holes[0] < threat.holes[1] {
                    doubletons.insert((threat.holes[0], threat.holes[1]));
                } else {
                    doubletons.insert((threat.holes[1], threat.holes[0]));
                }
            }
        }
        let mut out = ImmediateThreatSet::new();
        out.singletons = singletons.into_iter().collect();
        out.doubletons = doubletons.into_iter().collect();
        out
    }

    pub fn print(&self) {
        for size in 0..5 {
            println!("Threats of size {}:", size);
            for threat in self.threats.iter() {
                if threat.size == size {
                    for hole in threat.holes.iter() {
                        print!("({}, {}) ", hole.u, hole.v);
                    }
                    println!();
                }
            }
        }
    }
}

impl Threat {
    pub fn new(holes: Vec<Position>) -> Threat {
        Threat { 
            size: holes.len(), 
            holes 
        }
    }

    pub fn remove_point(&mut self, point: Position) {
        let mut index = None;
        for (i, hole) in self.holes.iter().enumerate() {
            if *hole == point {
                index = Some(i);
                break;
            }
        }
        if let Some(index) = index {
            self.holes.swap_remove(index);
            self.size -= 1;
        }
    }

    pub fn contains_point(&self, point: Position) -> bool {
        self.holes.iter().any(|hole| *hole == point)
    }
}

impl ImmediateThreatSet {
    pub fn new() -> ImmediateThreatSet {
        ImmediateThreatSet { 
            singletons: vec![], 
            doubletons: vec![],
         }
    }

    pub fn print(&self) {
        println!("  Singletons:");
        for singleton in self.singletons.iter() {
            println!("    ({}, {})", singleton.u, singleton.v);
        }
        println!("  Doubletons:");
        for (p1, p2) in self.doubletons.iter() {
            println!("    ({}, {}) and ({}, {})", p1.u, p1.v, p2.u, p2.v);
        }
    }

    pub fn has_at_least_three_singletons(&self) -> bool {
        self.singletons.len() >= 3
    }

    /**
     * If there are exactly two singletons, return them.
     * Otherwise, return None.
     */
    pub fn get_the_exact_two_singletons(&self) -> Option<(Position, Position)> {
        if self.singletons.len() == 2 {
            Some((self.singletons[0], self.singletons[1]))
        } else {
            None
        }
    }

    /**
     * If there is exactly one singleton, return it.
     * Otherwise, return None.
     */
    pub fn get_the_exact_one_singleton(&self) -> Option<Position> {
        if self.singletons.len() == 1 {
            Some(self.singletons[0])
        } else {
            None
        }
    }

    /**
     * Return a vec of those points which, by themselves, can block everything.
     * There will be at most two of them.
     */
    pub fn get_all_blocking_points(&self) -> Vec<Position> {
        if self.singletons.len() >= 2 {
            vec![]
        } else if self.singletons.len() == 1 {
            if self.after_playing(self.singletons[0]).is_empty() {
                vec![self.singletons[0]]
            } else {
                vec![]
            }
        } else if self.doubletons.is_empty() {
            vec![]
        } else {
            let mut out = vec![];
            let (p1, p2) = self.doubletons[0];
            if self.after_playing(p1).is_empty() {
                out.push(p1)
            }
            if self.after_playing(p2).is_empty() {
                out.push(p2)
            }
            out
        }
    }

    pub fn get_first_doubleton(&self) -> Option<(Position, Position)> {
        self.doubletons.first().cloned()
    }

    /**
     * Remove every element which contains the given position.
     */
    pub fn after_playing(&self, pos: Position) -> ImmediateThreatSet {
        let mut out = ImmediateThreatSet::new();
        for singleton in self.singletons.iter() {
            if *singleton != pos {
                out.singletons.push(*singleton);
            }
        }
        for doubleton in self.doubletons.iter() {
            let (p1, p2) = *doubleton;
            if p1 != pos && p2 != pos {
                out.doubletons.push((p1, p2));
            }
        }
        out
    }

    pub fn is_empty(&self) -> bool {
        self.singletons.is_empty() && self.doubletons.is_empty()
    }

    /**
     * Does every element contain a given position?
     */
    pub fn is_star(&self) -> bool {
        if self.singletons.len() >= 2 {
            false
        } else if self.singletons.len() == 1 {
            let singleton = self.singletons[0];
            // All the pairs must contain this point.
            self.doubletons.iter().all(|(p1, p2)| *p1 == singleton || *p2 == singleton)
        } else if self.doubletons.len() <= 1 {
            // There's only one element, so this is a star.
            true
        } else {
            // We have at least two doubletons, so we need to check if they all share a common point.
            let (p1, p2) = self.doubletons[0];
            let candidate_points = [p1, p2];
            for candidate in candidate_points.iter() {
                if self.doubletons.iter().all(|(x1, x2)| *x1 == *candidate || *x2 == *candidate) {
                    return true;
                }
            }
            false
        }
    }
}