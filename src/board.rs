//Modules
use crate::movegen::Move;


// Array Index for respective BitBoard
const PAWNS: usize = 0;
const KNIGHTS: usize = 1;
const BISHOPS: usize = 2;
const ROOKS: usize = 3;
const QUEENS: usize = 4;
const KINGS: usize = 5;
const WHITE_PIECES: usize = 6;
const BLACK_PIECES: usize = 7;
const EMPTY_SQUARE: u8 = 8;

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

            //Updates Array of Pieces 
            self.array_of_pieces[move_data.get_start() as usize] = EMPTY_SQUARE;
            self.array_of_pieces[move_data.get_target() as usize] = move_data.get_piece() as u8;
        }
        //Creates a u64 BitMask that contains the start square and target square
        let move_mask: u64 = (1u64 << move_data.get_start()) | (1u64 << move_data.get_target());

        //Applies the bitmask on the respective bitboard
        self.bitboards_of_pieces[move_data.get_piece()] ^= move_mask;

        //Determines which color Bitboard to modify
        let color_index = if self.is_white_turn() {WHITE_PIECES} else {BLACK_PIECES};

        //Applies the bitmask on the respective bitboard
        self.bitboards_of_pieces[color_index] ^= move_mask;

        //Relinquishes turn
        self.turn_end();
    }

    pub fn unmake_move(&mut self, move_data: &Move)
    {
        
    }

    pub fn is_white_turn(&self) -> bool
    {
        self.white_to_move
    }

    pub fn turn_end(&mut self)
    {
        self.white_to_move = !self.white_to_move;
    }
}