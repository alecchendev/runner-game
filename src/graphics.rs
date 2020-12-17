use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Graphics {
    positions: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<u32>,

    cam_pos: Vec<f32>,
    cam_theta: f32,
    cam_phi: f32,
}

#[wasm_bindgen]
impl Graphics {
    pub fn new() -> Self {
        Self {
            positions: vec![],
            colors: vec![],
            indices: vec![],
            cam_pos: vec![],
            cam_theta: 0.,
            cam_phi: 0.,
        }
    }

    pub fn update(&mut self, positions: Vec<f32>, colors: Vec<f32>, indices: Vec<u32>, cam_pos: Vec<f32>, cam_theta: f32, cam_phi: f32) {
        // update positions, colors, indices
        self.positions = positions;
        self.colors = colors;
        self.indices = indices;
        self.cam_pos = cam_pos;
        self.cam_theta = cam_theta;
        self.cam_phi = cam_phi;
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
    pub fn cam_pos(&self) -> Vec<f32> {
        self.cam_pos.clone()
    }
    pub fn cam_theta(&self) -> f32 {
        self.cam_theta
    }
    pub fn cam_phi(&self) -> f32 {
        self.cam_phi
    }
}