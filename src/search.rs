use std::cmp::max;
use std::time::{Instant, Duration};
use crate::constants::PIECE_VALUES;
use crate::movegenerator::generate_tactical_moves;
use crate::transposition::{NodeType, TranspositionTable};
use crate::zobrist::{self, Zobrist};
use crate::{board::{self, Board}, evaluation::evaluation_board, magics::MagicBitBoards, movedata::Move, movegenerator::{MoveList, generate_psuedo_legal_moves, is_move_legal, is_square_attacked, static_exchange_evaluation}};

const MAX_SCORE: i32 = 50000;
const MIN_SCORE: i32 = -50000;

pub fn get_best_move(board: &mut Board, time_limit_ms: u64, magic_bitboards: &MagicBitBoards, zobrist: &Zobrist, tt: &mut TranspositionTable) -> Move 
{
    let mut best_overall_move = Move::new(0, 0, 0, 0, 0);
    let mut info = SearchInfo::new(time_limit_ms);

    for current_depth in 1..= 64 
    {
        let current_best_move = search_root(board, current_depth, magic_bitboards, &mut info, zobrist, tt);

        if info.stopped || Instant::now() >= info.end_time
        {
            break;
        }

        println!("info depth {} nodes {}", current_depth, info.nodes);
        best_overall_move = current_best_move;
    }
    
    best_overall_move
}

pub fn search_root(board: &mut Board, depth: u8, magic_bitboards: &MagicBitBoards, info: &mut SearchInfo, zobrist: &Zobrist, tt: &mut TranspositionTable) -> Move 
{
    let mut best_score = MIN_SCORE;
    let mut best_move = Move::new(0, 0, 0, 0, 0); 
    let mut alpha = MIN_SCORE;
    let beta = MAX_SCORE;
    let mut tt_move: Option<Move> = None;
    if let Some(entry) = tt.probe(board.zobrist_key) 
    {
        if entry.best_move.get_start() != 0 || entry.best_move.get_target() != 0 
        {
            tt_move = Some(entry.best_move);
        }
    }

    let k1 = info.killer_moves[0][depth as usize];
    let k2 = info.killer_moves[1][depth as usize];
    
    // Convert them to Option<Move> 
    let killer_1 = if k1.get_start() != 0 || k1.get_target() != 0 { Some(k1) } else { None };
    let killer_2 = if k2.get_start() != 0 || k2.get_target() != 0 { Some(k2) } else { None };

    let mut move_list = MoveList::new();
    generate_psuedo_legal_moves(board, &mut move_list, magic_bitboards);
    move_list.score_moves(board, tt_move, killer_1, killer_2, &info.history_moves);
    let mut legal_moves_played = 0;

    for i in 0..move_list.get_count() 
    {
        let the_move = move_list.pick_move(i);
        board.make_move(&the_move, zobrist);
        if is_move_legal(board, magic_bitboards) 
        {
            legal_moves_played += 1;
            let mut score = 0;

            if legal_moves_played == 1 
            {
                // 1. First move (PV): Search with Full Window
                best_move = the_move;
                best_score = MIN_SCORE;
                score = -minimax(board, depth - 1, -beta, -alpha, magic_bitboards, info, zobrist, tt, true, 1);
            } 
            else 
            {
                // 2. Subsequent moves: Search with Zero Window
                score = -minimax(board, depth - 1, -alpha - 1, -alpha, magic_bitboards, info, zobrist, tt, true, 1);
                
                // 3. PVS Fail-High: If the move beats alpha, re-search with Full Window
                if score > alpha 
                {
                    score = -minimax(board, depth - 1, -beta, -alpha, magic_bitboards, info, zobrist, tt, true, 1);
                }
            }

            board.unmake_move(&the_move);

            if info.stopped 
            {
                break;
            }

            if score > best_score 
            {
                best_score = score;
                best_move = the_move;
            }

            if score > alpha 
            {
                alpha = score;
            }
        }
        else 
        {
            board.unmake_move(&the_move);
        }
    }

    best_move
}

pub fn minimax(board: &mut Board, depth: u8, mut alpha: i32, beta: i32, magic_bitboards: &MagicBitBoards, info: &mut SearchInfo, zobrist: &Zobrist, tt: &mut TranspositionTable, allow_null: bool, ply: u8) -> i32
{
    info.nodes += 1;
    info.check_time();
    if info.stopped 
    { 
        return 0; 
    }

    if depth == 0
    {   
        return quiescence_search(board, alpha, beta, magic_bitboards, info, zobrist, ply);
    }

    // Draw detection: must happen before the TT probe. A repeated or 50-move
    // position is a draw regardless of what an earlier search path stored in
    // the TT for this zobrist key (graph-history-interaction safety).
    if ply > 0 && (board.is_repetition() || board.is_fifty_move_draw())
    {
        return 0;
    }

    let mut tt_move: Option<Move> = None;
    let original_alpha = alpha;
    if let Some(entry) = tt.probe(board.zobrist_key) 
    {
        if entry.best_move.get_start() != 0 || entry.best_move.get_target() != 0 
        {
            tt_move = Some(entry.best_move);
        }
        if entry.depth >= depth 
        {
            let mut tt_score = entry.score;
            if tt_score > 40000 { tt_score -= ply as i32; }
            if tt_score < -40000 { tt_score += ply as i32; }

            match entry.node_type 
            {
                NodeType::Exact => return tt_score,
                NodeType::Alpha => if tt_score <= alpha { return tt_score; },
                NodeType::Beta => if tt_score >= beta { return tt_score; },
            }
        }
    }

    let current_color = if board.is_white_turn() { board::WHITE_PIECES } else { board::BLACK_PIECES };
    let has_non_pawn_material = (board.get_bitboard(board::ROOKS) | 
                            board.get_bitboard(board::KNIGHTS) |
                            board.get_bitboard(board::BISHOPS) | 
                            board.get_bitboard(board::QUEENS)) & 
                            board.get_bitboard(current_color) != 0;

    let in_check = is_current_player_in_check(board, magic_bitboards);
    if allow_null && depth >= 3 && has_non_pawn_material && !in_check
    {
        board.make_null_move(zobrist);
        let null_score = -minimax(board, depth - 3, -beta, -beta + 1, magic_bitboards, info, zobrist, tt, false, ply + 1);
        board.unmake_null_move();

        if info.stopped 
        {
            return 0;
        }

        if null_score >= beta 
        {
            return beta; 
        }
    }

    let k1 = info.killer_moves[0][depth as usize];
    let k2 = info.killer_moves[1][depth as usize];
    let killer_1 = if k1.get_start() != 0 || k1.get_target() != 0 { Some(k1) } else { None };
    let killer_2 = if k2.get_start() != 0 || k2.get_target() != 0 { Some(k2) } else { None };
    let mut best_score = MIN_SCORE;
    let mut best_move = Move::new(0, 0, 0, 0, 0);
    let mut move_list = MoveList::new();
    generate_psuedo_legal_moves(board, &mut move_list, magic_bitboards);
    move_list.score_moves(board, tt_move, killer_1, killer_2, &info.history_moves);
    let mut legal_moves_played = 0;

    for i in 0..move_list.get_count() 
    {
        let the_move = move_list.pick_move(i);
        board.make_move(&the_move, zobrist);

        if is_move_legal(board, magic_bitboards) 
        {
            legal_moves_played += 1;
            let gives_check = is_current_player_in_check(board, magic_bitboards); // Evaluates opponent's king
            let is_capture = the_move.get_captured_piece() != board::EMPTY_SQUARE as usize;
            let is_promotion = the_move.get_flags() >= board::FLAG_PROMOTE_QUEEN && the_move.get_flags() <= board::FLAG_PROMOTE_KNIGHT;
            let is_quiet = !is_capture && !is_promotion;
            let is_killer = the_move == k1 || the_move == k2;
            let mut score = 0;
            let extension = if gives_check && ply < 20 { 1 } else { 0 };
            let new_depth = (depth - 1) + extension;

            if legal_moves_played == 1 
            {
                // 1. PV Move: Full Window Search
                score = -minimax(board, new_depth, -beta, -alpha, magic_bitboards, info, zobrist, tt, true, ply + 1);
            } 
            else 
            {
                // 2. Non-PV Moves: Zero Window Search
                let mut needs_full_search = true;

                if new_depth >= 3 && legal_moves_played >= 4 && is_quiet && !in_check && !gives_check && !is_killer 
                {
                    // LMR with Zero Window
                    score = -minimax(board, new_depth - 1, -alpha - 1, -alpha, magic_bitboards, info, zobrist, tt, true, ply + 1);
                    needs_full_search = score > alpha; // If it beats alpha, LMR failed, need full depth zero-window
                }

                if needs_full_search 
                {
                    // Normal Depth with Zero Window
                    score = -minimax(board, new_depth, -alpha - 1, -alpha, magic_bitboards, info, zobrist, tt, true, ply + 1);
                    
                    // 3. PVS Fail-High: If better than alpha but not a beta cutoff, Re-Search with Full Window
                    if score > alpha && score < beta 
                    {
                        score = -minimax(board, new_depth, -beta, -alpha, magic_bitboards, info, zobrist, tt, true, ply + 1);
                    }
                }
            }

            board.unmake_move(&the_move); 

            if info.stopped 
            {
                break;
            }

            if score > best_score 
            {
                best_score = score;
                best_move = the_move;
            }

            if score > alpha 
            {
                alpha = score;
            }

            if alpha >= beta
            {
                if is_quiet && the_move != info.killer_moves[0][depth as usize] 
                {
                    info.killer_moves[1][depth as usize] = info.killer_moves[0][depth as usize];
                    info.killer_moves[0][depth as usize] = the_move;
                    let color_offset = if board.is_white_turn() { 0 } else { 6 };
                    let piece = (the_move.get_piece() + color_offset) % 12;
                    let target = (the_move.get_target() as usize) & 63;
                    info.history_moves[piece][target] += (depth as i32) * (depth as i32);
                }
                break;
            }
        } 
        else 
        {
            board.unmake_move(&the_move);
        }
    }

    // Game Over Detection
    if legal_moves_played == 0
    {
        if in_check 
        {
            return -49000 + (ply as i32);
        } 
        else 
        {
            return 0; 
        }
    }
    
    if info.stopped 
    {
        return 0;
    }
    let node_type = if best_score <= original_alpha 
    {
        NodeType::Alpha
    } 
    else if best_score >= beta 
    {
        NodeType::Beta 
    } 
    else 
    {
        NodeType::Exact
    };

    let mut store_score = best_score;
    if store_score > 40000 { store_score += ply as i32; }
    if store_score < -40000 { store_score -= ply as i32; }
    tt.store(board.zobrist_key, depth, store_score, node_type, best_move);
    best_score
}

fn is_current_player_in_check(board: &Board, magic_bitboards: &MagicBitBoards) -> bool 
{
    let current_color = if board.is_white_turn() { board::WHITE_PIECES } else { board::BLACK_PIECES };
    let king_bitboard = board.get_bitboard(board::KINGS) & board.get_bitboard(current_color);

    if king_bitboard == 0 
    { 
        return true; 
    }

    let king_square = king_bitboard.trailing_zeros() as usize;
    let occupied_squares = board.get_bitboard(board::WHITE_PIECES) | board.get_bitboard(board::BLACK_PIECES);
    is_square_attacked(king_square, occupied_squares, board, magic_bitboards)
}

fn quiescence_search(board: &mut Board, mut alpha: i32, beta: i32, magic_bitboards: &MagicBitBoards, info: &mut SearchInfo, zobrist: &Zobrist, ply: u8) -> i32
{
    info.nodes += 1;
    info.check_time();
    if info.stopped { return 0; }

    let in_check = is_current_player_in_check(board, magic_bitboards);
    let mut pat: i32 = 0;
    if !in_check
    {
        pat = evaluation_board(board, magic_bitboards);
        if pat >= beta { return beta; }
        if pat > alpha { alpha = pat; }
    }

    let mut move_list = MoveList::new();
    
    // Generate ALL moves if in check to find evasions
    if in_check 
    {
        generate_psuedo_legal_moves(board, &mut move_list, magic_bitboards);
    } 
    else 
    {
        generate_tactical_moves(board, &mut move_list, magic_bitboards);
    }
    
    move_list.score_moves(board, None, None, None, &info.history_moves);
    let mut legal_moves = 0;
    const DELTA_MARGIN: i32 = 200;

    for i in 0..move_list.get_count() 
    {
        let the_move = move_list.pick_move(i);

        // Pruning only applies to quiet-search captures when not in check —
        // when in check we must consider every evasion, no matter how it scores.
        if !in_check
        {
            let captured = the_move.get_captured_piece();
            let is_capture = captured != board::EMPTY_SQUARE as usize;
            let is_promotion = the_move.get_flags() >= board::FLAG_PROMOTE_QUEEN && the_move.get_flags() <= board::FLAG_PROMOTE_KNIGHT;
            let is_en_passant = the_move.get_flags() == board::FLAG_EN_PASSANT;

            if is_capture && !is_promotion && !is_en_passant
            {
                // Delta pruning: even winning the captured piece outright can't reach alpha.
                if pat + PIECE_VALUES[captured] + DELTA_MARGIN < alpha
                {
                    continue;
                }

                // SEE pruning: skip captures that are a losing trade after all recaptures.
                let see_score = static_exchange_evaluation(board, the_move.get_target() as usize, the_move.get_start() as usize, the_move.get_piece(), magic_bitboards);
                if see_score < 0
                {
                    continue;
                }
            }
        }

        board.make_move(&the_move, zobrist);

        if is_move_legal(board, magic_bitboards) 
        {
            legal_moves += 1;
            let score = -quiescence_search(board, -beta, -alpha, magic_bitboards, info, zobrist, ply + 1);
            board.unmake_move(&the_move);

            if score >= beta 
            {
                return beta;
            }
            if score > alpha 
            {
                alpha = score;
            }
        } 
        else 
        {   
            board.unmake_move(&the_move);
        }
    }

    if in_check && legal_moves == 0 
    {
        return -49000 + (ply as i32);
    }
    alpha
}

pub struct SearchInfo 
{
    pub end_time: Instant,
    pub stopped: bool,
    pub nodes: u64,
    pub killer_moves: [[Move; 100]; 2],
    pub history_moves: [[i32; 64]; 12],
}

impl SearchInfo 
{
    pub fn new(time_limit_ms: u64) -> Self 
    {
        SearchInfo 
        {
            end_time: Instant::now() + Duration::from_millis(time_limit_ms),
            stopped: false,
            nodes: 0,
            killer_moves: [[Move::new(0, 0, 0, 0, 0); 100]; 2],
            history_moves: [[0; 64]; 12],
        }
    }

    pub fn check_time(&mut self) 
    {
        if self.nodes % 2048 == 0 && Instant::now() >= self.end_time 
        {
            self.stopped = true;
        }
    }
}