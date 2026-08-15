// Modules
use crate::movedata::Move;
use crate::board::{self, Board};
use crate::constants::*;

pub fn generate_psuedo_legal_moves(board: &Board, move_list: &mut MoveList)
{
    let occupied_squares: u64 = board.get_bitboard(board::WHITE_PIECES) | board.get_bitboard(board::BLACK_PIECES);
    let empty_squares: u64 = !occupied_squares;

    generate_pawn_moves(board, move_list, empty_squares);
    generate_knight_moves(board, move_list, empty_squares);
    generate_king_moves(board, move_list, empty_squares);
    generate_bishop_moves(board, move_list, empty_squares, occupied_squares);
    generate_rook_moves(board, move_list, empty_squares, occupied_squares);
    generate_queen_moves(board, move_list, empty_squares, occupied_squares);
}

pub fn generate_pawn_moves(board: &Board, move_list: &mut MoveList, empty_squares: u64)
{
    let mut pawn_bitboard: u64 = board.get_bitboard(board::PAWNS);

    if board.is_white_turn()
    {
        // Current White Pawns
        pawn_bitboard = pawn_bitboard & board.get_bitboard(board::WHITE_PIECES);

        // Single and Double Pawn Pushes
        let mut single_pushes: u64 = (pawn_bitboard << 8) & empty_squares;
        let mut double_pushes: u64 = ((single_pushes & RANK_3) << 8) & empty_squares;

        // Right and Left Captures
        let mut right_captures: u64 = ((pawn_bitboard & ZERO_H_FILE) << 9) & board.get_bitboard(board::BLACK_PIECES);
        let mut left_captures: u64 = ((pawn_bitboard & ZERO_A_FILE) << 7) & board.get_bitboard(board::BLACK_PIECES);

        // TODO Pawn Promotion
        // TODO En Passant

        // Loops for move generation
        while single_pushes != 0 
        {
            let target_square = single_pushes.trailing_zeros();
            let the_move: Move = Move::new(target_square - 8, target_square, 0, board::PAWNS as u32, board::EMPTY_SQUARE as u32);
            move_list.add_move(the_move);
            single_pushes &= single_pushes - 1;
        }

        while double_pushes != 0 
        {
            let target_square = double_pushes.trailing_zeros();
            let the_move: Move = Move::new(target_square - 16, target_square, board::FLAG_DOUBLE_PUSH, board::PAWNS as u32, board::EMPTY_SQUARE as u32);
            move_list.add_move(the_move);
            double_pushes &= double_pushes - 1;
        }

        while right_captures != 0 
        {
            let target_square = right_captures.trailing_zeros();
            let the_move: Move = Move::new(target_square - 9, target_square, 0, board::PAWNS as u32, board.get_piece_from_array(target_square));
            move_list.add_move(the_move);
            right_captures &= right_captures - 1;
        }

        while left_captures != 0 
        {
            let target_square = left_captures.trailing_zeros();
            let the_move: Move = Move::new(target_square - 7, target_square, 0, board::PAWNS as u32, board.get_piece_from_array(target_square));
            move_list.add_move(the_move);
            left_captures &= left_captures - 1;
        }
    }
    else
    {
        // Current Black Pawns
        pawn_bitboard = pawn_bitboard & board.get_bitboard(board::BLACK_PIECES);

        // Single and Double Pawn Pushes
        let mut single_pushes: u64 = (pawn_bitboard >> 8) & empty_squares;
        let mut double_pushes: u64 = ((single_pushes & RANK_6) >> 8) & empty_squares;

        // Right and Left Captures
        let mut right_captures: u64 = ((pawn_bitboard & ZERO_A_FILE) >> 9) & board.get_bitboard(board::WHITE_PIECES);
        let mut left_captures: u64 = ((pawn_bitboard & ZERO_H_FILE) >> 7) & board.get_bitboard(board::WHITE_PIECES);
        
        // TODO Pawn Promotion
        // TODO En Passant

        // Loops for Move Generation
        while single_pushes != 0 
        {
            let target_square = single_pushes.trailing_zeros();
            let the_move: Move = Move::new(target_square + 8, target_square, 0, board::PAWNS as u32, board::EMPTY_SQUARE as u32);
            move_list.add_move(the_move);
            single_pushes &= single_pushes - 1;
        }

        while double_pushes != 0 
        {
            let target_square = double_pushes.trailing_zeros();
            let the_move: Move = Move::new(target_square + 16, target_square, board::FLAG_DOUBLE_PUSH, board::PAWNS as u32, board::EMPTY_SQUARE as u32);
            move_list.add_move(the_move);
            double_pushes &= double_pushes - 1;
        }

        while right_captures != 0 
        {
            let target_square = right_captures.trailing_zeros();
            let the_move: Move = Move::new(target_square + 9, target_square, 0, board::PAWNS as u32, board.get_piece_from_array(target_square));
            move_list.add_move(the_move);
            right_captures &= right_captures - 1;
        }

        while left_captures != 0 
        {
            let target_square = left_captures.trailing_zeros();
            let the_move: Move = Move::new(target_square + 7, target_square, 0, board::PAWNS as u32, board.get_piece_from_array(target_square));
            move_list.add_move(the_move);
            left_captures &= left_captures - 1;
        }
    }
}

pub fn generate_knight_moves(board: &Board, move_list: &mut MoveList, empty_squares: u64)
{
    let current_color = if board.is_white_turn() {board::WHITE_PIECES} else {board::BLACK_PIECES};
    let mut knight_bitboard = board.get_bitboard(board::KNIGHTS) & board.get_bitboard(current_color);

    while knight_bitboard != 0 
    {
        let start_square = knight_bitboard.trailing_zeros();
        let mut current_knight_bitboard = KNIGHT_ATTACK_MAP[start_square as usize];
        current_knight_bitboard &= !board.get_bitboard(current_color);
        while current_knight_bitboard != 0
        {
            let target_square = current_knight_bitboard.trailing_zeros();
            let the_move: Move = Move::new(start_square, target_square, 0, board::KNIGHTS as u32, board.get_piece_from_array(target_square));
            move_list.add_move(the_move);
            current_knight_bitboard &= current_knight_bitboard - 1;
        }
        knight_bitboard &= knight_bitboard - 1;
    }
}

pub fn generate_king_moves(board: &Board, move_list: &mut MoveList, empty_squares: u64)
{
    let current_color = if board.is_white_turn() {board::WHITE_PIECES} else {board::BLACK_PIECES};
    let mut king_bitboard = board.get_bitboard(board::KINGS) & board.get_bitboard(current_color);

    while king_bitboard != 0 
    {
        let start_square = king_bitboard.trailing_zeros();
        let mut current_king_bitboard = KING_ATTACK_MAP[start_square as usize];
        current_king_bitboard &= !board.get_bitboard(current_color);
        while current_king_bitboard != 0
        {
            let target_square = current_king_bitboard.trailing_zeros();
            let the_move: Move = Move::new(start_square, target_square, 0, board::KINGS as u32, board.get_piece_from_array(target_square));
            move_list.add_move(the_move);
            current_king_bitboard &= current_king_bitboard - 1;
        }
        king_bitboard &= king_bitboard - 1;
    }
}

pub fn generate_bishop_moves(board: &Board, move_list: &mut MoveList, empty_squares: u64, occupied_squares: u64)
{

}

pub fn generate_rook_moves(board: &Board, move_list: &mut MoveList, empty_squares: u64, occupied_squares: u64)
{
    
}

pub fn generate_queen_moves(board: &Board, move_list: &mut MoveList, empty_squares: u64, occupied_squares: u64)
{
    
}

pub fn is_square_attacked(square: usize, board: &Board) -> bool
{
    let enemy_color: usize = if board.is_white_turn() {board::BLACK_PIECES} else {board::WHITE_PIECES};

    let mut current_bitboard = board.get_bitboard(board::KNIGHTS) & board.get_bitboard(enemy_color);
    if current_bitboard & KNIGHT_ATTACK_MAP[square] > 0
    {
        return true;
    }
    
    current_bitboard = board.get_bitboard(board::KINGS) & board.get_bitboard(enemy_color);
    if current_bitboard & KING_ATTACK_MAP[square] > 0
    {
        return true;
    }

    current_bitboard = board.get_bitboard(board::PAWNS) & board.get_bitboard(enemy_color);
    let target_mask = 1u64 << square;

    if board.is_white_turn()
    {
        let left_attack = (target_mask & ZERO_A_FILE) << 7;
        let right_attack = (target_mask & ZERO_H_FILE) << 9;

        if current_bitboard & (left_attack | right_attack) > 0
        {
            return true;
        }
    }
    else
    {
        let left_attack = (target_mask & ZERO_H_FILE) >> 7;
        let right_attack = (target_mask & ZERO_A_FILE) >> 9;

        if current_bitboard & (left_attack | right_attack) > 0
        {
            return true;
        }
    }
    false
}

pub struct MoveList
{
    move_list : [Move; 256],
    count: usize,
}

impl MoveList
{
    pub fn new() -> Self
    {
        MoveList
        { 
            move_list: [Move::new(0,0,0,0,0); 256], 
            count: 0, 
        }
    }

    pub fn add_move(&mut self, move_data: Move)
    {
        self.move_list[self.count] = move_data;
        self.count += 1;
    }
}