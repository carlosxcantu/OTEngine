pub mod board;
pub mod movedata;
pub mod movegenerator;
pub mod constants;
pub mod magics;
pub mod perft;
pub mod evaluation;
pub mod search;
pub mod zobrist;
pub mod transposition;
pub mod uci;

use wasm_bindgen::prelude::*;
use crate::board::Board;
use crate::magics::MagicBitBoards;
use crate::zobrist::Zobrist;
use crate::movedata::Move;
use crate::transposition::TranspositionTable;

#[wasm_bindgen]
pub struct EngineBridge {
    board: Board,
    magic_bitboards: MagicBitBoards,
    zobrist: Zobrist,
    tt: TranspositionTable,
}

#[wasm_bindgen]
impl EngineBridge {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Initialize your engine components exactly like you do in uci_loop
        let magic_bitboards = MagicBitBoards::new();
        let zobrist = Zobrist::new();
        let board = Board::new(&zobrist);
        let tt = TranspositionTable::new(4_194_304);

        Self {
            board,
            magic_bitboards,
            zobrist,
            tt,
        }
    }

    pub fn send_command(&mut self, command: &str) -> String {
        let tokens: Vec<&str> = command.split_whitespace().collect();
        if tokens.is_empty() {
            return String::new();
        }

        match tokens[0] {
            "isready" => {
                String::from("readyok")
            }
            "ucinewgame" => {
                self.board = Board::new(&self.zobrist);
                self.tt = crate::transposition::TranspositionTable::new(4_194_304);
                String::from("readyok")
            }
            "position" => {
                let mut move_idx = 1;
                
                if tokens.get(1) == Some(&"startpos") {
                    self.board = Board::new(&self.zobrist);
                    if tokens.get(2) == Some(&"moves") {
                        move_idx = 3;
                    }
                } 
                
                // Replay the move history sent by the web UI
                for i in move_idx..tokens.len() {
                    let move_str = tokens[i];
                    if let Some(the_move) = crate::uci::parse_uci_move(move_str, &mut self.board, &self.magic_bitboards, &self.zobrist) {
                        self.board.make_move(&the_move, &self.zobrist);
                    }
                }
                String::new() // No response needed for position updates
            }
            "go" => {
                let mut time_limit: u64 = 1200; // Default fallback
                let mut time_left: Option<u64> = None;
                let mut increment: u64 = 0;
                let mut moves_to_go: u64 = 30; // Sudden death estimation

                // Parse the time parameters sent by the JavaScript UI
                for i in 1..tokens.len() {
                    if self.board.is_white_turn() {
                        if tokens[i] == "wtime" { time_left = tokens.get(i + 1).unwrap_or(&"0").parse::<u64>().ok(); }
                        if tokens[i] == "winc"  { increment = tokens.get(i + 1).unwrap_or(&"0").parse::<u64>().unwrap_or(0); }
                    } else {
                        if tokens[i] == "btime" { time_left = tokens.get(i + 1).unwrap_or(&"0").parse::<u64>().ok(); }
                        if tokens[i] == "binc"  { increment = tokens.get(i + 1).unwrap_or(&"0").parse::<u64>().unwrap_or(0); }
                    }
                    if tokens[i] == "movestogo" {
                        moves_to_go = tokens.get(i + 1).unwrap_or(&"30").parse::<u64>().unwrap_or(30);
                    }
                }

                // Apply your exact uci.rs time management logic
                if let Some(t) = time_left {
                    let base_time = t / moves_to_go.max(1);
                    let inc_bonus = (increment as f64 * 0.75) as u64;
                    time_limit = (base_time + inc_bonus).max(50); 
                    
                    if time_limit > t {
                        time_limit = (t as f64 * 0.8) as u64;
                    }
                }
                
                let best_move = crate::search::get_best_move(
                    &mut self.board, 
                    time_limit, 
                    &self.magic_bitboards, 
                    &self.zobrist, 
                    &mut self.tt
                );
                
                let move_str = crate::perft::get_algebraic_move(&best_move);
                format!("bestmove {}", move_str)
            }
            _ => String::new(),
        }
    }
}