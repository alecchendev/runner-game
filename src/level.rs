use super::block::Block;
use super::utils::Vec3;

pub struct Level {
    block_data: Vec<f32>,
    win_block: Block,
    start_pos: Vec3,
}

impl Level {
    pub fn new(block_data: Vec<f32>, win_block: Block, start_pos: Vec3) -> Self {
        Self {
            block_data,
            win_block,
            start_pos
        }
    }

    pub fn block_data(&self) -> Vec<f32> {
        self.block_data.clone()
    }

    pub fn start_pos(&self) -> Vec3 {
        self.start_pos
    }

    pub fn win_block(&self) -> Block {
        self.win_block.clone()
    }
}