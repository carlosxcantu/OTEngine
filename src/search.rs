use std::cmp::max;

use crate::{board::{self, Board}, evaluation::evaluation_board, magics::MagicBitBoards, movedata::Move, movegenerator::{MoveList, generate_psuedo_legal_moves, is_square_attacked}};

const MAX_SCORE: i32 = 50000;
const MIN_SCORE: i32 = -50000;

pub fn search_root(board: &mut Board, depth: u8, magic_bitboards: &MagicBitBoards) -> Move 
{
    let mut best_score = MIN_SCORE;
    let mut best_move = Move::new(0, 0, 0, 0, 0); 
    let mut alpha = MIN_SCORE;
    let beta = MAX_SCORE;
    let mut move_list = MoveList::new();
    generate_psuedo_legal_moves(board, &mut move_list, magic_bitboards);
    move_list.sort_moves();

    for i in 0..move_list.get_count() 
    {
        let the_move = move_list.get_move(i);
        board.make_move(&the_move);

        if is_move_legal(board, magic_bitboards) 
        {
            let score = -minimax(board, depth - 1, -beta, -alpha, magic_bitboards);
            board.unmake_move(&the_move);

            if score > best_score 
            {
                best_score = score;
                best_move = the_move;
            }

            if score > alpha 
            {
                alpha = score;
            }
        } 
        else 
        {
            board.unmake_move(&the_move);
        }
    }

    best_move
}

pub fn minimax(board: &mut Board, depth: u8, mut alpha: i32, beta: i32, magic_bitboards: &MagicBitBoards) -> i32
{
    if depth == 0
    {
        return quiescence_search(board, alpha, beta, magic_bitboards);
    }

    let mut best_score = MIN_SCORE;
    let mut move_list = MoveList::new();
    generate_psuedo_legal_moves(board, &mut move_list, magic_bitboards);
    move_list.sort_moves();

    for i in 0..move_list.get_count() 
    {
        let the_move = move_list.get_move(i);
        board.make_move(&the_move);

        if is_move_legal(board, magic_bitboards) 
        {
            let score = -minimax(board, depth - 1, -beta, -alpha, magic_bitboards);
            best_score = max(best_score, score);
            alpha = max(score, alpha);
        }

        board.unmake_move(&the_move);
        
        if alpha >= beta
        {
            break;
        }
    }
    best_score
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

fn quiescence_search(board: &mut Board, mut alpha: i32, beta: i32, magic_bitboards: &MagicBitBoards) -> i32
{
    let pat: i32 = evaluation_board(board);

    if pat >= beta
    {
        return beta
    }

    if pat > alpha 
    {
        alpha = pat;
    }

    let mut move_list = MoveList::new();
    generate_psuedo_legal_moves(board, &mut move_list, magic_bitboards);
    move_list.sort_moves();

    for i in 0..move_list.get_count() 
    {
        let the_move = move_list.get_move(i);
        let is_capture = the_move.get_captured_piece() != board::EMPTY_SQUARE as usize;
        let is_promotion = the_move.get_flags() >= board::FLAG_PROMOTE_QUEEN && the_move.get_flags() <= board::FLAG_PROMOTE_KNIGHT;
        
        if !is_capture && !is_promotion 
        {
            continue;
        }

        board.make_move(&the_move);

        if is_move_legal(board, magic_bitboards) 
        {
            let score = -quiescence_search(board, -beta, -alpha, magic_bitboards);
            board.unmake_move(&the_move);

            if score >= beta 
            {
                return beta;
            }
            if score > alpha 
            {
                alpha = score;
            }
        } 
        else 
        {   
            board.unmake_move(&the_move);
        }
    }

    alpha
}