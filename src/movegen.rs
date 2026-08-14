pub struct Move
{
    data: u32,
}

impl Move
{
    // Constructor to pack it
    pub fn new(start: u32, target: u32, flags: u32, piece: u32, captured_piece: u32) -> Self 
    {
        Move 
        {
            data: start | (target << 6) | (flags << 12) | (piece << 16) | (captured_piece << 20),
        }
    }

    // Methods to unpack it
    pub fn get_start(&self) -> u32 
    {
        self.data & 0x3F
    }

    pub fn get_target(&self) -> u32 
    {
        (self.data >> 6) & 0x3F
    }
    
    pub fn get_flags(&self) -> u32 
    {
        (self.data >> 12) & 0xF
    }

    pub fn get_piece(&self) -> usize 
    {
        ((self.data >> 16) & 0xF) as usize
    }

    pub fn get_captured_piece(&self) -> usize 
    {
        ((self.data >> 20) & 0xF) as usize
    }
}