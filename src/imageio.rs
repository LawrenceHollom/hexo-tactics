use crate::pixel::*;
use crate::board::*;
use crate::player::Player;
use crate::tactic;
use image::{ImageBuffer, Rgba};

const HEX_MASK: [[u8; 11]; 12] = [
    [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 2, 2, 2, 1, 1, 0, 0],
    [1, 1, 2, 2, 2, 2, 2, 2, 2, 1, 1],
    [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
    [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
    [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
    [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
    [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
    [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
    [1, 1, 2, 2, 2, 2, 2, 2, 2, 1, 1],
    [0, 0, 1, 1, 2, 2, 2, 1, 1, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0]
];

const BORDER_SIZE: u32 = 3;
const EXTRA_SIZE: usize = 40;

fn draw_hex(buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, hex: &Hexagon, x_offset: i32, y_offset: i32) {
    let centre_x = hex.centre_x + x_offset;
    let centre_y = hex.centre_y + y_offset;
    let player_color = Rgba(match hex.player {
        Some(Player::Yellow) => Pixel::YELLOW,
        Some(Player::Blue) => Pixel::CYAN,
        None => Pixel::BACKGROUND,
    }.to_ints());
    let (width, height ) = buffer.dimensions();
    for (dy, row) in HEX_MASK.iter().enumerate() {
        for (dx, &mask) in row.iter().enumerate() {
            if mask == 0 {
                continue;
            }
            let x = (centre_x + dx as i32 - 5) as u32;
            let y = (centre_y + dy as i32 - 5) as u32;
            let color = if mask == 1 {
                Rgba(Pixel::LINES.to_ints())
            } else {
                player_color
            };
            if x < width && y < height {
                buffer.put_pixel(x, y, color);
            }
        }
    }
}

fn draw_border(buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, to_move: Player) {
    let color = Rgba(match to_move {
        Player::Yellow => Pixel::YELLOW,
        Player::Blue => Pixel::CYAN,
    }.to_ints());
    let (width, height) = buffer.dimensions();
    for x in 0..width {
        for s in 0..BORDER_SIZE {
            buffer.put_pixel(x, s, color);
            buffer.put_pixel(x, height - 1 - s, color);
        }
    }
    for y in 0..height {
        for s in 0..BORDER_SIZE {
            buffer.put_pixel(s, y, color);
            buffer.put_pixel(width - 1 - s, y, color);
        }
    }
}

pub fn print_board(board: &Board, tactic: tactic::Tactic, filename: &str) {
    let hexes = board.get_hexagons();

    let mut min_centre_x = 0;
    let mut max_centre_x = 0;
    let mut min_centre_y = 0;
    let mut max_centre_y = 0;
    for hex in &hexes {
        min_centre_x = min_centre_x.min(hex.centre_x);
        max_centre_x = max_centre_x.max(hex.centre_x);
        min_centre_y = min_centre_y.min(hex.centre_y);
        max_centre_y = max_centre_y.max(hex.centre_y);
    }

    let width = (max_centre_x - min_centre_x) as usize + 22 + EXTRA_SIZE;
    let height = (max_centre_y - min_centre_y) as usize + 22 + EXTRA_SIZE;
    let x_offset = -min_centre_x + 10 + EXTRA_SIZE as i32 / 2;
    let y_offset = -min_centre_y + 10 + EXTRA_SIZE as i32 / 2;

    let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width as u32, height as u32);

    for u in -100..100 {
        for v in -100..100 {
            let hex = Hexagon::new(u, v, None);
            if hex.centre_x + x_offset >= -10 && hex.centre_x + x_offset < width as i32 + 10 &&
                    hex.centre_y + y_offset >= -10 && hex.centre_y + y_offset < height as i32 + 10 {
                draw_hex(&mut buffer, &hex, x_offset, y_offset);
            }
        }
    }

    for hex in hexes {
        draw_hex(&mut buffer, &hex, x_offset, y_offset);
    }

    draw_border(&mut buffer, board.to_move);

    let filename = format!("out/{}/{}.png", tactic.to_string(), filename);
    println!("Saving image to {}", filename);

    buffer.save(filename).expect("Failed to save image");
    
}