mod board;
mod pieces;
mod engine;
mod movedata;
mod movegenerator;
mod constants;
use crate::movegenerator::{calculate_bishop_mask, find_magic_number};

fn main() {
    println!("pub const BISHOP_MAGICS: [u64; 64] = [");
    for square in 0usize..64 {
        // 1. Swap to the Bishop mask
        let mask = calculate_bishop_mask(square);
        let bit_count = mask.count_ones();
        
        // 2. Set the is_bishop flag to true
        let magic = find_magic_number(square, bit_count, true);
        
        println!("    0x{:016x}, // Square {}", magic, square);
    }
    println!("];");
}