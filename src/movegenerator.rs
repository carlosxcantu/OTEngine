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

pub fn calculate_rook_mask(square: usize) -> u64
{
    let mut bitboard = 0u64;
    let rank = (square / 8) as i32;
    let file = (square % 8) as i32;

    // North (Rank increases, stops at Rank 6)
    for r in (rank + 1)..=6 
    {
        bitboard |= 1u64 << (r * 8 + file);
    }
    
    for r in (1..rank).rev() 
    { 
        bitboard |= 1u64 << (r * 8 + file);
    }

    for f in (file + 1)..= 6 
    {
        bitboard |= 1u64 << (rank * 8 + f);
    }
    
    for f in (1..file).rev() 
    {
        bitboard |= 1u64 << (rank * 8 + f);
    }
    bitboard
}

pub fn calculate_bishop_mask(square: usize) -> u64 
{
    let mut bitboard = 0u64;
    let rank = (square / 8) as i32;
    let file = (square % 8) as i32;

    let mut r = rank + 1;
    let mut f = file + 1;
    while r <= 6 && f <= 6 
    {
        bitboard |= 1u64 << (r * 8 + f);
        r += 1;
        f += 1;
    }

    let mut r = rank - 1;
    let mut f = file + 1;
    while r >= 1 && f <= 6 
    {
        bitboard |= 1u64 << (r * 8 + f);
        r -= 1;
        f += 1;
    }

    let mut r = rank - 1;
    let mut f = file - 1;
    while r >= 1 && f >= 1 
    {
        bitboard |= 1u64 << (r * 8 + f);
        r -= 1;
        f -= 1;
    }

    let mut r = rank + 1;
    let mut f = file - 1;
    while r <= 6 && f >= 1 
    {
        bitboard |= 1u64 << (r * 8 + f);
        r += 1;
        f -= 1;
    }

    bitboard
}

pub fn generate_permutations(mut mask: u64) -> Vec<u64> 
{
    let mut permutations: Vec<u64> = Vec::new();
    let mut square_indices: Vec<u32> = Vec::new();

    while mask != 0 
    {
        let square = mask.trailing_zeros();
        square_indices.push(square);
        mask &= mask - 1; 
    }

    let bit_count = square_indices.len();
    let total_permutations = 1 << bit_count;

    for i in 0..total_permutations 
    {
        let mut occupancy = 0u64;
        for j in 0..bit_count 
        {
            if (i & (1 << j)) != 0 
            {
                occupancy |= 1u64 << square_indices[j];
            }
        }
        permutations.push(occupancy);
    }
    permutations
}

pub fn calculate_rook_attacks_raycaster(square: usize, block: u64) -> u64 
{
    let mut attacks = 0u64;
    let rank = (square / 8) as i32;
    let file = (square % 8) as i32;

    // North
    for r in (rank + 1)..=7 
    {
        let sq = r * 8 + file;
        attacks |= 1u64 << sq;
        if (block & (1u64 << sq)) != 0 { break; }
    }
    
    // South
    for r in (0..rank).rev() 
    {
        let sq = r * 8 + file;
        attacks |= 1u64 << sq;
        if (block & (1u64 << sq)) != 0 { break; }
    }
    
    // East
    for f in (file + 1)..=7 
    {
        let sq = rank * 8 + f;
        attacks |= 1u64 << sq;
        if (block & (1u64 << sq)) != 0 { break; }
    }
    
    // West
    for f in (0..file).rev() 
    {
        let sq = rank * 8 + f;
        attacks |= 1u64 << sq;
        if (block & (1u64 << sq)) != 0 { break; }
    }

    attacks
}

pub fn calculate_bishop_attacks_raycaster(square: usize, block: u64) -> u64 
{
    let mut attacks = 0u64;
    let rank = (square / 8) as i32;
    let file = (square % 8) as i32;

    // North-East
    let mut r = rank + 1;
    let mut f = file + 1;
    while r <= 7 && f <= 7 
    {
        let sq = r * 8 + f;
        attacks |= 1u64 << sq;
        if (block & (1u64 << sq)) != 0 { break; }
        r += 1;
        f += 1;
    }

    // South-East
    let mut r = rank - 1;
    let mut f = file + 1;
    while r >= 0 && f <= 7 
    {
        let sq = r * 8 + f;
        attacks |= 1u64 << sq;
        if (block & (1u64 << sq)) != 0 { break; }
        r -= 1;
        f += 1;
    }

    // South-West
    let mut r = rank - 1;
    let mut f = file - 1;
    while r >= 0 && f >= 0 
    {
        let sq = r * 8 + f;
        attacks |= 1u64 << sq;
        if (block & (1u64 << sq)) != 0 { break; }
        r -= 1;
        f -= 1;
    }

    // North-West
    let mut r = rank + 1;
    let mut f = file - 1;
    while r <= 7 && f >= 0 
    {
        let sq = r * 8 + f;
        attacks |= 1u64 << sq;
        if (block & (1u64 << sq)) != 0 { break; }
        r += 1;
        f -= 1;
    }
    attacks
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

//Offline Script for to generate Hash Table for Magic Bitboards
pub struct PRNG { seed: u64 }
impl PRNG {
    pub fn new() -> Self { PRNG { seed: 0x98f107 | 1 } }
    pub fn random_u64(&mut self) -> u64 {
        let mut x = self.seed;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.seed = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    // The trick to making it solve in seconds instead of hours
    pub fn random_sparse_u64(&mut self) -> u64 {
        self.random_u64() & self.random_u64() & self.random_u64()
    }
}

pub fn find_magic_number(square: usize, bit_count: u32, is_bishop: bool) -> u64 
{
    // 1. Get the mask and permutations
    let mask = if is_bishop { calculate_bishop_mask(square) } else { calculate_rook_mask(square) };
    let occupancies = generate_permutations(mask);
    let mut true_attacks = vec![0u64; occupancies.len()];

    // 2. Generate the "Answer Key" using your slow raycasters
    for i in 0..occupancies.len() {
        true_attacks[i] = if is_bishop {
            calculate_bishop_attacks_raycaster(square, occupancies[i])
        } else {
            calculate_rook_attacks_raycaster(square, occupancies[i])
        };
    }

    let mut prng = PRNG::new();
    let mut used_attacks = vec![0u64; 4096]; // Max size needed is 4096 (2^12 for Rook central squares)

    // 3. The Brute Force Loop
    loop {
        let magic = prng.random_sparse_u64();
        
        // Skip numbers that don't have enough weight to scramble the bits
        if (mask.wrapping_mul(magic) & 0xFF00000000000000).count_ones() < 6 { continue; }

        // Clear the test array for the new magic number
        used_attacks.fill(0);
        let mut failed = false;

        // 4. Test the magic number against every single permutation
        for i in 0..occupancies.len() {
            // The Magic Hashing Formula!
            let magic_index = (occupancies[i].wrapping_mul(magic)) >> (64 - bit_count);
            
            let attack_key = true_attacks[i];

            if used_attacks[magic_index as usize] == 0 {
                // The slot is empty! Store the answer here.
                used_attacks[magic_index as usize] = attack_key;
            } else if used_attacks[magic_index as usize] != attack_key {
                // COLLISION! Two different attack boards tried to share the same index.
                failed = true;
                break;
            }
        }

        // 5. If we tested all permutations without a collision, we found it!
        if !failed {
            return magic;
        }
    }
}