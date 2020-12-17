#[derive(Clone)]
pub struct Graphics {
    positions: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<f32>,
}

impl Graphics {
    pub fn new(blocks, &Vec<Block>) -> Self {
        
    }

    pub fn update(&mut self, blocks: &Vec<Block>) {
        // update positions, colors, indices
    }
    pub fn positions(&self) -> Vec<f32> {

    }
    pub fn colors(&self) -> Vec<f32> {
        
    }
    pub fn indices(&self) -> Vec<f32> {
        
    }
}