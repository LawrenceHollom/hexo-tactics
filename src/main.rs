mod game;
mod board;
mod fileio;
mod utils;
mod player;
mod position;
mod imageio;
mod pixel;
mod tactic;
mod moves;

use std::time;
use std::io;

use utils::*;

use crate::tactic::Tactic;

fn main() {
    loop {
        get_input();
    }  
}

fn get_input() -> String {
    println!("Enter instruction:");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read line");
    let start = time::Instant::now();
    let (func, args) = parse_function_like(text.as_str());
    match func.to_lowercase().trim() {
        "test" => test(),
        "1" | "one_move" => one_move(),
        "2" | "two_moves" => two_moves(),
        _ => println!("Unknown function {}", func)
    }
    let dur = start.elapsed();
    println!("Duration = {}", pretty_format_time(dur));
    text
}

fn test() {
    let json = fileio::read_json("one_game");
    let game = game::Game::from_json(json);
    let boards = game.get_two_step_wins();
    println!("Found {} winning boards", boards.len());
    if let Some(board) = boards.first() {
        imageio::print_board(board, Tactic::TwoMoves, "test");
    }
    game.print();
}

/**
 * Find and print the instances where one player has an instant win.
 */
fn one_move() {
    let json = fileio::read_json("games");
    let mut i = 0;
    for one_json in json.as_array().unwrap().iter() {
        let game = game::Game::from_json(one_json.to_owned());
        let boards = game.get_forced_wins();
        for board in boards {
            imageio::print_board(&board, Tactic::OneMove, &format!("{:03}", i));
            i += 1;
        }
    }
}

fn two_moves() {
    let start = time::Instant::now();
    let json = fileio::read_json("games");
    println!("Finished reading file. Time taken = {}", pretty_format_time(start.elapsed()));
    let mut i = 0;
    for one_json in json.as_array().unwrap().iter() {
        let game = game::Game::from_json(one_json.to_owned());
        let boards = game.get_two_step_wins();
        for board in boards {
            imageio::print_board(&board, Tactic::TwoMoves, &format!("{:04}", i));
            i += 1;
        }
    }
}