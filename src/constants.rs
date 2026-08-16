pub const RANK_1: u64 = 0x00000000000000FF;
pub const RANK_2: u64 = 0x000000000000FF00;
pub const RANK_3: u64 = 0x0000000000FF0000;
pub const RANK_4: u64 = 0x00000000FF000000;
pub const RANK_5: u64 = 0x000000FF00000000;
pub const RANK_6: u64 = 0x0000FF0000000000;
pub const RANK_7: u64 = 0x00FF000000000000;
pub const RANK_8: u64 = 0xFF00000000000000;

pub const ZERO_A_FILE: u64 = 0xFEFEFEFEFEFEFEFE;
pub const ZERO_B_FILE: u64 = 0xFDFDFDFDFDFDFDFD;
pub const ZERO_C_FILE: u64 = 0xFBFBFBFBFBFBFBFB;
pub const ZERO_D_FILE: u64 = 0xF7F7F7F7F7F7F7F7;
pub const ZERO_E_FILE: u64 = 0xEFEFEFEFEFEFEFEF;
pub const ZERO_F_FILE: u64 = 0xDFDFDFDFDFDFDFDF;
pub const ZERO_G_FILE: u64 = 0xBFBFBFBFBFBFBFBF;
pub const ZERO_H_FILE: u64 = 0x7F7F7F7F7F7F7F7F;

pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_B: u64 = 0x0202020202020202;
pub const FILE_C: u64 = 0x0404040404040404;
pub const FILE_D: u64 = 0x0808080808080808;
pub const FILE_E: u64 = 0x1010101010101010;
pub const FILE_F: u64 = 0x2020202020202020;
pub const FILE_G: u64 = 0x4040404040404040;
pub const FILE_H: u64 = 0x8080808080808080;

pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 320;
pub const BISHOP_VALUE: i32 = 330;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;

pub const KNIGHT_ATTACK_MAP: [u64; 64] = calculate_knight_attack_map();
pub const KING_ATTACK_MAP: [u64; 64] = calculate_king_attack_map();

pub const CASTLING_RIGHTS_UPDATE_TABLE: [u8; 64] = [
    13, 15, 15, 15, 12, 15, 15, 14, // Rank 1 (A1, E1, H1)
    15, 15, 15, 15, 15, 15, 15, 15, // Rank 2
    15, 15, 15, 15, 15, 15, 15, 15, // Rank 3
    15, 15, 15, 15, 15, 15, 15, 15, // Rank 4
    15, 15, 15, 15, 15, 15, 15, 15, // Rank 5
    15, 15, 15, 15, 15, 15, 15, 15, // Rank 6
    15, 15, 15, 15, 15, 15, 15, 15, // Rank 7
    7,  15, 15, 15, 3,  15, 15, 11, // Rank 8 (A8, E8, H8)
];

pub const ROOK_MAGICS: [u64; 64] = [
    0x2080008020400016, // Square 0
    0x0040002000401004, // Square 1
    0x4480100020028008, // Square 2
    0x0480080004100180, // Square 3
    0x0e00042110420038, // Square 4
    0x0b00020881000c00, // Square 5
    0x0480010012000480, // Square 6
    0x8100054221000082, // Square 7
    0x0848800040008020, // Square 8
    0x10a1400050002008, // Square 9
    0x4400808020001000, // Square 10
    0x0d02803000480180, // Square 11
    0x2205001005000800, // Square 12
    0x1008012040100408, // Square 13
    0x0000800100800200, // Square 14
    0x0001002500004192, // Square 15
    0x0000208000400080, // Square 16
    0x0000484000201002, // Square 17
    0x4400808020001000, // Square 18
    0x0808008008100080, // Square 19
    0x2002020010080420, // Square 20
    0x0004008080040200, // Square 21
    0x0090808001000200, // Square 22
    0x0500620004009dc3, // Square 23
    0x1240084480008020, // Square 24
    0x40905000c0012000, // Square 25
    0x4520008080100020, // Square 26
    0x0002004200100822, // Square 27
    0x3808008180080400, // Square 28
    0x0004008080040200, // Square 29
    0x0001003100420004, // Square 30
    0x1018084600029403, // Square 31
    0x4080004000402008, // Square 32
    0x8040002800201000, // Square 33
    0x0190080020200401, // Square 34
    0x0000100080800800, // Square 35
    0x2205001005000800, // Square 36
    0x4000a04028011004, // Square 37
    0x4400100804000201, // Square 38
    0x8140004102001094, // Square 39
    0x0480004020004005, // Square 40
    0x00c1008040010028, // Square 41
    0x0000410020010010, // Square 42
    0x1200100100090020, // Square 43
    0x0000110008010004, // Square 44
    0x0002000510020008, // Square 45
    0x0901013002240008, // Square 46
    0x800010441182000b, // Square 47
    0x0506028435410200, // Square 48
    0x0000810040003100, // Square 49
    0x0010410420001100, // Square 50
    0x0800100008028280, // Square 51
    0x2004080104018080, // Square 52
    0x2821820084008080, // Square 53
    0x3304020108108400, // Square 54
    0x0000208044211200, // Square 55
    0x2211204090800509, // Square 56
    0x4004190080400221, // Square 57
    0x1c01000820021041, // Square 58
    0x90020010204028d6, // Square 59
    0x4282002049841002, // Square 60
    0x8801000208040001, // Square 61
    0x1020009022010804, // Square 62
    0x4020022489004c02, // Square 63
];

pub const BISHOP_MAGICS: [u64; 64] = [
    0x0450521000408101, // Square 0
    0x0108086820444040, // Square 1
    0x0088080850901100, // Square 2
    0x4034404081284010, // Square 3
    0x0002121001002400, // Square 4
    0x004208020a00c012, // Square 5
    0x2401040120080000, // Square 6
    0x0002028404018400, // Square 7
    0x2540410401820600, // Square 8
    0x2020020222040100, // Square 9
    0x0600180823202005, // Square 10
    0x6080040410800050, // Square 11
    0x0008440422410004, // Square 12
    0x6000020804040080, // Square 13
    0x08c8006202202005, // Square 14
    0x0000020042021040, // Square 15
    0x0070821b02080800, // Square 16
    0x1004001024280048, // Square 17
    0x0002040102040300, // Square 18
    0x0008004082850040, // Square 19
    0x8002000420210426, // Square 20
    0x8800200d00884000, // Square 21
    0x4064600488080802, // Square 22
    0x0000900240441004, // Square 23
    0x8446980432501002, // Square 24
    0x0402088402100400, // Square 25
    0x1824010210011020, // Square 26
    0x0800808028020002, // Square 27
    0x9005010090104000, // Square 28
    0x0800410102100200, // Square 29
    0x0008210082090100, // Square 30
    0x04020041202c0208, // Square 31
    0x09b0022080090840, // Square 32
    0x0022101000051108, // Square 33
    0x1484004400084401, // Square 34
    0x4481420081080080, // Square 35
    0x80110204000a0030, // Square 36
    0x88200c0110028800, // Square 37
    0x8021080204810120, // Square 38
    0x8100a22040020500, // Square 39
    0x004208020a00c012, // Square 40
    0x0404092108109000, // Square 41
    0x8441040022008404, // Square 42
    0x0041020212000400, // Square 43
    0x0018080104004040, // Square 44
    0x0002881008e06100, // Square 45
    0x0103080800828101, // Square 46
    0x0001082908404502, // Square 47
    0x2401040120080000, // Square 48
    0x020210420e500088, // Square 49
    0x2440048048088044, // Square 50
    0x0000020042021040, // Square 51
    0x01a4000460820000, // Square 52
    0x0102301091084202, // Square 53
    0x080460041400a200, // Square 54
    0x0108086820444040, // Square 55
    0x0002028404018400, // Square 56
    0x0000020042021040, // Square 57
    0xc000202202010410, // Square 58
    0x000a000000420200, // Square 59
    0x2104000041102480, // Square 60
    0x0449004011027080, // Square 61
    0x2540410401820600, // Square 62
    0x0450521000408101, // Square 63
];

pub const PAWN_PST: [i32; 64] = [
    // Rank 1 - Pawns can never be here
    0,   0,   0,   0,   0,   0,   0,   0,
    // Rank 2 - Negative center encourages pushing D and E pawns early
    5,  10,  10, -20, -20,  10,  10,   5,
    // Rank 3
    5,  -5, -10,   0,   0, -10,  -5,   5,
    // Rank 4
    0,   0,   0,  20,  20,   0,   0,   0,
    // Rank 5
    5,   5,  10,  25,  25,  10,   5,   5,
    // Rank 6 - Passed pawns become highly dangerous
    10,  10,  20,  30,  30,  20,  10,  10,
    // Rank 7 - About to promote, massive bonus
    50,  50,  50,  50,  50,  50,  50,  50,
    // Rank 8 - Pawns promote, handled by material logic
    0,   0,   0,   0,   0,   0,   0,   0,
];

pub const KNIGHT_PST: [i32; 64] = [
    // Rank 1 (A1 to H1) - Heavy penalties for back-rank/corners
    -50, -40, -30, -30, -30, -30, -40, -50,
    // Rank 2
    -40, -20,   0,   0,   0,   0, -20, -40,
    // Rank 3 - Knights start getting active
    -30,   0,  10,  15,  15,  10,   0, -30,
    // Rank 4 - Central outposts
    -30,   5,  15,  20,  20,  15,   5, -30,
    // Rank 5
    -30,   0,  15,  20,  20,  15,   0, -30,
    // Rank 6 - C6 is index 42 (+20 bonus!)
    -30,   5,  20,  15,  15,  20,   5, -30,
    // Rank 7
    -40, -20,   0,   5,   5,   0, -20, -40,
    // Rank 8 (A8 to H8) - Deep enemy territory corners
    -50, -40, -30, -30, -30, -30, -40, -50,
];

pub const KING_MIDGAME_PST: [i32; 64] = [
    // Rank 1 - G1 (+30) and C1 (+10) encourage castling
    20,  30,  10,   0,   0,  10,  30,  20,
    // Rank 2 - Pawn shield squares
    20,  20,   0,   0,   0,   0,  20,  20,
    // Rank 3
    -10, -20, -20, -20, -20, -20, -20, -10,
    // Rank 4 - Heavy penalty for King marching out early
    -20, -30, -30, -40, -40, -30, -30, -20,
    // Rank 5
    -30, -40, -40, -50, -50, -40, -40, -30,
    // Rank 6
    -30, -40, -40, -50, -50, -40, -40, -30,
    // Rank 7
    -30, -40, -40, -50, -50, -40, -40, -30,
    // Rank 8
    -30, -40, -40, -50, -50, -40, -40, -30,
];

pub const KING_ENDGAME_PST: [i32; 64] = [
    // Rank 1 (Corners are heavily penalized)
    -50, -30, -30, -30, -30, -30, -30, -50,
    // Rank 2
    -30, -10,   0,   0,   0,   0, -10, -30,
    // Rank 3
    -30,  -5,  20,  30,  30,  20,  -5, -30,
    // Rank 4 (Maximum central bonuses)
    -30,  -5,  30,  40,  40,  30,  -5, -30,
    // Rank 5
    -30,  -5,  30,  40,  40,  30,  -5, -30,
    // Rank 6
    -30,  -5,  20,  30,  30,  20,  -5, -30,
    // Rank 7
    -30, -10,   0,   0,   0,   0, -10, -30,
    // Rank 8
    -50, -30, -30, -30, -30, -30, -30, -50,
];

pub const BISHOP_PST: [i32; 64] = [
    // Rank 1 (A1 to H1)
    -20, -10, -10, -10, -10, -10, -10, -20,
    // Rank 2
    -10,   5,   0,   0,   0,   0,   5, -10,
    // Rank 3
    -10,  10,  10,  10,  10,  10,  10, -10,
    // Rank 4
    -10,   0,  10,  10,  10,  10,   0, -10,
    // Rank 5
    -10,   5,   5,  10,  10,   5,   5, -10,
    // Rank 6
    -10,   0,   5,  10,  10,   5,   0, -10,
    // Rank 7
    -10,   0,   0,   0,   0,   0,   0, -10,
    // Rank 8 (A8 to H8)
    -20, -10, -10, -10, -10, -10, -10, -20,
];

pub const ROOK_PST: [i32; 64] = [
    // Rank 1
    0,   0,   0,   5,   5,   0,   0,   0,
    // Rank 2
    -5,   0,   0,   0,   0,   0,   0,  -5,
    // Rank 3
    -5,   0,   0,   0,   0,   0,   0,  -5,
    // Rank 4
    -5,   0,   0,   0,   0,   0,   0,  -5,
    // Rank 5
    -5,   0,   0,   0,   0,   0,   0,  -5,
    // Rank 6
    -5,   0,   0,   0,   0,   0,   0,  -5,
    // Rank 7 (7th Rank Rook bonus)
    5,  10,  10,  10,  10,  10,  10,   5,
    // Rank 8
    0,   0,   0,   0,   0,   0,   0,   0,
];

pub const QUEEN_PST: [i32; 64] = [
    // Rank 1
    -20, -10, -10,  -5,  -5, -10, -10, -20,
    // Rank 2
    -10,   0,   5,   0,   0,   0,   0, -10,
    // Rank 3
    -10,   5,   5,   5,   5,   5,   0, -10,
    // Rank 4
    0,   0,   5,   5,   5,   5,   0,  -5,
    // Rank 5
    -5,   0,   5,   5,   5,   5,   0,  -5,
    // Rank 6
    -10,   0,   5,   5,   5,   5,   0, -10,
    // Rank 7
    -10,   0,   0,   0,   0,   0,   0, -10,
    // Rank 8
    -20, -10, -10,  -5,  -5, -10, -10, -20,
];

const fn calculate_knight_attack_map() -> [u64; 64] {
    let mut attack_map: [u64; 64] = [0u64; 64];
    let mut current_square: usize = 0;

    while current_square < 64 {
        let current_square_bitboard: u64 = 1u64 << current_square;
        let mut attack_bitboard: u64 = 0u64;

        attack_bitboard |= (current_square_bitboard & ZERO_H_FILE) << 17; //NE
        attack_bitboard |= (current_square_bitboard & ZERO_H_FILE & ZERO_G_FILE) << 10; //NE
        attack_bitboard |= (current_square_bitboard & ZERO_H_FILE & ZERO_G_FILE) >> 6; //SE
        attack_bitboard |= (current_square_bitboard & ZERO_H_FILE) >> 15; //SE
        attack_bitboard |= (current_square_bitboard & ZERO_A_FILE) >> 17; //SW
        attack_bitboard |= (current_square_bitboard & ZERO_A_FILE & ZERO_B_FILE) >> 10; //SW
        attack_bitboard |= (current_square_bitboard & ZERO_A_FILE & ZERO_B_FILE) << 6; //NW
        attack_bitboard |= (current_square_bitboard & ZERO_A_FILE) << 15; //NW

        attack_map[current_square] = attack_bitboard;
        current_square += 1;
    }
    attack_map
}

const fn calculate_king_attack_map() -> [u64; 64] {
    let mut attack_map: [u64; 64] = [0u64; 64];
    let mut current_square: usize = 0;

    while current_square < 64 {
        let current_square_bitboard: u64 = 1u64 << current_square;
        let mut attack_bitboard: u64 = 0u64;

        attack_bitboard |= current_square_bitboard << 8; //N
        attack_bitboard |= (current_square_bitboard & ZERO_H_FILE) << 9; //NE
        attack_bitboard |= (current_square_bitboard & ZERO_H_FILE) << 1; //E
        attack_bitboard |= (current_square_bitboard & ZERO_H_FILE) >> 7; //SE
        attack_bitboard |= (current_square_bitboard) >> 8; //S
        attack_bitboard |= (current_square_bitboard & ZERO_A_FILE) >> 9; //SW
        attack_bitboard |= (current_square_bitboard & ZERO_A_FILE) >> 1; //W
        attack_bitboard |= (current_square_bitboard & ZERO_A_FILE) << 7; //NW

        attack_map[current_square] = attack_bitboard;
        current_square += 1;
    }
    attack_map
}