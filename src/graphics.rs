use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Graphics {
    positions: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<u32>,
}

#[wasm_bindgen]
impl Graphics {
    pub fn new() -> Self {
        Self {
            positions: vec![],
            colors: vec![],
            indices: vec![],
        }
    }

    pub fn update(&mut self, positions: Vec<f32>, colors: Vec<f32>, indices: Vec<u32>) {
        // update positions, colors, indices
        self.positions = positions;
        self.colors = colors;
        self.indices = indices;
    }

    pub fn positions(&self) -> Vec<f32> {
        self.positions.clone()
    }
    pub fn colors(&self) -> Vec<f32> {
        self.colors.clone()
    }
    pub fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
}