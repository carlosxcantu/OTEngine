//Modules
use crate::magics::MagicBitBoards;
use crate::movedata::Move;
use crate::board::{self, Board};
use crate::{constants::*, magics};
use std::mem::MaybeUninit;

pub fn generate_psuedo_legal_moves(board: &Board, move_list: &mut MoveList, magic_bitboards: &MagicBitBoards)
{
    let occupied_squares: u64 = board.get_bitboard(board::WHITE_PIECES) | board.get_bitboard(board::BLACK_PIECES);
    let empty_squares: u64 = !occupied_squares;

    generate_pawn_moves(board, move_list, empty_squares);
    generate_knight_moves(board, move_list);
    generate_king_moves(board, move_list, occupied_squares, magic_bitboards);
    generate_bishop_moves(board, move_list, occupied_squares, magic_bitboards);
    generate_rook_moves(board, move_list, occupied_squares, magic_bitboards);
    generate_queen_moves(board, move_list, occupied_squares, magic_bitboards);
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

pub fn generate_knight_moves(board: &Board, move_list: &mut MoveList)
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

pub fn generate_king_moves(board: &Board, move_list: &mut MoveList, occupied_squares: u64, magic_bitboards: &MagicBitBoards)
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

pub fn generate_rook_moves(board: &Board, move_list: &mut MoveList, occupied_squares: u64, magic_bitboards: &MagicBitBoards)
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

pub fn generate_bishop_moves(board: &Board, move_list: &mut MoveList, occupied_squares: u64, magic_bitboards: &MagicBitBoards)
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

pub fn generate_queen_moves(board: &Board, move_list: &mut MoveList, occupied_squares: u64, magic_bitboards: &MagicBitBoards)
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

pub fn generate_tactical_moves(board: &Board, move_list: &mut MoveList, magic_bitboards: &MagicBitBoards) 
{
    let occupied_squares = board.get_bitboard(board::WHITE_PIECES) | board.get_bitboard(board::BLACK_PIECES);
    let enemies = if board.is_white_turn() { board.get_bitboard(board::BLACK_PIECES) } else { board.get_bitboard(board::WHITE_PIECES) };
    let empty_squares = !occupied_squares;

    generate_tactical_pawn_moves(board, move_list, empty_squares, enemies);
    generate_tactical_piece_moves(board, move_list, occupied_squares, enemies, magic_bitboards);
}

pub fn generate_tactical_pawn_moves(board: &Board, move_list: &mut MoveList, empty_squares: u64, enemies: u64) 
{
    let mut pawn_bitboard = board.get_bitboard(board::PAWNS);

    if board.is_white_turn() 
    {
        pawn_bitboard &= board.get_bitboard(board::WHITE_PIECES);

        // Only generate pushes if they reach Rank 8 (Promotions)
        let single_pushes = (pawn_bitboard << 8) & empty_squares;
        let mut promoting_pushes = single_pushes & RANK_8;

        let mut right_captures = ((pawn_bitboard & ZERO_H_FILE) << 9) & enemies;
        let mut right_capture_promotion = right_captures & RANK_8;
        right_captures &= !RANK_8;

        let mut left_captures = ((pawn_bitboard & ZERO_A_FILE) << 7) & enemies;
        let mut left_capture_promotion = left_captures & RANK_8;
        left_captures &= !RANK_8;

        while right_captures != 0 
        {
            let target_square = right_captures.trailing_zeros();
            move_list.add_move(Move::new(target_square - 9, target_square, 0, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            right_captures &= right_captures - 1;
        }

        while left_captures != 0 
        {
            let target_square = left_captures.trailing_zeros();
            move_list.add_move(Move::new(target_square - 7, target_square, 0, board::PAWNS as u32, board.get_piece_from_array(target_square)));
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

            while right_ep != 0 {
                let target_square = right_ep.trailing_zeros();
                move_list.add_move(Move::new(target_square - 9, target_square, board::FLAG_EN_PASSANT, board::PAWNS as u32, board::PAWNS as u32));
                right_ep &= right_ep - 1;
            }

            while left_ep != 0 {
                let target_square = left_ep.trailing_zeros();
                move_list.add_move(Move::new(target_square - 7, target_square, board::FLAG_EN_PASSANT, board::PAWNS as u32, board::PAWNS as u32));
                left_ep &= left_ep - 1;
            }
        }
    } 
    else 
    {
        // Black pawns
        pawn_bitboard &= board.get_bitboard(board::BLACK_PIECES);

        let single_pushes = (pawn_bitboard >> 8) & empty_squares;
        let mut promoting_pushes = single_pushes & RANK_1;

        let mut right_captures = ((pawn_bitboard & ZERO_A_FILE) >> 9) & enemies;
        let mut right_capture_promotion = right_captures & RANK_1;
        right_captures &= !RANK_1;

        let mut left_captures = ((pawn_bitboard & ZERO_H_FILE) >> 7) & enemies;
        let mut left_capture_promotion = left_captures & RANK_1;
        left_captures &= !RANK_1;

        while right_captures != 0 
        {
            let target_square = right_captures.trailing_zeros();
            move_list.add_move(Move::new(target_square + 9, target_square, 0, board::PAWNS as u32, board.get_piece_from_array(target_square)));
            right_captures &= right_captures - 1;
        }

        while left_captures != 0 
        {
            let target_square = left_captures.trailing_zeros();
            move_list.add_move(Move::new(target_square + 7, target_square, 0, board::PAWNS as u32, board.get_piece_from_array(target_square)));
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

            while right_ep != 0 {
                let target_square = right_ep.trailing_zeros();
                move_list.add_move(Move::new(target_square + 9, target_square, board::FLAG_EN_PASSANT, board::PAWNS as u32, board::PAWNS as u32));
                right_ep &= right_ep - 1;
            }

            while left_ep != 0 {
                let target_square = left_ep.trailing_zeros();
                move_list.add_move(Move::new(target_square + 7, target_square, board::FLAG_EN_PASSANT, board::PAWNS as u32, board::PAWNS as u32));
                left_ep &= left_ep - 1;
            }
        }
    }
}

pub fn generate_tactical_piece_moves(board: &Board, move_list: &mut MoveList, occupied_squares: u64, enemies: u64, magic_bitboards: &MagicBitBoards) 
{
    let current_color = if board.is_white_turn() { board::WHITE_PIECES } else { board::BLACK_PIECES };

    // Knights
    let mut knight_bitboard = board.get_bitboard(board::KNIGHTS) & board.get_bitboard(current_color);
    while knight_bitboard != 0 
    {
        let start_square = knight_bitboard.trailing_zeros();
        let mut attacks = KNIGHT_ATTACK_MAP[start_square as usize] & enemies; // Filter by enemies mask
        while attacks != 0 
        {
            let target_square = attacks.trailing_zeros();
            move_list.add_move(Move::new(start_square, target_square, 0, board::KNIGHTS as u32, board.get_piece_from_array(target_square)));
            attacks &= attacks - 1;
        }
        knight_bitboard &= knight_bitboard - 1;
    }

    // Kings (No castling generated)
    let mut king_bitboard = board.get_bitboard(board::KINGS) & board.get_bitboard(current_color);
    while king_bitboard != 0 
    {
        let start_square = king_bitboard.trailing_zeros();
        let mut attacks = KING_ATTACK_MAP[start_square as usize] & enemies;
        while attacks != 0 
        {
            let target_square = attacks.trailing_zeros();
            move_list.add_move(Move::new(start_square, target_square, 0, board::KINGS as u32, board.get_piece_from_array(target_square)));
            attacks &= attacks - 1;
        }
        king_bitboard &= king_bitboard - 1;
    }

    // Bishops
    let mut bishop_bitboard = board.get_bitboard(board::BISHOPS) & board.get_bitboard(current_color);
    while bishop_bitboard != 0 
    {
        let start_square = bishop_bitboard.trailing_zeros();
        let mut attacks = magic_bitboards.get_bishop_attacks(start_square as usize, occupied_squares) & enemies;
        while attacks != 0 
        {
            let target_square = attacks.trailing_zeros();
            move_list.add_move(Move::new(start_square, target_square, 0, board::BISHOPS as u32, board.get_piece_from_array(target_square)));
            attacks &= attacks - 1;
        }
        bishop_bitboard &= bishop_bitboard - 1;
    }

    // Rooks
    let mut rook_bitboard = board.get_bitboard(board::ROOKS) & board.get_bitboard(current_color);
    while rook_bitboard != 0 
    {
        let start_square = rook_bitboard.trailing_zeros();
        let mut attacks = magic_bitboards.get_rook_attacks(start_square as usize, occupied_squares) & enemies;
        while attacks != 0 
        {
            let target_square = attacks.trailing_zeros();
            move_list.add_move(Move::new(start_square, target_square, 0, board::ROOKS as u32, board.get_piece_from_array(target_square)));
            attacks &= attacks - 1;
        }
        rook_bitboard &= rook_bitboard - 1;
    }

    // Queens
    let mut queen_bitboard = board.get_bitboard(board::QUEENS) & board.get_bitboard(current_color);
    while queen_bitboard != 0 
    {
        let start_square = queen_bitboard.trailing_zeros();
        let mut attacks = (magic_bitboards.get_bishop_attacks(start_square as usize, occupied_squares) | magic_bitboards.get_rook_attacks(start_square as usize, occupied_squares)) & enemies;
        while attacks != 0 
        {
            let target_square = attacks.trailing_zeros();
            move_list.add_move(Move::new(start_square, target_square, 0, board::QUEENS as u32, board.get_piece_from_array(target_square)));
            attacks &= attacks - 1;
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

    let enemy_rooks_and_queens = board.get_bitboard(board::ROOKS) | board.get_bitboard(board::QUEENS);
    if (enemy_rooks_and_queens & board.get_bitboard(enemy_color)) & magic_bitboards.get_rook_attacks(square, occupied_squares) > 0 
    {
        return true;
    }

    let enemy_bishops_and_queens = board.get_bitboard(board::BISHOPS) | board.get_bitboard(board::QUEENS);
    if (enemy_bishops_and_queens & board.get_bitboard(enemy_color)) & magic_bitboards.get_bishop_attacks(square, occupied_squares) > 0 
    {
        return true;
    }
    false
}

pub fn is_move_legal(board: &mut Board, magic_bitboards: &MagicBitBoards) -> bool 
{
    board.turn_end();
    let current_color = if board.is_white_turn() { board::WHITE_PIECES } else { board::BLACK_PIECES };
    let king_bitboard = board.get_bitboard(board::KINGS) & board.get_bitboard(current_color);
    if king_bitboard == 0 
    {
        board.turn_end();
        return false;
    }

    let king_square = king_bitboard.trailing_zeros() as usize;
    let occupied_squares = board.get_bitboard(board::WHITE_PIECES) | board.get_bitboard(board::BLACK_PIECES);
    let is_in_check = is_square_attacked(king_square, occupied_squares, board, magic_bitboards);
    board.turn_end();
    !is_in_check
}

// Static Exchange Evaluation. Simulates the full sequence of captures on
// `target_square`, assuming both sides always recapture with their least
// valuable attacker, and returns the net material result for the side
// initiating the capture (the side to move when this is called). A negative
// result means the initiating capture is a material-losing trade even after
// all recaptures are accounted for.
//
// This ignores absolute pins for speed (standard simplification used by
// essentially all engines that implement SEE) — a very rare inaccuracy
// compared to the search-depth gains from pruning bad captures.
pub fn static_exchange_evaluation(board: &Board, target_square: usize, from_square: usize, attacking_piece: usize, magic_bitboards: &MagicBitBoards) -> i32
{
    let mut gain = [0i32; 32];
    let mut depth = 0usize;
    let mut occupancy = board.get_bitboard(board::WHITE_PIECES) | board.get_bitboard(board::BLACK_PIECES);
    let mut side_is_white = board.is_white_turn();
    let mut current_attacker_piece = attacking_piece;

    gain[0] = PIECE_VALUES[board.get_piece_from_array(target_square as u32) as usize];
    occupancy &= !(1u64 << from_square);

    loop
    {
        depth += 1;
        gain[depth] = PIECE_VALUES[current_attacker_piece] - gain[depth - 1];

        // If neither side can improve their position further, stop early.
        if gain[depth].max(-gain[depth - 1]) < 0 
        { 
            break; 
        }

        side_is_white = !side_is_white;

        match least_valuable_attacker(board, target_square, occupancy, side_is_white, magic_bitboards)
        {
            Some((attacker_square, piece)) =>
            {
                occupancy &= !(1u64 << attacker_square);
                current_attacker_piece = piece;
            }
            None => break,
        }

        if depth >= 31 
        { 
            break; 
        }
    }

    while depth > 0
    {
        gain[depth - 1] = -(-gain[depth - 1]).max(gain[depth]);
        depth -= 1;
    }

    gain[0]
}

fn least_valuable_attacker(board: &Board, target_square: usize, occupancy: u64, white_attacker: bool, magic_bitboards: &MagicBitBoards) -> Option<(usize, usize)>
{
    let color_bb = (if white_attacker { board.get_bitboard(board::WHITE_PIECES) } else { board.get_bitboard(board::BLACK_PIECES) }) & occupancy;

    // Pawns
    let pawns = board.get_bitboard(board::PAWNS) & color_bb;
    let mut pawn_attackers = 0u64;
    if white_attacker
    {
        if target_square % 8 != 0 && target_square >= 9 { pawn_attackers |= 1u64 << (target_square - 9); }
        if target_square % 8 != 7 && target_square >= 7 { pawn_attackers |= 1u64 << (target_square - 7); }
    }
    else
    {
        if target_square % 8 != 7 && target_square + 9 < 64 { pawn_attackers |= 1u64 << (target_square + 9); }
        if target_square % 8 != 0 && target_square + 7 < 64 { pawn_attackers |= 1u64 << (target_square + 7); }
    }
    pawn_attackers &= pawns;
    if pawn_attackers != 0 
    { 
        return Some((pawn_attackers.trailing_zeros() as usize, board::PAWNS)); 
    }

    // Knights
    let knights = board.get_bitboard(board::KNIGHTS) & color_bb & KNIGHT_ATTACK_MAP[target_square];
    if knights != 0 
    { 
        return Some((knights.trailing_zeros() as usize, board::KNIGHTS)); 
    }

    // Bishops (recomputed against current shrinking occupancy so x-ray attacks are found)
    let bishop_attacks = magic_bitboards.get_bishop_attacks(target_square, occupancy);
    let bishops = board.get_bitboard(board::BISHOPS) & color_bb & bishop_attacks;
    if bishops != 0 
    { 
        return Some((bishops.trailing_zeros() as usize, board::BISHOPS)); 
    }

    // Rooks
    let rook_attacks = magic_bitboards.get_rook_attacks(target_square, occupancy);
    let rooks = board.get_bitboard(board::ROOKS) & color_bb & rook_attacks;
    if rooks != 0 
    { 
        return Some((rooks.trailing_zeros() as usize, board::ROOKS)); 
    }

    // Queens
    let queens = board.get_bitboard(board::QUEENS) & color_bb & (bishop_attacks | rook_attacks);
    if queens != 0 
    { 
        return Some((queens.trailing_zeros() as usize, board::QUEENS)); 
    }

    // King
    let king = board.get_bitboard(board::KINGS) & color_bb & KING_ATTACK_MAP[target_square];
    if king != 0 
    { 
        return Some((king.trailing_zeros() as usize, board::KINGS)); 
    }

    None
}

pub struct MoveList
{
    move_list : [MaybeUninit<Move>; 256],
    scores: [i32; 256],
    count: usize,
    order: [usize; 256],
}

impl MoveList
{
    pub fn new() -> Self
    {
        MoveList
        { 
            move_list: [const { MaybeUninit::uninit() }; 256], 
            scores: [0; 256],
            count: 0, 
            order: [0; 256],
        }
    }

    pub fn add_move(&mut self, move_data: Move)
    {
        self.move_list[self.count].write(move_data);
        self.count += 1;
    }

    pub fn get_move(&self, index: usize) -> Move 
    {
        unsafe {self.move_list[index].assume_init()}
    }

    pub fn get_count(&self) -> usize
    {
        self.count
    }

    pub fn score_moves(&mut self, board: &Board, tt_move: Option<Move>, killer_1: Option<Move>, killer_2: Option<Move>, history: &[[i32; 64]; 12]) 
    {
        for i in 0..self.count 
        {
            self.scores[i] = self.score_move(board, &self.get_move(i), tt_move, killer_1, killer_2, history);
            self.order[i] = i;
        }
        self.order[0..self.count].sort_by_key(|&i| std::cmp::Reverse(self.scores[i]));
    }

    pub fn pick_move(&mut self, index: usize) -> Move 
    {
        self.get_move(self.order[index])
    }

    // pub fn pick_move(&mut self, start_index: usize) -> Move 
    // {
    //     let mut best_score = -99999;
    //     let mut best_index = start_index;

    //     // Find the highest score from start_index to the end
    //     for i in start_index..self.count 
    //     {
    //         if self.scores[i] > best_score 
    //         {
    //             best_score = self.scores[i];
    //             best_index = i;
    //         }
    //     }

    //     // Swap both the move AND its score to the current position
    //     let temp_move = self.get_move(start_index);
    //     let best_move = self.get_move(best_index);
        
    //     self.move_list[start_index].write(best_move);
    //     self.move_list[best_index].write(temp_move);
    //     self.scores.swap(start_index, best_index);

    //     best_move
    // }

    fn score_move(&self, board: &Board, move_data: &Move, tt_move: Option<Move>, killer_1: Option<Move>, killer_2: Option<Move>, history: &[[i32; 64]; 12]) -> i32 
    {
        if let Some(best) = tt_move 
        {
            if *move_data == best 
            {
                return 30000;
            }
        }
        let mut score = 0;
        let captured = move_data.get_captured_piece();
        let piece = move_data.get_piece();
        let flags = move_data.get_flags();

        // MVV-LVA
        if captured != board::EMPTY_SQUARE as usize 
        {
            score = 10000 + (10 * PIECE_VALUES[captured] - PIECE_VALUES[piece]);
        }
        else if flags < board::FLAG_PROMOTE_QUEEN 
        {
            if let Some(k1) = killer_1 
            {
                if *move_data == k1 
                { 
                    return 9000; 
                }
            }
            if let Some(k2) = killer_2 
            {
                if *move_data == k2 
                { 
                    return 8000; 
                }
            }
            let color_offset = if board.is_white_turn() { 0 } else { 6 };
            let piece_index = (piece + color_offset) % 12;
            score += history[piece_index][(move_data.get_target() as usize) & 63];
        }

        // Promotion Bonuses
        if flags >= board::FLAG_PROMOTE_QUEEN && flags <= board::FLAG_PROMOTE_KNIGHT 
        {
            score += match flags {
                board::FLAG_PROMOTE_QUEEN => 9000,
                board::FLAG_PROMOTE_ROOK => 5000,
                board::FLAG_PROMOTE_BISHOP => 3300,
                board::FLAG_PROMOTE_KNIGHT => 3200,
                _ => 0,
            };
        }

        score
    }
}
