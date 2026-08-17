use crate::constants::*;

pub struct MagicBitBoards
{
    rook_attacks: Vec<u64>,
    bishop_attacks: Vec<u64>,
    rook_offsets: [usize; 64],
    bishop_offsets: [usize; 64],
    rook_masks: [u64; 64],
    bishop_masks: [u64; 64],
}

impl MagicBitBoards
{
    pub fn new() -> Self
    {
        let mut mb = MagicBitBoards
        {
            rook_attacks: vec![0; 102400],
            bishop_attacks: vec![0; 5248],
            rook_offsets: [0; 64],
            bishop_offsets: [0; 64],
            rook_masks: [0; 64], 
            bishop_masks: [0; 64],
        };
        mb.init();
        mb
    }

    pub fn init(&mut self)
    {
        let mut current_rook_offset = 0;
        let mut current_bishop_offset = 0;

        for square in 0..64
        {
            // Rook Init
            self.rook_offsets[square] = current_rook_offset;
            let rook_mask = calculate_rook_mask(square);
            self.rook_masks[square] = rook_mask;
            let rook_bit_count = rook_mask.count_ones();
            let rook_permutations = generate_permutations(rook_mask);

            for occupancy in rook_permutations 
            {
                // The Magic Hashing Formula
                let magic_index = (occupancy.wrapping_mul(ROOK_MAGICS[square])) >> (64 - rook_bit_count);
                
                // The Answer Key
                let attack_board = calculate_rook_attacks_raycaster(square, occupancy);
                
                // Store the answer at the specific offset + the hashed index
                self.rook_attacks[current_rook_offset + (magic_index as usize)] = attack_board;
            }
            current_rook_offset += 1 << rook_bit_count;

            // Bishop Init
            self.bishop_offsets[square] = current_bishop_offset;
            let bishop_mask = calculate_bishop_mask(square);
            self.bishop_masks[square] = bishop_mask;
            let bishop_bit_count = bishop_mask.count_ones();
            let bishop_permutations = generate_permutations(bishop_mask);

            for occupancy in bishop_permutations 
            {
                let magic_index = (occupancy.wrapping_mul(BISHOP_MAGICS[square])) >> (64 - bishop_bit_count);
                let attack_board = calculate_bishop_attacks_raycaster(square, occupancy);
                
                self.bishop_attacks[current_bishop_offset + (magic_index as usize)] = attack_board;
            }
            current_bishop_offset += 1 << bishop_bit_count;
        }
    }

    pub fn get_rook_attacks(&self, square: usize, occupancy: u64) -> u64
    {
        let mask = self.rook_masks[square];
        let blocked_occupancy = occupancy & mask;

        // Hash Formula
        let magic_index = (blocked_occupancy.wrapping_mul(ROOK_MAGICS[square])) >> (64 - mask.count_ones());

        // Offset Lookup
        let offset = self.rook_offsets[square];
        self.rook_attacks[offset + (magic_index as usize)]
    }

    pub fn get_bishop_attacks(&self, square: usize, occupancy: u64) -> u64
    {
        let mask = self.bishop_masks[square];
        let blocked_occupancy = occupancy & mask;
        
        // Hash Formula
        let magic_index = (blocked_occupancy.wrapping_mul(BISHOP_MAGICS[square])) >> (64 - mask.count_ones());

        // Offset Lookup
        let offset = self.bishop_offsets[square];
        self.bishop_attacks[offset + (magic_index as usize)]
    }
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