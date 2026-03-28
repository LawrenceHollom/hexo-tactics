mod game;
mod board;
mod fileio;
mod utils;
mod player;
mod position;
mod imageio;
mod pixel;

use std::time;
use std::io;

use utils::*;

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
    _ => println!("Unknown function {}", func)
  }
  let dur = start.elapsed();
  println!("Duration = {}", pretty_format_time(dur));
  text
}

fn test() {
    let json = fileio::read_json("one_game");
    let game = game::Game::from_json(json);
    game.print();
}