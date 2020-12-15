

pub struct Universe {
    players: Vec<Player>,
    
    gravity: f32, // negative

    blocks: Vec<Block>,

    graphics: Graphics,
}

impl Universe {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn update(&mut self) {
        self.player.update(&self.blocks);
        self.graphics.update(&self.blocks);
    }
}

pub struct Graphics {
    positions: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<f32>,
}

impl Graphics {
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