use crate::magics::MagicBitBoards;
// Modules
use crate::movedata::Move;
use crate::board::{self, Board};
use crate::{constants::*, magics};

pub fn generate_psuedo_legal_moves(board: &Board, move_list: &mut MoveList, magic_bitboards: &MagicBitBoards)
{
    let occupied_squares: u64 = board.get_bitboard(board::WHITE_PIECES) | board.get_bitboard(board::BLACK_PIECES);
    let empty_squares: u64 = !occupied_squares;

    generate_pawn_moves(board, move_list, empty_squares);
    generate_knight_moves(board, move_list, empty_squares);
    generate_king_moves(board, move_list, empty_squares, occupied_squares, magic_bitboards);
    generate_bishop_moves(board, move_list, empty_squares, occupied_squares, magic_bitboards);
    generate_rook_moves(board, move_list, empty_squares, occupied_squares, magic_bitboards);
    generate_queen_moves(board, move_list, empty_squares, occupied_squares, magic_bitboards);
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
        let mut promoting_pushes = single_pushes & RANK_8;
        single_pushes &= !RANK_8;
        let mut double_pushes: u64 = ((single_pushes & RANK_3) << 8) & empty_squares;

        // Right and Left Captures
        let mut right_captures: u64 = ((pawn_bitboard & ZERO_H_FILE) << 9) & board.get_bitboard(board::BLACK_PIECES);
        let mut right_capture_promotion = right_captures & RANK_8;
        right_captures &= !RANK_8;
        let mut left_captures: u64 = ((pawn_bitboard & ZERO_A_FILE) << 7) & board.get_bitboard(board::BLACK_PIECES);
        let mut left_capture_promotion = left_captures & RANK_8;
        left_captures &= !RANK_8;

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

        while promoting_pushes != 0 
        {
            let target_square = promoting_pushes.trailing_zeros();
            move_list.add_move(Move::new(target_square - 8, target_square, board::FLAG_PROMOTE_QUEEN, board::PAWNS as u32, board::EMPTY_SQUARE as u32));
            move_list.add_move(Move::new(target_square - 8, target_square, board::FLAG_PROMOTE_ROOK, board::PAWNS as u32, board::EMPTY_SQUARE as u32));
            move_list.add_move(Move::new(target_square - 8, target_square, board::FLAG_PROMOTE_BISHOP, board::PAWNS as u32, board::EMPTY_SQUARE as u32));
            move_list.add_move(Move::new(target_square - 8, target_square, board::FLAG_PROMOTE_KNIGHT, board::PAWNS as u32, board::EMPTY_SQUARE as u32));
            promoting_pushes &= promoting_pushes - 1;
        }

        while right_capture_promotion != 0 
        {
            let target_square = right_capture_promotion.trailing_zeros();
            move_list.add_move(Move::new(target_square - 9, target_square, board::FLAG_PROMOTE_QUEEN, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            move_list.add_move(Move::new(target_square - 9, target_square, board::FLAG_PROMOTE_ROOK, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            move_list.add_move(Move::new(target_square - 9, target_square, board::FLAG_PROMOTE_BISHOP, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            move_list.add_move(Move::new(target_square - 9, target_square, board::FLAG_PROMOTE_KNIGHT, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            right_capture_promotion &= right_capture_promotion - 1;
        }

        while left_capture_promotion != 0 
        {
            let target_square = left_capture_promotion.trailing_zeros();
            move_list.add_move(Move::new(target_square - 7, target_square, board::FLAG_PROMOTE_QUEEN, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            move_list.add_move(Move::new(target_square - 7, target_square, board::FLAG_PROMOTE_ROOK, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            move_list.add_move(Move::new(target_square - 7, target_square, board::FLAG_PROMOTE_BISHOP, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            move_list.add_move(Move::new(target_square - 7, target_square, board::FLAG_PROMOTE_KNIGHT, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            left_capture_promotion &= left_capture_promotion - 1;
        }

        if board.get_en_passant_target() != 0
        {
            let mut right_ep = ((pawn_bitboard & ZERO_H_FILE) << 9) & board.get_en_passant_target();
            let mut left_ep = ((pawn_bitboard & ZERO_A_FILE) << 7) & board.get_en_passant_target();

            while right_ep != 0 
            {
                let target_square = right_ep.trailing_zeros();
                move_list.add_move(Move::new(target_square - 9, target_square, board::FLAG_EN_PASSANT, board::PAWNS as u32, board::PAWNS as u32));
                right_ep &= right_ep - 1;
            }

            while left_ep != 0 
            {
                let target_square = left_ep.trailing_zeros();
                move_list.add_move(Move::new(target_square - 7, target_square, board::FLAG_EN_PASSANT, board::PAWNS as u32, board::PAWNS as u32));
                left_ep &= left_ep - 1;
            }
        }
    }
    else
    {
        // Current Black Pawns
        pawn_bitboard = pawn_bitboard & board.get_bitboard(board::BLACK_PIECES);

        // Single and Double Pawn Pushes
        let mut single_pushes: u64 = (pawn_bitboard >> 8) & empty_squares;
        let mut promoting_pushes = single_pushes & RANK_1;
        single_pushes &= !RANK_1;
        let mut double_pushes: u64 = ((single_pushes & RANK_6) >> 8) & empty_squares;

        // Right and Left Captures
        let mut right_captures: u64 = ((pawn_bitboard & ZERO_A_FILE) >> 9) & board.get_bitboard(board::WHITE_PIECES);
        let mut right_capture_promotion = right_captures & RANK_1;
        right_captures &= !RANK_1;
        let mut left_captures: u64 = ((pawn_bitboard & ZERO_H_FILE) >> 7) & board.get_bitboard(board::WHITE_PIECES);
        let mut left_capture_promotion = left_captures & RANK_1;
        left_captures &= !RANK_1;

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

        while promoting_pushes != 0 
        {
            let target_square = promoting_pushes.trailing_zeros();
            move_list.add_move(Move::new(target_square + 8, target_square, board::FLAG_PROMOTE_QUEEN, board::PAWNS as u32, board::EMPTY_SQUARE as u32));
            move_list.add_move(Move::new(target_square + 8, target_square, board::FLAG_PROMOTE_ROOK, board::PAWNS as u32, board::EMPTY_SQUARE as u32));
            move_list.add_move(Move::new(target_square + 8, target_square, board::FLAG_PROMOTE_BISHOP, board::PAWNS as u32, board::EMPTY_SQUARE as u32));
            move_list.add_move(Move::new(target_square + 8, target_square, board::FLAG_PROMOTE_KNIGHT, board::PAWNS as u32, board::EMPTY_SQUARE as u32));
            promoting_pushes &= promoting_pushes - 1;
        }

        while right_capture_promotion != 0 
        {
            let target_square = right_capture_promotion.trailing_zeros();
            move_list.add_move(Move::new(target_square + 9, target_square, board::FLAG_PROMOTE_QUEEN, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            move_list.add_move(Move::new(target_square + 9, target_square, board::FLAG_PROMOTE_ROOK, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            move_list.add_move(Move::new(target_square + 9, target_square, board::FLAG_PROMOTE_BISHOP, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            move_list.add_move(Move::new(target_square + 9, target_square, board::FLAG_PROMOTE_KNIGHT, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            right_capture_promotion &= right_capture_promotion - 1;
        }

        while left_capture_promotion != 0 
        {
            let target_square = left_capture_promotion.trailing_zeros();
            move_list.add_move(Move::new(target_square + 7, target_square, board::FLAG_PROMOTE_QUEEN, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            move_list.add_move(Move::new(target_square + 7, target_square, board::FLAG_PROMOTE_ROOK, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            move_list.add_move(Move::new(target_square + 7, target_square, board::FLAG_PROMOTE_BISHOP, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            move_list.add_move(Move::new(target_square + 7, target_square, board::FLAG_PROMOTE_KNIGHT, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            left_capture_promotion &= left_capture_promotion - 1;
        }

        if board.get_en_passant_target() != 0
        {
            let mut right_ep = ((pawn_bitboard & ZERO_A_FILE) >> 9) & board.get_en_passant_target();
            let mut left_ep = ((pawn_bitboard & ZERO_H_FILE) >> 7) & board.get_en_passant_target();

            while right_ep != 0 
            {
                let target_square = right_ep.trailing_zeros();
                move_list.add_move(Move::new(target_square + 9, target_square, board::FLAG_EN_PASSANT, board::PAWNS as u32, board::PAWNS as u32));
                right_ep &= right_ep - 1;
            }

            while left_ep != 0 
            {
                let target_square = left_ep.trailing_zeros();
                move_list.add_move(Move::new(target_square + 7, target_square, board::FLAG_EN_PASSANT, board::PAWNS as u32, board::PAWNS as u32));
                left_ep &= left_ep - 1;
            }
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

pub fn generate_king_moves(board: &Board, move_list: &mut MoveList, empty_squares: u64, occupied_squares: u64, magic_bitboards: &MagicBitBoards)
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

    let castling_rights = board.get_castling_rights();

    if board.is_white_turn() 
    {
        // White King-side (O-O) (0001)
        if (castling_rights & 1) != 0 
        {
            // Path check
            if (occupied_squares & ((1u64 << 5) | (1u64 << 6))) == 0 
            {
                // Safety check
                if !is_square_attacked(4, occupied_squares, board, magic_bitboards) &&
                    !is_square_attacked(5, occupied_squares, board, magic_bitboards) &&
                    !is_square_attacked(6, occupied_squares, board, magic_bitboards) 
                {
                    move_list.add_move(Move::new(4, 6, board::FLAG_KING_CASTLE, board::KINGS as u32, board::EMPTY_SQUARE as u32));
                }
            }
        }

        // White Queen-side (O-O-O) (0010)
        if (castling_rights & 2) != 0 
        {
            // Path check
            if (occupied_squares & ((1u64 << 1) | (1u64 << 2) | (1u64 << 3))) == 0 
            {
                // Safety check
                if !is_square_attacked(4, occupied_squares, board, magic_bitboards) &&
                    !is_square_attacked(3, occupied_squares, board, magic_bitboards) &&
                    !is_square_attacked(2, occupied_squares, board, magic_bitboards) 
                {
                    move_list.add_move(Move::new(4, 2, board::FLAG_QUEEN_CASTLE, board::KINGS as u32, board::EMPTY_SQUARE as u32));
                }
            }
        }
    } 
    else 
    {
        // Black King-side (O-O) (0100)
        if (castling_rights & 4) != 0 
        {
            // Path check
            if (occupied_squares & ((1u64 << 61) | (1u64 << 62))) == 0 
            {
                // Safety check
                if !is_square_attacked(60, occupied_squares, board, magic_bitboards) &&
                    !is_square_attacked(61, occupied_squares, board, magic_bitboards) &&
                    !is_square_attacked(62, occupied_squares, board, magic_bitboards) 
                {
                    move_list.add_move(Move::new(60, 62, board::FLAG_KING_CASTLE, board::KINGS as u32, board::EMPTY_SQUARE as u32));
                }
            }
        }

        // Black Queen-side (O-O-O) (1000)
        if (castling_rights & 8) != 0 
        {
            // Path check
            if (occupied_squares & ((1u64 << 57) | (1u64 << 58) | (1u64 << 59))) == 0 
            {
                // Safety check
                if !is_square_attacked(60, occupied_squares, board, magic_bitboards) &&
                    !is_square_attacked(59, occupied_squares, board, magic_bitboards) &&
                    !is_square_attacked(58, occupied_squares, board, magic_bitboards) 
                {
                    move_list.add_move(Move::new(60, 58, board::FLAG_QUEEN_CASTLE, board::KINGS as u32, board::EMPTY_SQUARE as u32));
                }
            }
        }
    }
}

pub fn generate_rook_moves(board: &Board, move_list: &mut MoveList, empty_squares: u64, occupied_squares: u64, magic_bitboards: &MagicBitBoards)
{
    let current_color = if board.is_white_turn() {board::WHITE_PIECES} else {board::BLACK_PIECES};
    let mut rook_bitboard = board.get_bitboard(board::ROOKS) & board.get_bitboard(current_color);

    while rook_bitboard != 0 
    {
        let start_square = rook_bitboard.trailing_zeros();
        let mut current_rook_bitboard = magic_bitboards.get_rook_attacks(start_square as usize, occupied_squares);
        current_rook_bitboard &= !board.get_bitboard(current_color);
        while current_rook_bitboard != 0
        {
            let target_square = current_rook_bitboard.trailing_zeros();
            let the_move: Move = Move::new(start_square, target_square, 0, board::ROOKS as u32, board.get_piece_from_array(target_square));
            move_list.add_move(the_move);
            current_rook_bitboard &= current_rook_bitboard - 1;
        }
        rook_bitboard &= rook_bitboard - 1;
    }
}

pub fn generate_bishop_moves(board: &Board, move_list: &mut MoveList, empty_squares: u64, occupied_squares: u64, magic_bitboards: &MagicBitBoards)
{
    let current_color = if board.is_white_turn() {board::WHITE_PIECES} else {board::BLACK_PIECES};
    let mut bishop_bitboard = board.get_bitboard(board::BISHOPS) & board.get_bitboard(current_color);

    while bishop_bitboard != 0 
    {
        let start_square = bishop_bitboard.trailing_zeros();
        let mut current_bishop_bitboard = magic_bitboards.get_bishop_attacks(start_square as usize, occupied_squares);
        current_bishop_bitboard &= !board.get_bitboard(current_color);
        while current_bishop_bitboard != 0
        {
            let target_square = current_bishop_bitboard.trailing_zeros();
            let the_move: Move = Move::new(start_square, target_square, 0, board::BISHOPS as u32, board.get_piece_from_array(target_square));
            move_list.add_move(the_move);
            current_bishop_bitboard &= current_bishop_bitboard - 1;
        }
        bishop_bitboard &= bishop_bitboard - 1;
    }
}

pub fn generate_queen_moves(board: &Board, move_list: &mut MoveList, empty_squares: u64, occupied_squares: u64, magic_bitboards: &MagicBitBoards)
{
    let current_color = if board.is_white_turn() {board::WHITE_PIECES} else {board::BLACK_PIECES};
    let mut queen_bitboard = board.get_bitboard(board::QUEENS) & board.get_bitboard(current_color);

    while queen_bitboard != 0 
    {
        let start_square = queen_bitboard.trailing_zeros();
        let mut current_queen_bitboard = magic_bitboards.get_bishop_attacks(start_square as usize, occupied_squares) | magic_bitboards.get_rook_attacks(start_square as usize, occupied_squares);
        current_queen_bitboard &= !board.get_bitboard(current_color);
        while current_queen_bitboard != 0
        {
            let target_square = current_queen_bitboard.trailing_zeros();
            let the_move: Move = Move::new(start_square, target_square, 0, board::QUEENS as u32, board.get_piece_from_array(target_square));
            move_list.add_move(the_move);
            current_queen_bitboard &= current_queen_bitboard - 1;
        }
        queen_bitboard &= queen_bitboard - 1;
    }
}

pub fn is_square_attacked(square: usize, occupied_squares: u64, board: &Board, magic_bitboards: &MagicBitBoards) -> bool
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

    current_bitboard = board.get_bitboard(board::ROOKS) & board.get_bitboard(enemy_color);
    if current_bitboard & magic_bitboards.get_rook_attacks(square, occupied_squares) > 0
    {
        return true;
    }

    current_bitboard = board.get_bitboard(board::BISHOPS) & board.get_bitboard(enemy_color);
    if current_bitboard & magic_bitboards.get_bishop_attacks(square, occupied_squares) > 0
    {
        return true;
    }

    current_bitboard = board.get_bitboard(board::QUEENS) & board.get_bitboard(enemy_color);
    if current_bitboard & (magic_bitboards.get_bishop_attacks(square, occupied_squares) | magic_bitboards.get_rook_attacks(square, occupied_squares)) > 0
    {
        return true;
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