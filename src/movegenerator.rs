// Modules
use crate::movedata::Move;
use crate::board::{self, BLACK_PIECES, Board, PAWNS, WHITE_PIECES};


//Constants
const RANK_1: u64 = 0x00000000000000FF;
const RANK_2: u64 = 0x000000000000FF00;
const RANK_3: u64 = 0x0000000000FF0000;
const RANK_4: u64 = 0x00000000FF000000;
const RANK_5: u64 = 0x000000FF00000000;
const RANK_6: u64 = 0x0000FF0000000000;
const RANK_7: u64 = 0x00FF000000000000;
const RANK_8: u64 = 0xFF00000000000000;
const ZERO_A_FILE: u64 = 0xFEFEFEFEFEFEFEFE;
const ZERO_B_FILE: u64 = 0xFDFDFDFDFDFDFDFD;
const ZERO_C_FILE: u64 = 0xFBFBFBFBFBFBFBFB;
const ZERO_D_FILE: u64 = 0xF7F7F7F7F7F7F7F7;
const ZERO_E_FILE: u64 = 0xEFEFEFEFEFEFEFEF;
const ZERO_F_FILE: u64 = 0xDFDFDFDFDFDFDFDF;
const ZERO_G_FILE: u64 = 0xBFBFBFBFBFBFBFBF;
const ZERO_H_FILE: u64 = 0x7F7F7F7F7F7F7F7F;
const FILE_A: u64 = 0x0101010101010101;
const FILE_B: u64 = 0x0202020202020202;
const FILE_C: u64 = 0x0404040404040404;
const FILE_D: u64 = 0x0808080808080808;
const FILE_E: u64 = 0x1010101010101010;
const FILE_F: u64 = 0x2020202020202020;
const FILE_G: u64 = 0x4040404040404040;
const FILE_H: u64 = 0x8080808080808080;
const KNIGHT_ATTACK_MAP: [u64; 64] = calculate_knight_attack_map();

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
            let the_move: Move = Move::new(target_square - 16, target_square, 0, board::PAWNS as u32, board::EMPTY_SQUARE as u32);
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
            let the_move: Move = Move::new(target_square + 16, target_square, 0, board::PAWNS as u32, board::EMPTY_SQUARE as u32);
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

const fn calculate_knight_attack_map() -> [u64; 64]
{
    let mut attack_map: [u64; 64] = [0u64; 64];
    let mut current_square: usize = 0;

    while current_square < 64
    {
        // Creates respective bitboards
        let current_square_bitboard: u64 = 1u64 << current_square;
        let mut attack_bitboard: u64 = 0u64;

        // Applies Bitmask to attack Bitboard for all 8 directions
        attack_bitboard |= (current_square_bitboard & ZERO_H_FILE) << 17; //NE
        attack_bitboard |= (current_square_bitboard & ZERO_H_FILE & ZERO_G_FILE) << 10; //NE
        attack_bitboard |= (current_square_bitboard & ZERO_H_FILE & ZERO_G_FILE) >> 6; //SE
        attack_bitboard |= (current_square_bitboard & ZERO_H_FILE) >> 15; //SE
        attack_bitboard |= (current_square_bitboard & ZERO_A_FILE) >> 17; //SW
        attack_bitboard |= (current_square_bitboard & ZERO_A_FILE & ZERO_B_FILE) >> 10; //SW
        attack_bitboard |= (current_square_bitboard & ZERO_A_FILE & ZERO_B_FILE) << 6; //NW
        attack_bitboard |= (current_square_bitboard & ZERO_A_FILE) << 15; //NW

        // Sets respective square to the repsective bitmask
        attack_map[current_square] = attack_bitboard;
        current_square += 1;
    }
    attack_map
}