//Modules
use crate::{constants::{self, CASTLING_RIGHTS_UPDATE_TABLE}, movedata::Move};

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
    bitboards_of_pieces: [u64; 8],
    array_of_pieces: [u8; 64],
    white_to_move: bool,
    castling_rights: u8, // A 4-bit flag to track who can castle
    en_passant_target: u64, // The square a pawn just skipped over
    history: Vec<GameState>
}

impl Board
{
    pub fn new() -> Self 
    {
        let mut starting_array: [u8; 64] = [EMPTY_SQUARE; 64];

        // Starting Position
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

        Self 
        {
            bitboards_of_pieces: starting_bitboards,
            array_of_pieces: starting_array,
            white_to_move: true,        // White always moves first
            castling_rights: 15,        // 15 is binary 1111 (All 4 castling rights available)
            en_passant_target: 0,       // No en passant target on turn 1
            history: Vec::with_capacity(512)
        }
    }

    pub fn make_move(&mut self, move_data: &Move)
    {
        self.history.push(GameState
        {
            castling_rights: self.castling_rights,
            en_passant_target: self.en_passant_target,
        });

        self.update_en_passant_target(move_data);
        self.update_castling_rights(move_data);

        // Capture Detector
        if self.array_of_pieces[move_data.get_target() as usize] != EMPTY_SQUARE
        {
            let target_mask: u64 = 1u64 << move_data.get_target();
            self.bitboards_of_pieces[self.array_of_pieces[move_data.get_target() as usize] as usize] &= !target_mask;

            if self.is_white_turn()
            {
                self.bitboards_of_pieces[BLACK_PIECES] &= !target_mask;
            }
            else
            {
                self.bitboards_of_pieces[WHITE_PIECES] &= !target_mask;
            }
        }

        let move_mask: u64 = (1u64 << move_data.get_start()) | (1u64 << move_data.get_target());
        self.bitboards_of_pieces[move_data.get_piece()] ^= move_mask;
        let color_index = if self.is_white_turn() {WHITE_PIECES} else {BLACK_PIECES};
        self.bitboards_of_pieces[color_index] ^= move_mask;
        self.array_of_pieces[move_data.get_start() as usize] = EMPTY_SQUARE;
        self.array_of_pieces[move_data.get_target() as usize] = move_data.get_piece() as u8;
        let flag = move_data.get_flags();

        //Flags for Special Moves
        if flag >= FLAG_PROMOTE_QUEEN && flag <= FLAG_PROMOTE_KNIGHT 
        {
            let target_square = move_data.get_target() as usize;
            let target_mask = 1u64 << target_square;
            self.bitboards_of_pieces[PAWNS] &= !target_mask;

            let promoted_piece = match flag 
            {
                FLAG_PROMOTE_QUEEN => QUEENS,
                FLAG_PROMOTE_ROOK => ROOKS,
                FLAG_PROMOTE_BISHOP => BISHOPS,
                FLAG_PROMOTE_KNIGHT => KNIGHTS,
                _ => unreachable!(),
            };

            self.bitboards_of_pieces[promoted_piece] |= target_mask;
            self.array_of_pieces[target_square] = promoted_piece as u8;
        }

        if flag == FLAG_EN_PASSANT 
        {
            let target_square = move_data.get_target() as usize;
            let captured_square = if self.is_white_turn() { target_square - 8 } else { target_square + 8 };
            let captured_mask = 1u64 << captured_square;
            self.bitboards_of_pieces[PAWNS] &= !captured_mask;

            if self.is_white_turn() 
            {
                self.bitboards_of_pieces[BLACK_PIECES] &= !captured_mask;
            } 
            else 
            {
                self.bitboards_of_pieces[WHITE_PIECES] &= !captured_mask;
            }

            self.array_of_pieces[captured_square] = EMPTY_SQUARE;
        }

        if flag == FLAG_KING_CASTLE || flag == FLAG_QUEEN_CASTLE 
        {
            let target_square = move_data.get_target() as usize;
            let (rook_start, rook_target) = if flag == FLAG_KING_CASTLE 
            {
                (target_square + 1, target_square - 1)
            } 
            else 
            {
                (target_square - 2, target_square + 1)
            };

            let rook_mask = (1u64 << rook_start) | (1u64 << rook_target);
            self.bitboards_of_pieces[ROOKS] ^= rook_mask;

            if self.is_white_turn() 
            {
                self.bitboards_of_pieces[WHITE_PIECES] ^= rook_mask;
            } 
            else 
            {
                self.bitboards_of_pieces[BLACK_PIECES] ^= rook_mask;
            }

            self.array_of_pieces[rook_start] = EMPTY_SQUARE;
            self.array_of_pieces[rook_target] = ROOKS as u8;
        }

        self.turn_end();
    }

    pub fn unmake_move(&mut self, move_data: &Move)
    {
        self.turn_end();

        if let Some(previous_state) = self.history.pop() 
        {
            self.castling_rights = previous_state.castling_rights;
            self.en_passant_target = previous_state.en_passant_target;
        }

        let flag = move_data.get_flags();
        let start_square = move_data.get_start() as usize;
        let target_square = move_data.get_target() as usize;
        let move_mask: u64 = (1u64 << target_square) | (1u64 << start_square);
        let color_index = if self.is_white_turn() { WHITE_PIECES } else { BLACK_PIECES };

        self.bitboards_of_pieces[color_index] ^= move_mask;

        if flag >= FLAG_PROMOTE_QUEEN && flag <= FLAG_PROMOTE_KNIGHT 
        {
            let target_mask = 1u64 << target_square;
            let start_mask = 1u64 << start_square;
            let promoted_piece = match flag 
            {
                FLAG_PROMOTE_QUEEN => QUEENS,
                FLAG_PROMOTE_ROOK => ROOKS,
                FLAG_PROMOTE_BISHOP => BISHOPS,
                FLAG_PROMOTE_KNIGHT => KNIGHTS,
                _ => unreachable!(),
            };

            self.bitboards_of_pieces[promoted_piece] &= !target_mask;
            self.bitboards_of_pieces[PAWNS] |= start_mask;
        } 
        else 
        {
            self.bitboards_of_pieces[move_data.get_piece()] ^= move_mask;
        }

        self.array_of_pieces[start_square] = move_data.get_piece() as u8;
        self.array_of_pieces[target_square] = EMPTY_SQUARE;

        if flag == FLAG_KING_CASTLE || flag == FLAG_QUEEN_CASTLE 
        {
            let (rook_start, rook_target) = if flag == FLAG_KING_CASTLE 
            {
                (target_square + 1, target_square - 1)
            } 
            else 
            {
                (target_square - 2, target_square + 1)
            };

            let rook_mask = (1u64 << rook_start) | (1u64 << rook_target);
            self.bitboards_of_pieces[ROOKS] ^= rook_mask;
            self.bitboards_of_pieces[color_index] ^= rook_mask;

            self.array_of_pieces[rook_target] = EMPTY_SQUARE;
            self.array_of_pieces[rook_start] = ROOKS as u8;
        }

        if flag == FLAG_EN_PASSANT 
        {
            let captured_square = if self.is_white_turn() { target_square - 8 } else { target_square + 8 };
            let captured_mask = 1u64 << captured_square;
            let enemy_color = if self.is_white_turn() { BLACK_PIECES } else { WHITE_PIECES };

            self.bitboards_of_pieces[PAWNS] |= captured_mask;
            self.bitboards_of_pieces[enemy_color] |= captured_mask;
            self.array_of_pieces[captured_square] = PAWNS as u8;
        } 
        else if move_data.get_captured_piece() as u8 != EMPTY_SQUARE 
        {
            let target_mask = 1u64 << target_square;
            let enemy_color = if self.is_white_turn() { BLACK_PIECES } else { WHITE_PIECES };

            self.bitboards_of_pieces[move_data.get_captured_piece()] |= target_mask;
            self.bitboards_of_pieces[enemy_color] |= target_mask;
            self.array_of_pieces[target_square] = move_data.get_captured_piece() as u8;
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

    pub fn get_en_passant_target(&self) -> u64
    {
        self.en_passant_target
    }

    pub fn get_castling_rights(&self) -> u8
    {
        self.castling_rights
    }

    fn update_en_passant_target(&mut self, move_data: &Move)
    {
        if move_data.get_flags() == FLAG_DOUBLE_PUSH 
        {
            let skipped_square = (move_data.get_start() + move_data.get_target()) / 2;
            self.en_passant_target = 1u64 << skipped_square;
        } 
        else 
        {
            self.en_passant_target = 0;
        }
    }

    fn update_castling_rights(&mut self, move_data: &Move)
    {
        self.castling_rights &= constants::CASTLING_RIGHTS_UPDATE_TABLE[move_data.get_start() as usize];
        self.castling_rights &= constants::CASTLING_RIGHTS_UPDATE_TABLE[move_data.get_target() as usize];
    }
}

pub struct GameState 
{
    pub castling_rights: u8,
    pub en_passant_target: u64,
}