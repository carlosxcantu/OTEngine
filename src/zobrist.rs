pub struct Zobrist 
{
    pub piece_keys: [[u64; 64]; 12], // 12 pieces (6 white, 6 black) x 64 squares
    pub castling_keys: [u64; 16],    // 16 possible castling right combinations (0-15)
    pub en_passant_keys: [u64; 8],   // 8 possible files for en passant
    pub side_to_move: u64,           // 1 key to flip when it's Black's turn
}

impl Zobrist 
{
    pub fn new() -> Self 
    {
        let mut prng = PRNG::new(0x98f107); 
        let mut z = Zobrist {
            piece_keys: [[0; 64]; 12],
            castling_keys: [0; 16],
            en_passant_keys: [0; 8],
            side_to_move: prng.random_u64(),
        };

        for piece in 0..12 {
            for square in 0..64 {
                z.piece_keys[piece][square] = prng.random_u64();
            }
        }

        for i in 0..16 {
            z.castling_keys[i] = prng.random_u64();
        }

        for i in 0..8 {
            z.en_passant_keys[i] = prng.random_u64();
        }

        z
    }
}

// Generates 64-bit keys
struct PRNG { seed: u64 }
impl PRNG {
    fn new(seed: u64) -> Self { PRNG { seed } }
    fn random_u64(&mut self) -> u64 {
        let mut x = self.seed;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.seed = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
}