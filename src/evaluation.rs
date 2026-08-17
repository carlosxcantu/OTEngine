use crate::board::{self, BISHOPS, BLACK_PIECES, Board, KINGS, KNIGHTS, PAWNS, QUEENS, ROOKS, WHITE_PIECES};
use crate::constants::*;
use crate::magics::MagicBitBoards;

pub fn evaluation_board(board: &Board, magic_bitboards: &MagicBitBoards) -> i32
{
    // Retrieve incrementally calculated base
    let mut current_eval = board.incremental_eval;
    let phase_weight = board.phase_weight;
    let occypancy = board.get_bitboard(WHITE_PIECES) | board.get_bitboard(BLACK_PIECES);

    // 1. Evaluate Kings Dynamically
    let white_king_sq = (board.get_bitboard(KINGS) & board.get_bitboard(WHITE_PIECES)).trailing_zeros() as usize;
    let black_king_sq = (board.get_bitboard(KINGS) & board.get_bitboard(BLACK_PIECES)).trailing_zeros() as usize;
    
    let max_phase = phase_weight.min(24);
    
    current_eval += ((KING_MIDGAME_PST[white_king_sq] * max_phase) + (KING_ENDGAME_PST[white_king_sq] * (24 - max_phase))) / 24;
    current_eval -= ((KING_MIDGAME_PST[black_king_sq ^ 56] * max_phase) + (KING_ENDGAME_PST[black_king_sq ^ 56] * (24 - max_phase))) / 24;

    // 2. White King Safety
    if white_king_sq == 6 || white_king_sq == 7 
    { 
        let fgh_pawns = board.get_bitboard(PAWNS) & board.get_bitboard(WHITE_PIECES) & ((1u64 << 13) | (1u64 << 14) | (1u64 << 15));
        current_eval -= (3 - fgh_pawns.count_ones() as i32) * 20; 
    }
    if white_king_sq == 1 || white_king_sq == 2 
    { 
        let abc_pawns = board.get_bitboard(PAWNS) & board.get_bitboard(WHITE_PIECES) & ((1u64 << 8) | (1u64 << 9) | (1u64 << 10));
        current_eval -= (3 - abc_pawns.count_ones() as i32) * 20;
    }

    // 3. Black King Safety
    if black_king_sq == 62 || black_king_sq == 63 
    { 
        let fgh_pawns = board.get_bitboard(PAWNS) & board.get_bitboard(BLACK_PIECES) & ((1u64 << 53) | (1u64 << 54) | (1u64 << 55));
        current_eval += (3 - fgh_pawns.count_ones() as i32) * 20;
    }
    if black_king_sq == 57 || black_king_sq == 58 
    { 
        let abc_pawns = board.get_bitboard(PAWNS) & board.get_bitboard(BLACK_PIECES) & ((1u64 << 48) | (1u64 << 49) | (1u64 << 50));
        current_eval += (3 - abc_pawns.count_ones() as i32) * 20;
    }

    current_eval += mop_up_eval(white_king_sq, black_king_sq, current_eval);

    // White King X-Ray Danger
    let enemy_rooks_queens = board.get_bitboard(BLACK_PIECES) & (board.get_bitboard(ROOKS) | board.get_bitboard(QUEENS));
    if magic_bitboards.get_rook_attacks(white_king_sq, occypancy) & enemy_rooks_queens != 0 
    {
        current_eval -= 50; // Heavy penalty for King on same file/rank as enemy Rook
    }
    let enemy_bishops_queens = board.get_bitboard(BLACK_PIECES) & (board.get_bitboard(BISHOPS) | board.get_bitboard(QUEENS));
    if magic_bitboards.get_bishop_attacks(white_king_sq, occypancy) & enemy_bishops_queens != 0
    {
        current_eval -= 50; // Heavy penalty for King on same diagonal as enemy Bishop
    }

    // Black King X-Ray Danger
    let white_rooks_queens = board.get_bitboard(WHITE_PIECES) & (board.get_bitboard(ROOKS) | board.get_bitboard(QUEENS));
    if magic_bitboards.get_rook_attacks(black_king_sq, occypancy) & white_rooks_queens != 0 
    {
        current_eval += 50; 
    }
    let white_bishops_queens = board.get_bitboard(WHITE_PIECES) & (board.get_bitboard(BISHOPS) | board.get_bitboard(QUEENS));
    if magic_bitboards.get_bishop_attacks(black_king_sq, occypancy) & white_bishops_queens != 0 
    {
        current_eval += 50; 
    }

    if board.is_white_turn()
    {
        return current_eval
    }
    else 
    {
        return -current_eval
    }
}

fn mop_up_eval(winning_king_sq: usize, losing_king_sq: usize, material_lead: i32) -> i32 
{
    if material_lead.abs() < 300 
    {
        return 0;
    }

    let mut mop_score = 0;

    let losing_rank = (losing_king_sq / 8) as i32;
    let losing_file = (losing_king_sq % 8) as i32;
    let center_dist_rank = (losing_rank - 3).max(4 - losing_rank);
    let center_dist_file = (losing_file - 3).max(4 - losing_file);
    let center_distance = center_dist_rank + center_dist_file;
    mop_score += center_distance * 10; 

    let win_rank = (winning_king_sq / 8) as i32;
    let win_file = (winning_king_sq % 8) as i32;
    let king_distance = (win_rank - losing_rank).abs() + (win_file - losing_file).abs();
    mop_score += (14 - king_distance) * 8; 

    if material_lead > 0 { mop_score } else { -mop_score }
}