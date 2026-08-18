use crate::board::{self, BISHOPS, BLACK_PIECES, Board, KINGS, KNIGHTS, PAWNS, QUEENS, ROOKS, WHITE_PIECES};
use crate::constants::*;
use crate::magics::MagicBitBoards;

pub fn evaluation_board(board: &Board, magic_bitboards: &MagicBitBoards) -> i32
{
    // Retrieve incrementally calculated base
    let mut current_eval = board.incremental_eval;
    let phase_weight = board.phase_weight;
    let occypancy = board.get_bitboard(WHITE_PIECES) | board.get_bitboard(BLACK_PIECES);

    // Evaluate Kings Dynamically
    let white_king_sq = (board.get_bitboard(KINGS) & board.get_bitboard(WHITE_PIECES)).trailing_zeros() as usize;
    let black_king_sq = (board.get_bitboard(KINGS) & board.get_bitboard(BLACK_PIECES)).trailing_zeros() as usize;
    
    let max_phase = phase_weight.min(24);
    
    current_eval += ((KING_MIDGAME_PST[white_king_sq] * max_phase) + (KING_ENDGAME_PST[white_king_sq] * (24 - max_phase))) / 24;
    current_eval -= ((KING_MIDGAME_PST[black_king_sq ^ 56] * max_phase) + (KING_ENDGAME_PST[black_king_sq ^ 56] * (24 - max_phase))) / 24;

    // White King Safety
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

    // Black King Safety
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

    // Bishop Pair
    let white_bishops = board.get_bitboard(BISHOPS) & board.get_bitboard(WHITE_PIECES);
    let black_bishops = board.get_bitboard(BISHOPS) & board.get_bitboard(BLACK_PIECES);
    if white_bishops.count_ones() >= 2 { current_eval += 30; }
    if black_bishops.count_ones() >= 2 { current_eval -= 30; }

    // Mobility + King Safety 
    let white_king_zone = KING_ATTACK_MAP[white_king_sq] | (1u64 << white_king_sq);
    let black_king_zone = KING_ATTACK_MAP[black_king_sq] | (1u64 << black_king_sq);

    let (white_mobility, white_attack_units) = mobility_and_king_pressure(board, magic_bitboards, occypancy, true, black_king_zone);
    let (black_mobility, black_attack_units) = mobility_and_king_pressure(board, magic_bitboards, occypancy, false, white_king_zone);

    current_eval += white_mobility - black_mobility;

    // King danger scales with material on the board 
    current_eval += (king_danger_penalty(white_attack_units) * max_phase) / 24;
    current_eval -= (king_danger_penalty(black_attack_units) * max_phase) / 24;

    // Rooks on Open / Semi-Open Files
    current_eval += rook_file_eval(board, true);
    current_eval -= rook_file_eval(board, false);

    // Passed Pawns
    current_eval += passed_pawn_eval(board, true);
    current_eval -= passed_pawn_eval(board, false);

    // Pawn Structure (Doubled / Isolated / Backward Pawns)
    current_eval -= pawn_structure_eval(board, true) + backward_pawn_eval(board, true);
    current_eval += pawn_structure_eval(board, false) + backward_pawn_eval(board, false);
    current_eval += mop_up_eval(white_king_sq, black_king_sq, current_eval);

    // White King X-Ray Danger
    let enemy_rooks_queens = board.get_bitboard(BLACK_PIECES) & (board.get_bitboard(ROOKS) | board.get_bitboard(QUEENS));
    if magic_bitboards.get_rook_attacks(white_king_sq, occypancy) & enemy_rooks_queens != 0 
    {
        current_eval -= 50; 
    }
    let enemy_bishops_queens = board.get_bitboard(BLACK_PIECES) & (board.get_bitboard(BISHOPS) | board.get_bitboard(QUEENS));
    if magic_bitboards.get_bishop_attacks(white_king_sq, occypancy) & enemy_bishops_queens != 0
    {
        current_eval -= 50;
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

fn mobility_and_king_pressure(board: &Board, magic_bitboards: &MagicBitBoards, occupancy: u64, white: bool, enemy_king_zone: u64) -> (i32, i32)
{
    let own = if white { board.get_bitboard(WHITE_PIECES) } else { board.get_bitboard(BLACK_PIECES) };
    let mut mobility = 0;
    let mut attack_units = 0;

    let mut knights = board.get_bitboard(KNIGHTS) & own;
    while knights != 0
    {
        let sq = knights.trailing_zeros() as usize;
        let attacks = KNIGHT_ATTACK_MAP[sq] & !own;
        mobility += attacks.count_ones() as i32 * 4;
        attack_units += 2 * (attacks & enemy_king_zone).count_ones() as i32;
        knights &= knights - 1;
    }

    let mut bishops = board.get_bitboard(BISHOPS) & own;
    while bishops != 0
    {
        let sq = bishops.trailing_zeros() as usize;
        let attacks = magic_bitboards.get_bishop_attacks(sq, occupancy) & !own;
        mobility += attacks.count_ones() as i32 * 3;
        attack_units += 2 * (attacks & enemy_king_zone).count_ones() as i32;
        bishops &= bishops - 1;
    }

    let mut rooks = board.get_bitboard(ROOKS) & own;
    while rooks != 0
    {
        let sq = rooks.trailing_zeros() as usize;
        let attacks = magic_bitboards.get_rook_attacks(sq, occupancy) & !own;
        mobility += attacks.count_ones() as i32 * 2;
        attack_units += 3 * (attacks & enemy_king_zone).count_ones() as i32;
        rooks &= rooks - 1;
    }

    let mut queens = board.get_bitboard(QUEENS) & own;
    while queens != 0
    {
        let sq = queens.trailing_zeros() as usize;
        let attacks = (magic_bitboards.get_bishop_attacks(sq, occupancy) | magic_bitboards.get_rook_attacks(sq, occupancy)) & !own;
        mobility += attacks.count_ones() as i32;
        attack_units += 5 * (attacks & enemy_king_zone).count_ones() as i32;
        queens &= queens - 1;
    }

    (mobility, attack_units)
}

const KING_DANGER_TABLE: [i32; 32] = 
[
    0, 0, 1, 2, 4, 6, 9, 12, 16, 20, 25, 30, 36, 42, 49, 56,
    64, 72, 81, 90, 100, 110, 121, 132, 144, 156, 169, 182, 196, 210, 225, 240,
];

fn king_danger_penalty(attack_units: i32) -> i32
{
    let index = attack_units.clamp(0, KING_DANGER_TABLE.len() as i32 - 1) as usize;
    KING_DANGER_TABLE[index]
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
    const CONNECTED_BONUS: i32 = 15;

    let own = if white { board.get_bitboard(WHITE_PIECES) } else { board.get_bitboard(BLACK_PIECES) };
    let enemy_pawns = if white
    {
        board.get_bitboard(PAWNS) & board.get_bitboard(BLACK_PIECES)
    }
    else
    {
        board.get_bitboard(PAWNS) & board.get_bitboard(WHITE_PIECES)
    };

    let own_pawns = board.get_bitboard(PAWNS) & own;

    // First pass: identify every passed pawn and collect them into a bitboard.
    let mut passed_bb: u64 = 0;
    let mut pawns = own_pawns;

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
            passed_bb |= 1u64 << sq;
        }

        pawns &= pawns - 1;
    }

    let mut score = 0;
    let mut passed = passed_bb;

    while passed != 0
    {
        let sq = passed.trailing_zeros() as usize;
        let file = (sq % 8) as i32;
        let rank = (sq / 8) as i32;

        let advance = if white { rank } else { 7 - rank };
        score += PASSED_BONUS[advance as usize];

        for af in [file - 1, file + 1]
        {
            if af < 0 || af > 7 { continue; }
            for dr in -1..=1
            {
                let r2 = rank + dr;
                if r2 < 0 || r2 > 7 { continue; }
                let sq2 = (r2 * 8 + af) as usize;
                if passed_bb & (1u64 << sq2) != 0
                {
                    score += CONNECTED_BONUS;
                }
            }
        }

        passed &= passed - 1;
    }

    score
}

// Penalizes weak pawn structure for one side.
fn pawn_structure_eval(board: &Board, white: bool) -> i32
{
    let own_pawns = board.get_bitboard(PAWNS) & (if white { board.get_bitboard(WHITE_PIECES) } else { board.get_bitboard(BLACK_PIECES) });
    let mut penalty = 0;

    for file in 0..8usize
    {
        let file_mask: u64 = 0x0101_0101_0101_0101u64 << file;
        let pawns_on_file = (own_pawns & file_mask).count_ones() as i32;

        if pawns_on_file == 0 
        { 
            continue; 
        }

        // Doubled (or tripled) pawns: penalize every pawn beyond the first
        if pawns_on_file > 1
        {
            penalty += (pawns_on_file - 1) * 12;
        }

        // Isolated pawns: no friendly pawn on either neighboring file
        let mut adjacent_mask: u64 = 0;
        if file > 0 { adjacent_mask |= 0x0101_0101_0101_0101u64 << (file - 1); }
        if file < 7 { adjacent_mask |= 0x0101_0101_0101_0101u64 << (file + 1); }

        if own_pawns & adjacent_mask == 0
        {
            penalty += 15 * pawns_on_file;
        }
    }

    penalty
}

// Penalizes backward pawns
fn backward_pawn_eval(board: &Board, white: bool) -> i32
{
    let own = if white { board.get_bitboard(WHITE_PIECES) } else { board.get_bitboard(BLACK_PIECES) };
    let own_pawns = board.get_bitboard(PAWNS) & own;
    let enemy_pawns = if white
    {
        board.get_bitboard(PAWNS) & board.get_bitboard(BLACK_PIECES)
    }
    else
    {
        board.get_bitboard(PAWNS) & board.get_bitboard(WHITE_PIECES)
    };

    let mut penalty = 0;
    let mut pawns = own_pawns;

    while pawns != 0
    {
        let sq = pawns.trailing_zeros() as usize;
        let file = (sq % 8) as i32;
        let rank = (sq / 8) as i32;

        let mut adjacent_mask: u64 = 0;
        if file > 0 { adjacent_mask |= 0x0101_0101_0101_0101u64 << (file - 1); }
        if file < 7 { adjacent_mask |= 0x0101_0101_0101_0101u64 << (file + 1); }
        let adjacent_pawns = own_pawns & adjacent_mask;

        // No neighboring pawns at all — this is isolated, not backward; skip.
        if adjacent_pawns == 0
        {
            pawns &= pawns - 1;
            continue;
        }

        // Is there a friendly pawn on an adjacent file that is level with or behind this one
        let mut has_support = false;
        let mut check = adjacent_pawns;
        while check != 0
        {
            let other_sq = check.trailing_zeros() as usize;
            let other_rank = (other_sq / 8) as i32;
            let is_behind_or_level = if white { other_rank <= rank } else { other_rank >= rank };
            if is_behind_or_level
            {
                has_support = true;
                break;
            }
            check &= check - 1;
        }

        if !has_support
        {
            let stop_rank = if white { rank + 1 } else { rank - 1 };
            if stop_rank >= 0 && stop_rank <= 7
            {
                // Enemy pawns that would attack the stop square
                let attacker_rank = if white { stop_rank + 1 } else { stop_rank - 1 };
                if attacker_rank >= 0 && attacker_rank <= 7
                {
                    let mut attackers: u64 = 0;
                    if file > 0 { attackers |= 1u64 << ((attacker_rank * 8 + (file - 1)) as usize); }
                    if file < 7 { attackers |= 1u64 << ((attacker_rank * 8 + (file + 1)) as usize); }

                    if enemy_pawns & attackers != 0
                    {
                        penalty += 8;
                    }
                }
            }
        }

        pawns &= pawns - 1;
    }

    penalty
}