//Modules
use crate::movedata::Move;

// Array Index for respective BitBoard
pub const PAWNS: usize = 0;
pub const KNIGHTS: usize = 1;
pub const BISHOPS: usize = 2;
pub const ROOKS: usize = 3;
pub const QUEENS: usize = 4;
pub const KINGS: usize = 5;
pub const WHITE_PIECES: usize = 6;
pub const BLACK_PIECES: usize = 7;
pub const EMPTY_SQUARE: u8 = 8;

pub const FLAG_NONE: u32 = 0;          
pub const FLAG_DOUBLE_PUSH: u32 = 1;   
pub const FLAG_KING_CASTLE: u32 = 2;   
pub const FLAG_QUEEN_CASTLE: u32 = 3; 
pub const FLAG_EN_PASSANT: u32 = 4;  
pub const FLAG_PROMOTE_QUEEN: u32 = 5; 
pub const FLAG_PROMOTE_ROOK: u32 = 6; 
pub const FLAG_PROMOTE_BISHOP: u32 = 7;
pub const FLAG_PROMOTE_KNIGHT: u32 = 8;

//Chess Board represented as a Struct of Bitboards
pub struct Board
{
    //Array of BitBoards
    bitboards_of_pieces: [u64; 8],

    //Array of Pieces positions
    array_of_pieces: [u8; 64],

    //Current turn
    white_to_move: bool,

    //BitBoards for Move Generation
    castling_rights: u8, // A 4-bit flag to track who can castle
    en_passant_target: u64, // The square a pawn just skipped over
}

impl Board
{
    pub fn new() -> Self {

        // 1. Initialize an empty array
        let mut starting_array: [u8; 64] = [EMPTY_SQUARE; 64];

        // 2. Set up the starting positions in the array
        // White Pieces
        starting_array[0] = ROOKS as u8;   // A1
        starting_array[1] = KNIGHTS as u8; // B1
        starting_array[2] = BISHOPS as u8; // C1
        starting_array[3] = QUEENS as u8;  // D1
        starting_array[4] = KINGS as u8;   // E1
        starting_array[5] = BISHOPS as u8; // F1
        starting_array[6] = KNIGHTS as u8; // G1
        starting_array[7] = ROOKS as u8;   // H1

        for i in 8..16 
        {
            starting_array[i] = PAWNS as u8; // Rank 2
        }

        // Black Pieces
        starting_array[56] = ROOKS as u8;   // A8
        starting_array[57] = KNIGHTS as u8; // B8
        starting_array[58] = BISHOPS as u8; // C8
        starting_array[59] = QUEENS as u8;  // D8
        starting_array[60] = KINGS as u8;   // E8
        starting_array[61] = BISHOPS as u8; // F8
        starting_array[62] = KNIGHTS as u8; // G8
        starting_array[63] = ROOKS as u8;   // H8
        for i in 48..56 {
            starting_array[i] = PAWNS as u8; // Rank 7
        }

        // 3. Construct the bitboards using standard starting hex values
        let starting_bitboards: [u64; 8] = 
        [
            0x00FF00000000FF00, // PAWNS: Rank 2 and 7
            0x4200000000000042, // KNIGHTS: B1, G1, B8, G8
            0x2400000000000024, // BISHOPS: C1, F1, C8, F8
            0x8100000000000081, // ROOKS: A1, H1, A8, H8
            0x0800000000000008, // QUEENS: D1, D8
            0x1000000000000010, // KINGS: E1, E8
            0x000000000000FFFF, // WHITE_PIECES: Rank 1 and 2
            0xFFFF000000000000, // BLACK_PIECES: Rank 7 and 8
        ];

        // 4. Return the fully instantiated struct
        Self 
        {
            bitboards_of_pieces: starting_bitboards,
            array_of_pieces: starting_array,
            white_to_move: true,        // White always moves first
            castling_rights: 15,        // 15 is binary 1111 (All 4 castling rights available)
            en_passant_target: 0,       // No en passant target on turn 1
        }
    }

    pub fn make_move(&mut self, move_data: &Move)
    {
        // capture detector
        if self.array_of_pieces[move_data.get_target() as usize] != EMPTY_SQUARE
        {
            // Creats a u64 Bitmask of the target square
            let target_mask: u64 = 1u64 << move_data.get_target();

            // Applies the Bitmask onto the respective Bitboard
            self.bitboards_of_pieces[self.array_of_pieces[move_data.get_target() as usize] as usize] &= !target_mask;

            // Applies the Bitmask to the bitboard of the captured color 
            if self.is_white_turn()
            {
                self.bitboards_of_pieces[BLACK_PIECES] &= !target_mask;
            }
            else
            {
                self.bitboards_of_pieces[WHITE_PIECES] &= !target_mask;
            }
        }
        //Creates a u64 BitMask that contains the start square and target square
        let move_mask: u64 = (1u64 << move_data.get_start()) | (1u64 << move_data.get_target());

        //Applies the bitmask on the respective bitboard
        self.bitboards_of_pieces[move_data.get_piece()] ^= move_mask;

        //Determines which color Bitboard to modify
        let color_index = if self.is_white_turn() {WHITE_PIECES} else {BLACK_PIECES};

        //Applies the bitmask on the respective bitboard
        self.bitboards_of_pieces[color_index] ^= move_mask;

        //Updates Array of Pieces 
        self.array_of_pieces[move_data.get_start() as usize] = EMPTY_SQUARE;
        self.array_of_pieces[move_data.get_target() as usize] = move_data.get_piece() as u8;

        //Relinquishes turn
        self.turn_end();
    }

    pub fn unmake_move(&mut self, move_data: &Move)
    {
        //Relinquishes turn
        self.turn_end();

        //Creates a u64 BitMask that contains the start square and target square
        let move_mask: u64 = (1u64 << move_data.get_target()) | (1u64 << move_data.get_start());

        //Applies the bitmask on the respective bitboard
        self.bitboards_of_pieces[move_data.get_piece()] ^= move_mask;

        //Determines which color Bitboard to modify
        let color_index = if self.is_white_turn() {WHITE_PIECES} else {BLACK_PIECES};

        //Applies the bitmask on the respective bitboard
        self.bitboards_of_pieces[color_index] ^= move_mask;

        //Updates Array of Pieces 
        self.array_of_pieces[move_data.get_start() as usize] = move_data.get_piece() as u8;
        self.array_of_pieces[move_data.get_target() as usize] = EMPTY_SQUARE;
        
        // capture detector
        if move_data.get_captured_piece() as u8 != EMPTY_SQUARE
        {
            // Creats a u64 Bitmask of the target square
            let target_mask: u64 = 1u64 << move_data.get_target();

            // Applies the Bitmask onto the respective Bitboard
            self.bitboards_of_pieces[move_data.get_captured_piece() as usize] |= target_mask;

            // Applies the Bitmask to the bitboard of the captured color 
            if self.is_white_turn()
            {
                self.bitboards_of_pieces[BLACK_PIECES] |= target_mask;
            }
            else
            {
                self.bitboards_of_pieces[WHITE_PIECES] |= target_mask;
            }

            //Updates Array of Pieces
            self.array_of_pieces[move_data.get_target() as usize] = move_data.get_captured_piece() as u8;
        }

    }

    pub fn is_white_turn(&self) -> bool
    {
        self.white_to_move
    }

    pub fn turn_end(&mut self)
    {
        self.white_to_move = !self.white_to_move;
    }

    pub fn get_bitboard(&self, bitboard: usize) -> u64
    {
        self.bitboards_of_pieces[bitboard]
    }

    pub fn get_piece_from_array(&self, target_square: u32) -> u32
    {
        self.array_of_pieces[target_square as usize] as u32
    }
}