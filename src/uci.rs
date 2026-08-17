use std::io::{self, BufRead};
use crate::
{
    board::{self, Board}, magics::MagicBitBoards, movedata::Move, movegenerator::{self, MoveList, generate_psuedo_legal_moves}, perft, search, transposition::TranspositionTable, zobrist::Zobrist,
};

const TIME_LIMIT: u64 = 1200;

pub fn uci_loop() 
{
    let magic_bitboards = MagicBitBoards::new();
    let zobrist = Zobrist::new();
    let mut board = Board::new(&zobrist);
    let mut tt = TranspositionTable::new(4_194_304); // 4 Million entries (~64MB)

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() 
    {
        let input = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        let command = input.trim();
        if command.is_empty() 
        {
            continue;
        }

        let tokens: Vec<&str> = command.split_whitespace().collect();
        
        match tokens[0] 
        {
            "uci" => 
            {
                println!("id name CustomRustEngine");
                println!("id author Student");
                println!("uciok");
            }
            "isready" => 
            {
                println!("readyok");
            }
            "ucinewgame" => 
            {
                board = Board::new(&zobrist);
                tt = TranspositionTable::new(4_194_304);
            }
            "position" => 
            {
                let mut move_idx = 1;
                
                if tokens.get(1) == Some(&"startpos") 
                {
                    board = Board::new(&zobrist);
                    if tokens.get(2) == Some(&"moves") 
                    {
                        move_idx = 3;
                    }
                } 
                else if tokens.get(1) == Some(&"fen") 
                {
                    // FEN Parsing
                    board = Board::new(&zobrist);
                    if let Some(pos) = tokens.iter().position(|&x| x == "moves") 
                    {
                        move_idx = pos + 1;
                    } 
                    else 
                    {
                        move_idx = tokens.len();
                    }
                }

                // Apply incoming moves sequentially
                for i in move_idx..tokens.len() 
                {
                    let move_str = tokens[i];
                    if let Some(the_move) = parse_uci_move(move_str, &mut board, &magic_bitboards, &zobrist) 
                    {
                        board.make_move(&the_move, &zobrist);
                    }
                }
            }
            "go" => 
            {
                let mut time_limit: u64 = TIME_LIMIT; // Default fallback
                let mut time_left: Option<u64> = None;
                let mut increment: u64 = 0;
                let mut moves_to_go: u64 = 30; // Estimate 30 moves left in sudden death

                for i in 1..tokens.len() 
                {
                    if board.is_white_turn() 
                    {
                        if tokens[i] == "wtime" { time_left = tokens.get(i + 1).unwrap_or(&"0").parse::<u64>().ok(); }
                        if tokens[i] == "winc"  { increment = tokens.get(i + 1).unwrap_or(&"0").parse::<u64>().unwrap_or(0); }
                    } 
                    else 
                    {
                        if tokens[i] == "btime" { time_left = tokens.get(i + 1).unwrap_or(&"0").parse::<u64>().ok(); }
                        if tokens[i] == "binc"  { increment = tokens.get(i + 1).unwrap_or(&"0").parse::<u64>().unwrap_or(0); }
                    }
                    
                    if tokens[i] == "movestogo" 
                    {
                        moves_to_go = tokens.get(i + 1).unwrap_or(&"30").parse::<u64>().unwrap_or(30);
                    }
                    if tokens[i] == "movetime" 
                    {
                        time_limit = tokens.get(i + 1).unwrap_or(&"1200").parse::<u64>().unwrap_or(TIME_LIMIT);
                        time_left = None; // Override dynamic math if exact time is forced
                    }
                }

                if let Some(t) = time_left 
                {
                    // Formula: (Time Left / expected moves) + (Increment * 75%)
                    let base_time = t / moves_to_go.max(1);
                    let inc_bonus = (increment as f64 * 0.75) as u64;
                    time_limit = (base_time + inc_bonus).max(50); // Minimum 50ms floor

                    // Safety Ceiling: Never use more than 80% of our total remaining time to prevent flagging
                    if time_limit > t 
                    {
                        time_limit = (t as f64 * 0.8) as u64;
                    }
                }

                let best_move = search::get_best_move(&mut board, time_limit, &magic_bitboards, &zobrist, &mut tt);
                let move_str = perft::get_algebraic_move(&best_move);
                
                println!("bestmove {}", move_str);
            }
            "quit" => 
            {
                break;
            }
            _ => {}
        }
    }
}

fn parse_uci_move(move_str: &str, board: &mut Board, magic_bitboards: &MagicBitBoards, zobrist: &Zobrist) -> Option<Move> 
{
    let mut move_list = MoveList::new();
    generate_psuedo_legal_moves(board, &mut move_list, magic_bitboards);

    for i in 0..move_list.get_count() 
    {
        let the_move = move_list.get_move(i);
        
        board.make_move(&the_move, zobrist);
        board.turn_end();
        let current_color = if board.is_white_turn() { board::WHITE_PIECES } else { board::BLACK_PIECES };
        let king_bitboard = board.get_bitboard(board::KINGS) & board.get_bitboard(current_color);
        let king_square = king_bitboard.trailing_zeros() as usize;
        let occupied_squares = board.get_bitboard(board::WHITE_PIECES) | board.get_bitboard(board::BLACK_PIECES);
        let is_in_check = movegenerator::is_square_attacked(king_square, occupied_squares, board, magic_bitboards);
        board.turn_end();
        let legal = !is_in_check;

        board.unmake_move(&the_move);

        if legal 
        {
            let algebraic = perft::get_algebraic_move(&the_move);
            if algebraic == move_str 
            {
                return Some(the_move);
            }
        }
    }
    None
}