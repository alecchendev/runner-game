

pub struct Universe {
    players: Vec<Player>,
    
    gravity: f32, // negative

    blocks: Vec<Block>,

    graphics: Graphics,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            players: vec![Player::new()],
            gravity: -0.015,
            blocks: vec![],
            graphics: Graphics::new(),
        }
    }

    pub fn update(&mut self) {
        self.player.update(&self.blocks, self.gravity);
        self.graphics.update(&self.blocks);
    }

    pub fn graphics(&self) -> Graphics {
        self.graphics.clone()
    }
}