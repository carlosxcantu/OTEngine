use crate::movedata::Move;

// The Node Flags
#[derive(Copy, Clone, PartialEq)]
pub enum NodeType 
{
    Exact, 
    Alpha,
    Beta, 
}

// The Memory Slot
#[derive(Copy, Clone)]
pub struct TTEntry 
{
    pub zobrist_key: u64,
    pub depth: u8,
    pub score: i32,
    pub node_type: NodeType,
    pub best_move: Move,
}

// The Table
pub struct TranspositionTable 
{
    table: Vec<TTEntry>,
    size_mask: usize,
}

impl TranspositionTable 
{
    pub fn new(num_entries: usize) -> Self 
    {
        TranspositionTable 
        {
            table: vec![
                TTEntry 
                {
                    zobrist_key: 0,
                    depth: 0,
                    score: 0,
                    node_type: NodeType::Exact,
                    best_move: Move::new(0, 0, 0, 0, 0),
                };
                num_entries
            ],
            size_mask: num_entries - 1,
        }
    }

    pub fn store(&mut self, key: u64, depth: u8, score: i32, node_type: NodeType, best_move: Move) 
    {
        let index = (key as usize) & self.size_mask;
        let current_entry = self.table[index];
        if current_entry.zobrist_key != key || depth >= current_entry.depth 
        {
            self.table[index] = TTEntry 
            {
                zobrist_key: key,
                depth,
                score,
                node_type,
                best_move,
            };
        }
    }

    pub fn probe(&self, key: u64) -> Option<TTEntry> 
    {
        let index = (key as usize) & self.size_mask;
        let entry = self.table[index];

        if entry.zobrist_key == key 
        {
            Some(entry)
        } else 
        {
            None
        }
    }
}