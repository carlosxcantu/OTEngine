use std::time::Instant;

use crate::board::{self, Board};
use crate::movedata::Move;
use crate::movegenerator::{generate_psuedo_legal_moves, is_square_attacked, MoveList};
use crate::magics::MagicBitBoards;

pub fn get_algebraic_move(move_data: &Move) -> String 
{
    let start = move_data.get_start();
    let target = move_data.get_target();

    let start_file = (start % 8) as u8;
    let start_rank = (start / 8) as u8;
    let target_file = (target % 8) as u8;
    let target_rank = (target / 8) as u8;

    let mut move_str = format!(
        "{}{}{}{}",
        (b'a' + start_file) as char,
        (b'1' + start_rank) as char,
        (b'a' + target_file) as char,
        (b'1' + target_rank) as char
    );

    // Append promotion piece if applicable
    let flag = move_data.get_flags();
    if flag >= board::FLAG_PROMOTE_QUEEN && flag <= board::FLAG_PROMOTE_KNIGHT 
    {
        let promo_char = match flag {
            board::FLAG_PROMOTE_QUEEN => 'q',
            board::FLAG_PROMOTE_ROOK => 'r',
            board::FLAG_PROMOTE_BISHOP => 'b',
            board::FLAG_PROMOTE_KNIGHT => 'n',
            _ => unreachable!(),
        };
        move_str.push(promo_char);
    }

    move_str
}

fn is_move_legal(board: &mut Board, magic_bitboards: &MagicBitBoards) -> bool 
{
    board.turn_end();
    let current_color = if board.is_white_turn() { board::WHITE_PIECES } else { board::BLACK_PIECES };
    let king_bitboard = board.get_bitboard(board::KINGS) & board.get_bitboard(current_color);
    let king_square = king_bitboard.trailing_zeros() as usize;
    let occupied_squares = board.get_bitboard(board::WHITE_PIECES) | board.get_bitboard(board::BLACK_PIECES);
    let is_in_check = is_square_attacked(king_square, occupied_squares, board, magic_bitboards);
    board.turn_end();
    !is_in_check
}

pub fn perft(board: &mut Board, magic_bitboards: &MagicBitBoards, depth: u8) -> u64 
{
    if depth == 0 
    {
        return 1;
    }

    let mut nodes: u64 = 0;
    let mut move_list = MoveList::new();

    generate_psuedo_legal_moves(board, &mut move_list, magic_bitboards);

    for i in 0..move_list.get_count() 
    {
        let the_move = move_list.get_move(i);
        board.make_move(&the_move);

        if is_move_legal(board, magic_bitboards) 
        {
            nodes += perft(board, magic_bitboards, depth - 1);
        }

        board.unmake_move(&the_move);
    }

    nodes
}

pub fn perft_divide(board: &mut Board, magic_bitboards: &MagicBitBoards, depth: u8) 
{
    if depth == 0 
    {
        println!("Depth 0 is 1 node.");
        return;
    }

    println!("--- Perft Divide Depth {} ---", depth);
    let start_time = Instant::now();
    let mut total_nodes: u64 = 0;

    let mut move_list = MoveList::new();
    generate_psuedo_legal_moves(board, &mut move_list, magic_bitboards);

    for i in 0..move_list.get_count() 
    {
        let the_move = move_list.get_move(i);
        board.make_move(&the_move);

        if is_move_legal(board, magic_bitboards) 
        {
            let branch_nodes = perft(board, magic_bitboards, depth - 1);
            println!("{}: {}", get_algebraic_move(&the_move), branch_nodes);
            total_nodes += branch_nodes;
        }

        board.unmake_move(&the_move);
    }

    let duration = start_time.elapsed();
    let seconds = duration.as_secs_f64();
    let nps = if seconds > 0.0 { (total_nodes as f64 / seconds) as u64 } else { 0 };

    println!("-----------------------------");
    println!("Total Nodes: {}", total_nodes);
    println!("Time Elapsed: {:.2} ms", seconds * 1000.0);
    println!("NPS (Nodes/Sec): {}", nps);
}