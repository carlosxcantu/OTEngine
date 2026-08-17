mod board;
mod pieces;
mod engine;
mod movedata;
mod movegenerator;
mod constants;
mod magics;
mod perft;
mod evaluation;
mod search;
use crate::{board::Board, magics::{MagicBitBoards, calculate_bishop_mask}, movegenerator::{MoveList, generate_psuedo_legal_moves}};

const depth: u8 = 5;

fn main() 
{
    let magic_bitboards = MagicBitBoards::new();
    let mut board = Board::new();

    println!("Engine is thinking...");
    
    let best_move = search::search_root(&mut board, depth, &magic_bitboards);
    
    let move_string = perft::get_algebraic_move(&best_move);
    
    println!("Best move found: {}", move_string);
}