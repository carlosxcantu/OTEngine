mod board;
mod pieces;
mod engine;
mod movedata;
mod movegenerator;
mod constants;
mod magics;
use crate::{board::Board, magics::{MagicBitBoards, calculate_bishop_mask, find_magic_number}, movegenerator::{MoveList, generate_psuedo_legal_moves}};

fn main() 
{
    let magic_bitboards = MagicBitBoards::new();

    let mut board = Board::new();
    let mut move_list = MoveList::new();
    
    generate_psuedo_legal_moves(&board, &mut move_list, &magic_bitboards);
}