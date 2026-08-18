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

    // 4. Bishop Pair
    let white_bishops = board.get_bitboard(BISHOPS) & board.get_bitboard(WHITE_PIECES);
    let black_bishops = board.get_bitboard(BISHOPS) & board.get_bitboard(BLACK_PIECES);
    if white_bishops.count_ones() >= 2 { current_eval += 30; }
    if black_bishops.count_ones() >= 2 { current_eval -= 30; }

    // 5. Mobility (Knights, Bishops, Rooks, Queens)
    current_eval += mobility_eval(board, magic_bitboards, occypancy, true);
    current_eval -= mobility_eval(board, magic_bitboards, occypancy, false);

    // 6. Rooks on Open / Semi-Open Files
    current_eval += rook_file_eval(board, true);
    current_eval -= rook_file_eval(board, false);

    // 7. Passed Pawns
    current_eval += passed_pawn_eval(board, true);
    current_eval -= passed_pawn_eval(board, false);

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

fn mobility_eval(board: &Board, magic_bitboards: &MagicBitBoards, occupancy: u64, white: bool) -> i32
{
    let own = if white { board.get_bitboard(WHITE_PIECES) } else { board.get_bitboard(BLACK_PIECES) };
    let mut score = 0;

    let mut knights = board.get_bitboard(KNIGHTS) & own;
    while knights != 0
    {
        let sq = knights.trailing_zeros() as usize;
        let attacks = KNIGHT_ATTACK_MAP[sq] & !own;
        score += attacks.count_ones() as i32 * 4;
        knights &= knights - 1;
    }

    let mut bishops = board.get_bitboard(BISHOPS) & own;
    while bishops != 0
    {
        let sq = bishops.trailing_zeros() as usize;
        let attacks = magic_bitboards.get_bishop_attacks(sq, occupancy) & !own;
        score += attacks.count_ones() as i32 * 3;
        bishops &= bishops - 1;
    }

    let mut rooks = board.get_bitboard(ROOKS) & own;
    while rooks != 0
    {
        let sq = rooks.trailing_zeros() as usize;
        let attacks = magic_bitboards.get_rook_attacks(sq, occupancy) & !own;
        score += attacks.count_ones() as i32 * 2;
        rooks &= rooks - 1;
    }

    let mut queens = board.get_bitboard(QUEENS) & own;
    while queens != 0
    {
        let sq = queens.trailing_zeros() as usize;
        let attacks = (magic_bitboards.get_bishop_attacks(sq, occupancy) | magic_bitboards.get_rook_attacks(sq, occupancy)) & !own;
        score += attacks.count_ones() as i32;
        queens &= queens - 1;
    }

    score
}

fn rook_file_eval(board: &Board, white: bool) -> i32
{
    let own = if white { board.get_bitboard(WHITE_PIECES) } else { board.get_bitboard(BLACK_PIECES) };
    let all_pawns = board.get_bitboard(PAWNS);
    let own_pawns = all_pawns & own;
    let mut rooks = board.get_bitboard(ROOKS) & own;
    let mut score = 0;

    while rooks != 0
    {
        let sq = rooks.trailing_zeros() as usize;
        let file = sq % 8;
        let file_mask: u64 = 0x0101_0101_0101_0101u64 << file;

        if all_pawns & file_mask == 0
        {
            score += 25; // Fully open file
        }
        else if own_pawns & file_mask == 0
        {
            score += 12; // Semi-open (no own pawns blocking)
        }
        rooks &= rooks - 1;
    }

    score
}

fn passed_pawn_eval(board: &Board, white: bool) -> i32
{
    const PASSED_BONUS: [i32; 8] = [0, 5, 10, 20, 35, 60, 100, 0];

    let own = if white { board.get_bitboard(WHITE_PIECES) } else { board.get_bitboard(BLACK_PIECES) };
    let enemy_pawns = if white
    {
        board.get_bitboard(PAWNS) & board.get_bitboard(BLACK_PIECES)
    }
    else
    {
        board.get_bitboard(PAWNS) & board.get_bitboard(WHITE_PIECES)
    };

    let mut pawns = board.get_bitboard(PAWNS) & own;
    let mut score = 0;

    while pawns != 0
    {
        let sq = pawns.trailing_zeros() as usize;
        let file = (sq % 8) as i32;
        let rank = (sq / 8) as i32;

        // Union of this pawn's file and adjacent files
        let mut span: u64 = 0;
        for f in (file - 1)..=(file + 1)
        {
            if f < 0 || f > 7 { continue; }
            span |= 0x0101_0101_0101_0101u64 << f;
        }

        // Restrict to ranks strictly ahead of this pawn (in its direction of travel)
        if white
        {
            for r in 0..=rank
            {
                span &= !(0xFFu64 << (r * 8));
            }
        }
        else
        {
            for r in rank..8
            {
                span &= !(0xFFu64 << (r * 8));
            }
        }

        if enemy_pawns & span == 0
        {
            let advance = if white { rank } else { 7 - rank };
            score += PASSED_BONUS[advance as usize];
        }

        pawns &= pawns - 1;
    }

    score
}