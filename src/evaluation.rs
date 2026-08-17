use crate::board::{self, BISHOPS, BLACK_PIECES, Board, KINGS, KNIGHTS, PAWNS, QUEENS, ROOKS, WHITE_PIECES};
use crate::constants::*;

pub fn evaluation_board(board: &Board) -> i32
{
    let mut current_eval: i32 = 0;
    let mut phase_weight = 0;
    let mut current_bitboard: u64 = board.get_bitboard(PAWNS) & board.get_bitboard(WHITE_PIECES);
    // Pawn eval
    while current_bitboard != 0
    {
        let target_square = current_bitboard.trailing_zeros();
        current_eval += PAWN_VALUE + PAWN_PST[target_square as usize];
        current_bitboard &= current_bitboard - 1;
    }

    current_bitboard = board.get_bitboard(PAWNS) & board.get_bitboard(BLACK_PIECES);
    while current_bitboard != 0
    {
        let target_square = current_bitboard.trailing_zeros() ^ 56;
        current_eval -= PAWN_VALUE + PAWN_PST[target_square as usize];
        current_bitboard &= current_bitboard - 1;
    }

    // Knight eval
    current_bitboard = board.get_bitboard(KNIGHTS) & board.get_bitboard(WHITE_PIECES);
    while current_bitboard != 0
    {
        let target_square = current_bitboard.trailing_zeros();
        current_eval += KNIGHT_VALUE + KNIGHT_PST[target_square as usize];
        current_bitboard &= current_bitboard - 1;
        phase_weight += 1;
    }

    current_bitboard = board.get_bitboard(KNIGHTS) & board.get_bitboard(BLACK_PIECES);
    while current_bitboard != 0
    {
        let target_square = current_bitboard.trailing_zeros() ^ 56;
        current_eval -= KNIGHT_VALUE + KNIGHT_PST[target_square as usize];
        current_bitboard &= current_bitboard - 1;
        phase_weight += 1;
    }

    // Bishop eval
    current_bitboard = board.get_bitboard(BISHOPS) & board.get_bitboard(WHITE_PIECES);
    while current_bitboard != 0
    {
        let target_square = current_bitboard.trailing_zeros();
        current_eval += BISHOP_VALUE + BISHOP_PST[target_square as usize];
        current_bitboard &= current_bitboard - 1;
        phase_weight += 1;
    }

    current_bitboard = board.get_bitboard(BISHOPS) & board.get_bitboard(BLACK_PIECES);
    while current_bitboard != 0
    {
        let target_square = current_bitboard.trailing_zeros() ^ 56;
        current_eval -= BISHOP_VALUE + BISHOP_PST[target_square as usize];
        current_bitboard &= current_bitboard - 1;
        phase_weight += 1;
    }

    // Rook eval
    current_bitboard = board.get_bitboard(ROOKS) & board.get_bitboard(WHITE_PIECES);
    while current_bitboard != 0
    {
        let target_square = current_bitboard.trailing_zeros();
        current_eval += ROOK_VALUE + ROOK_PST[target_square as usize];
        current_bitboard &= current_bitboard - 1;
        phase_weight += 2;
    }

    current_bitboard = board.get_bitboard(ROOKS) & board.get_bitboard(BLACK_PIECES);
    while current_bitboard != 0
    {
        let target_square = current_bitboard.trailing_zeros() ^ 56;
        current_eval -= ROOK_VALUE + ROOK_PST[target_square as usize];
        current_bitboard &= current_bitboard - 1;
        phase_weight += 2;
    }

    // Queen Eval
    current_bitboard = board.get_bitboard(QUEENS) & board.get_bitboard(WHITE_PIECES);
    while current_bitboard != 0
    {
        let target_square = current_bitboard.trailing_zeros();
        current_eval += QUEEN_VALUE + QUEEN_PST[target_square as usize];
        current_bitboard &= current_bitboard - 1;
        phase_weight += 4;
    }

    current_bitboard = board.get_bitboard(QUEENS) & board.get_bitboard(BLACK_PIECES);
    while current_bitboard != 0
    {
        let target_square = current_bitboard.trailing_zeros() ^ 56;
        current_eval -= QUEEN_VALUE + QUEEN_PST[target_square as usize];
        current_bitboard &= current_bitboard - 1;
        phase_weight += 4;
    }

    // King eval
    current_bitboard = board.get_bitboard(KINGS) & board.get_bitboard(WHITE_PIECES);
    let mut target_square = current_bitboard.trailing_zeros();
    let max_phase = phase_weight.min(24);
    current_eval += ((KING_MIDGAME_PST[target_square as usize] * max_phase) + (KING_ENDGAME_PST[target_square as usize] * (24 - max_phase))) / 24;
    current_bitboard = board.get_bitboard(KINGS) & board.get_bitboard(BLACK_PIECES);
    target_square = current_bitboard.trailing_zeros() ^ 56;
    current_eval -= ((KING_MIDGAME_PST[target_square as usize] * max_phase) + (KING_ENDGAME_PST[target_square as usize] * (24 - max_phase))) / 24;

    if board.is_white_turn()
    {
        return current_eval
    }
    else 
    {
        return -current_eval
    }
}