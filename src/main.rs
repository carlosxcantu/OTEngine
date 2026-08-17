mod board;
mod movedata;
mod movegenerator;
mod constants;
mod magics;
mod perft;
mod evaluation;
mod search;
mod zobrist;
mod transposition;
mod uci;
//lip_RpwV1s8lu5pGXzfnbSPE
use crate::{board::Board, magics::{MagicBitBoards, calculate_bishop_mask}, movegenerator::{MoveList, generate_psuedo_legal_moves}, zobrist::Zobrist, transposition::TranspositionTable};

// Python Script - 
// cd C:\Users\carlo\Desktop\Rust_Personal\OTEngine
// cd C:\Users\carlo\lichess-bot
// venv\Scripts\activate
// python lichess-bot.py
fn main() 
{
    // uci::uci_loop();
    let magic_bitboards = MagicBitBoards::new();
    let zobrist = Zobrist::new(); 
    let mut board = Board::new(&zobrist);

    println!("Engine is thinking...");
    let time_limit = 4000;   

    println!("Engine is thinking for {} seconds...", time_limit as f64 / 1000.0);

    let mut tt = TranspositionTable::new(4_194_304);
    let best_move = search::get_best_move(&mut board, time_limit, &magic_bitboards, &zobrist, &mut tt);
    let move_string = perft::get_algebraic_move(&best_move);
    
    println!("Best move found: {}", move_string);

    // perft::perft_divide(&mut board, &magic_bitboards, 7, &zobrist);
}