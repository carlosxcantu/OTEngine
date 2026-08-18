//Modules
use crate::{constants::{self, CASTLING_RIGHTS_UPDATE_TABLE}, movedata::Move, zobrist::Zobrist};
use crate::constants::*;

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
    history: Vec<GameState>,
    pub zobrist_key: u64,
    pub incremental_eval: i32, // NEW
    pub phase_weight: i32,
    pub halfmove_clock: u32, // NEW: plies since last capture or pawn move
}

impl Board
{
    pub fn new(zobrist: &Zobrist) -> Self 
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

        let mut new_board = Self 
        {
            bitboards_of_pieces: starting_bitboards,
            array_of_pieces: starting_array,
            white_to_move: true,        
            castling_rights: 15,        
            en_passant_target: 0,       
            history: Vec::with_capacity(512),
            zobrist_key: 0,
            incremental_eval: 0, 
            phase_weight: 0,
            halfmove_clock: 0,
        };

        new_board.init_eval();
        new_board.zobrist_key = new_board.generate_zobrist_key(zobrist);
        new_board
    }

    pub fn init_eval(&mut self) 
    {
        self.incremental_eval = 0;
        self.phase_weight = 0;
        for square in 0..64 
        {
            let piece = self.array_of_pieces[square] as usize;
            if piece != EMPTY_SQUARE as usize && piece != KINGS as usize 
            {
                let is_white = (self.get_bitboard(WHITE_PIECES) & (1u64 << square)) != 0;
                self.incremental_eval += Self::get_piece_value_pst(piece, square, is_white);
                self.phase_weight += Self::get_piece_phase(piece);
            }
        }
    }

    pub fn make_move(&mut self, move_data: &Move, zobrist: &Zobrist)
    {
        self.history.push(GameState
        {
            castling_rights: self.castling_rights,
            en_passant_target: self.en_passant_target,
            zobrist_key: self.zobrist_key,
            incremental_eval: self.incremental_eval, 
            phase_weight: self.phase_weight,
            halfmove_clock: self.halfmove_clock,
        });

        // Hash OUT the old Castling Rights and En Passant file
        self.zobrist_key ^= zobrist.castling_keys[self.castling_rights as usize];
        if self.en_passant_target != 0 
        {
            let file = self.en_passant_target.trailing_zeros() as usize % 8;
            self.zobrist_key ^= zobrist.en_passant_keys[file];
        }

        self.update_en_passant_target(move_data);
        self.update_castling_rights(move_data);
        self.zobrist_key ^= zobrist.castling_keys[self.castling_rights as usize];
        if self.en_passant_target != 0 
        {
            let file = self.en_passant_target.trailing_zeros() as usize % 8;
            self.zobrist_key ^= zobrist.en_passant_keys[file];
        }

        let is_white = self.is_white_turn();
        let my_offset = if self.is_white_turn() { 0 } else { 6 };
        let enemy_offset = if self.is_white_turn() { 6 } else { 0 };
        let target_square = move_data.get_target() as usize;
        let start_square = move_data.get_start() as usize;
        let piece = move_data.get_piece();
        let flag = move_data.get_flags();
        let is_capture = self.array_of_pieces[target_square] != EMPTY_SQUARE;

        if is_capture
        {
            let captured_piece = self.array_of_pieces[target_square] as usize;
            self.zobrist_key ^= zobrist.piece_keys[captured_piece + enemy_offset][target_square];
            let target_mask: u64 = 1u64 << target_square;
            self.bitboards_of_pieces[captured_piece] &= !target_mask;
            if self.is_white_turn() 
            { 
                self.bitboards_of_pieces[BLACK_PIECES] &= !target_mask; 
            }
            else 
            { 
                self.bitboards_of_pieces[WHITE_PIECES] &= !target_mask; 
            }

            self.incremental_eval -= Self::get_piece_value_pst(captured_piece, target_square, !is_white);
            self.phase_weight -= Self::get_piece_phase(captured_piece);
        }

        self.zobrist_key ^= zobrist.piece_keys[piece + my_offset][start_square];
        self.zobrist_key ^= zobrist.piece_keys[piece + my_offset][target_square];

        let move_mask: u64 = (1u64 << start_square) | (1u64 << target_square);
        self.bitboards_of_pieces[piece] ^= move_mask;
        let color_index = if self.is_white_turn() {WHITE_PIECES} else {BLACK_PIECES};
        self.bitboards_of_pieces[color_index] ^= move_mask;
        self.array_of_pieces[start_square] = EMPTY_SQUARE;
        self.array_of_pieces[target_square] = piece as u8;

        if piece != KINGS as usize 
        {
            self.incremental_eval -= Self::get_piece_value_pst(piece, start_square, is_white);
            self.incremental_eval += Self::get_piece_value_pst(piece, target_square, is_white);
        }
        
        if flag >= FLAG_PROMOTE_QUEEN && flag <= FLAG_PROMOTE_KNIGHT 
        {
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

            self.zobrist_key ^= zobrist.piece_keys[PAWNS + my_offset][target_square];
            self.zobrist_key ^= zobrist.piece_keys[promoted_piece + my_offset][target_square];

            self.bitboards_of_pieces[promoted_piece] |= target_mask;
            self.array_of_pieces[target_square] = promoted_piece as u8;
            self.incremental_eval -= Self::get_piece_value_pst(PAWNS, target_square, is_white);
            self.incremental_eval += Self::get_piece_value_pst(promoted_piece, target_square, is_white);
            self.phase_weight += Self::get_piece_phase(promoted_piece);
        }

        if flag == FLAG_EN_PASSANT 
        {
            let captured_square = if self.is_white_turn() { target_square - 8 } else { target_square + 8 };
            let captured_mask = 1u64 << captured_square;
            self.bitboards_of_pieces[PAWNS] &= !captured_mask;
            self.zobrist_key ^= zobrist.piece_keys[PAWNS + enemy_offset][captured_square];

            if self.is_white_turn()
            { 
                self.bitboards_of_pieces[BLACK_PIECES] &= !captured_mask; 
            } 
            else 
            { 
                self.bitboards_of_pieces[WHITE_PIECES] &= !captured_mask; 
            }

            self.array_of_pieces[captured_square] = EMPTY_SQUARE;
            self.incremental_eval -= Self::get_piece_value_pst(PAWNS, captured_square, !is_white);
        }

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

            self.zobrist_key ^= zobrist.piece_keys[ROOKS + my_offset][rook_start];
            self.zobrist_key ^= zobrist.piece_keys[ROOKS + my_offset][rook_target];

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
            self.incremental_eval -= Self::get_piece_value_pst(ROOKS, rook_start, is_white);
            self.incremental_eval += Self::get_piece_value_pst(ROOKS, rook_target, is_white);
        }

        // Fifty-move / repetition clock: resets on any capture or pawn move (irreversible)
        if is_capture || piece == PAWNS as usize
        {
            self.halfmove_clock = 0;
        }
        else
        {
            self.halfmove_clock += 1;
        }

        self.turn_end();
        self.zobrist_key ^= zobrist.side_to_move;
    }

    pub fn unmake_move(&mut self, move_data: &Move)
    {
        self.turn_end();

        if let Some(previous_state) = self.history.pop() 
        {
            self.castling_rights = previous_state.castling_rights;
            self.en_passant_target = previous_state.en_passant_target;
            self.zobrist_key = previous_state.zobrist_key;
            self.incremental_eval = previous_state.incremental_eval; 
            self.phase_weight = previous_state.phase_weight;
            self.halfmove_clock = previous_state.halfmove_clock;
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

    pub fn make_null_move(&mut self, zobrist: &Zobrist) 
    {
        self.history.push(GameState 
        {
            castling_rights: self.castling_rights,
            en_passant_target: self.en_passant_target,
            zobrist_key: self.zobrist_key,
            incremental_eval: self.incremental_eval, 
            phase_weight: self.phase_weight,
            halfmove_clock: self.halfmove_clock,
        });

        if self.en_passant_target != 0 
        {
            let file = self.en_passant_target.trailing_zeros() as usize % 8;
            self.zobrist_key ^= zobrist.en_passant_keys[file];
            self.en_passant_target = 0;
        }

        self.turn_end();
        self.zobrist_key ^= zobrist.side_to_move;
    }

    pub fn unmake_null_move(&mut self) 
    {
        self.turn_end();
        
        // Restores Everything
        if let Some(previous_state) = self.history.pop() 
        {
            self.castling_rights = previous_state.castling_rights;
            self.en_passant_target = previous_state.en_passant_target;
            self.zobrist_key = previous_state.zobrist_key;
            self.incremental_eval = previous_state.incremental_eval; 
            self.phase_weight = previous_state.phase_weight;
            self.halfmove_clock = previous_state.halfmove_clock;
        }
    }

    // Returns true if the current position's zobrist key has occurred earlier
    // within the current run of reversible moves (i.e. since the last capture
    // or pawn move). One repeated occurrence is treated as a draw for search
    // purposes — this is the standard, conservative approach engines use to
    // avoid the graph-history-interaction problem with the TT.
    pub fn is_repetition(&self) -> bool
    {
        let limit = self.halfmove_clock as usize;
        let len = self.history.len();
        if limit == 0 || len == 0 
        { 
            return false; 
        }

        let start = if len > limit { len - limit } else { 0 };

        for i in (start..len).rev()
        {
            if self.history[i].zobrist_key == self.zobrist_key 
            {
                return true;
            }
        }
        false
    }

    pub fn is_fifty_move_draw(&self) -> bool
    {
        self.halfmove_clock >= 100
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

    pub fn get_piece_value_pst(piece: usize, sq: usize, is_white: bool) -> i32 
    {
        let sq_idx = if is_white { sq } else { sq ^ 56 };
        let val = match piece 
        {
            PAWNS => PAWN_VALUE + PAWN_PST[sq_idx],
            KNIGHTS => KNIGHT_VALUE + KNIGHT_PST[sq_idx],
            BISHOPS => BISHOP_VALUE + BISHOP_PST[sq_idx],
            ROOKS => ROOK_VALUE + ROOK_PST[sq_idx],
            QUEENS => QUEEN_VALUE + QUEEN_PST[sq_idx],
            _ => 0,
        };
        if is_white { val } else { -val }
    }

    pub fn get_piece_phase(piece: usize) -> i32 
    {
        match piece 
        {
            KNIGHTS | BISHOPS => 1,
            ROOKS => 2,
            QUEENS => 4,
            _ => 0,
        }
    }

    pub fn generate_zobrist_key(&self, zobrist: &Zobrist) -> u64 
    {
        let mut final_key = 0;
        
        // Hash the pieces
        for square in 0..64 
        {
            let piece = self.array_of_pieces[square] as usize;
            if piece != EMPTY_SQUARE as usize 
            {
                let color_offset = if (self.get_bitboard(WHITE_PIECES) & (1u64 << square)) != 0 { 0 } else { 6 };
                final_key ^= zobrist.piece_keys[piece + color_offset][square];
            }
        }

        // Hash castling rights (0 to 15)
        final_key ^= zobrist.castling_keys[self.castling_rights as usize];

        // Hash En Passant target (if any)
        if self.en_passant_target != 0 
        {
            let file = self.en_passant_target.trailing_zeros() as usize % 8;
            final_key ^= zobrist.en_passant_keys[file];
        }

        // Hash Side to Move
        if !self.white_to_move 
        {
            final_key ^= zobrist.side_to_move;
        }

        final_key
    }
}

pub struct GameState 
{
    pub castling_rights: u8,
    pub en_passant_target: u64,
    pub zobrist_key: u64,
    pub incremental_eval: i32,
    pub phase_weight: i32,
    pub halfmove_clock: u32,
}