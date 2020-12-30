use wasm_bindgen::prelude::*;
use super::utils::Vec3;
use super::player::{Player, Go};

use super::block::Block;
use super::graphics::Graphics;

use super::log;

#[wasm_bindgen]
pub enum Input {
    GoLeft = 0,
    GoForward = 1,
    GoRight = 2,
    GoBack = 3,
    StopLeft = 4,
    StopForward = 5,
    StopRight = 6,
    StopBack = 7,
    Jump = 8,
    Cast = 9,
    Pull = 10,
    Release = 11,
}

#[wasm_bindgen]
pub struct Universe {
    players: Vec<Player>,
    
    gravity: f32, // negative

    blocks: Vec<Block>,

    graphics: Graphics,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let block_data = [
            -10., -1., -10., 20., 1., 25.,

            -10., 0., 10., 20., 5., 2.,
            -4., 0., 6., 5., 1.5, 4.,
            -3.25, 1.5, 7., 3., 1.5, 3.,
            -2.25, 3., 8., 1.5, 1., 2.,

            -10., 2., 25., 20., 1., 40.,
            -5., 11., 18., 10., 1., 8.,

            -7., 4., 35., 3., 0.3, 3.,
            -7., 5.5, 39.5, 3., 0.3, 3.,
            -7., 9., 47., 3., 3., 0.3,
            -7., 13., 49.5, 3., 0.3, 3.,

            -1.5, 20., 55., 3., 0.3, 3.,

            4.5, 15., 60., 3., 0.3, 6.,
        ];
        let mut blocks = vec![];
        for i in 0..(block_data.len() / 6) {
            let start = i * 6;
            let origin = Vec3::new(block_data[start], block_data[start + 1], block_data[start + 2]);
            let dims = Vec3::new(block_data[start + 3], block_data[start + 4], block_data[start + 5]);
            blocks.push(Block::new(origin, dims));
        }
        Self {
            players: vec![Player::new()],//, Player::new()],
            gravity: -0.01,
            blocks,
            graphics: Graphics::new(),
        }
    }

    pub fn update(&mut self, curr_player: usize) {
        for player in &mut self.players {
            player.update(&self.blocks, self.gravity);
        }
        self.update_graphics(curr_player);
    }

    fn update_graphics(&mut self, curr_player: usize) {
        let mut positions = vec![];
        let mut colors = vec![];
        let mut indices = vec![];

        let mut index = 0;
        for block in &self.blocks {
            positions.append(&mut Self::get_block_vertices(&block.origin, &block.dims));
            indices.append(&mut Self::get_block_indices(&mut index));
        }

        let mut floor_colors = vec![
            0.5,  0.5,  0.5,  1.0,
            0.5,  0.5,  0.5,  1.0,
            0.5,  0.5,  0.5,  1.0,
            0.5,  0.5,  0.5,  1.0,
            0.5,  0.5,  0.5,  1.0,
            0.5,  0.5,  0.5,  1.0,
        ];

        colors.append(&mut floor_colors);

        let color_pattern = vec![
            1.0,  0.0,  0.0,  1.0,
            0.0,  1.0,  0.0,  1.0,
            0.0,  0.0,  1.0,  1.0,
            1.0,  1.0,  0.0,  1.0,
            1.0,  0.0,  1.0,  1.0,
            0.0,  1.0,  1.0,  1.0,
        ];
        for block in 0..(self.blocks.len() - 1) {
            for index in 0..color_pattern.len() {
                colors.push(color_pattern[(block * 4 + index) % color_pattern.len()]);
            }
        }

        let mut player_index = 0;
        for player in &self.players {
            // PLAYER VIS
            if player_index != curr_player {
                positions.append(&mut Self::get_block_vertices(&(player.position - player.dims / 2.), &player.dims));
                indices.append(&mut Self::get_block_indices(&mut index));
                for index in 0..color_pattern.len() {
                    colors.push(color_pattern[(player_index * 4 + index) % color_pattern.len()]);
                }
            }
            player_index += 1;

            // GRAPPLE VIS
            if let Some(grapple) = &player.grapple {
                let grapple_width = 0.005;
                let h_dir = Vec3::new(player.theta().cos(), 0., -player.theta().sin());
                let start = player.position;
                let end = grapple.end;
                
                positions.append(&mut (start + h_dir * grapple_width).to_vec());
                positions.append(&mut (start - h_dir * grapple_width).to_vec());
                positions.append(&mut (end - h_dir * grapple_width).to_vec());
                positions.append(&mut (end + h_dir * grapple_width).to_vec());
                
                let mut new_indices = vec![0, 1, 2, 0, 2, 3];
                for new_index in &mut new_indices {
                    *new_index += index;
                }
                indices.append(&mut new_indices);
                index += 4;
    
                colors.append(&mut vec![1.0, 1.0, 1.0, 1.0]);
            }
        }
        
        let cam_pos = self.players[curr_player].position + Vec3::new(0., self.players[curr_player].dims.y / 8., 0.);

        self.graphics.update(positions, colors, indices, cam_pos.to_vec(), self.players[curr_player].theta(), -self.players[curr_player].phi());
    }

    fn get_block_vertices(origin: &Vec3, dims: &Vec3) -> Vec<f32> {
        let vertices = vec![
            *origin,
            *origin + Vec3::new(dims.x, 0., 0.),
            *origin + Vec3::new(dims.x, dims.y, 0.),
            *origin + Vec3::new(0., dims.y, 0.),

            *origin,
            *origin + Vec3::new(dims.x, 0., 0.),
            *origin + Vec3::new(dims.x, 0., dims.z),
            *origin + Vec3::new(0., 0., dims.z),

            *origin,
            *origin + Vec3::new(0., 0., dims.z),
            *origin + Vec3::new(0., dims.y, dims.z),
            *origin + Vec3::new(0., dims.y, 0.),

            *origin + *dims,
            *origin + *dims - Vec3::new(dims.x, 0., 0.),
            *origin + *dims - Vec3::new(dims.x, dims.y, 0.),
            *origin + *dims - Vec3::new(0., dims.y, 0.),

            *origin + *dims,
            *origin + *dims - Vec3::new(dims.x, 0., 0.),
            *origin + *dims - Vec3::new(dims.x, 0., dims.z),
            *origin + *dims - Vec3::new(0., 0., dims.z),

            *origin + *dims,
            *origin + *dims - Vec3::new(0., 0., dims.z),
            *origin + *dims - Vec3::new(0., dims.y, dims.z),
            *origin + *dims - Vec3::new(0., dims.y, 0.),
        ];
        let mut vertex_vec = Vec::new();
        for vertex in &vertices {
            vertex_vec.append(&mut vertex.to_vec());
        }
        vertex_vec
    }

    fn get_block_indices(index: &mut u32) -> Vec<u32> {
        let mut indices = Vec::new();
        let new_indices = vec![0, 1, 2, 0, 2, 3];
        for _face in 0..6 {
            let mut next_indices = new_indices.clone();
            for new_index in &mut next_indices {
                *new_index += *index;
            }
            indices.append(&mut next_indices);
            *index += 4;
        }
        indices
    }

    pub fn player_input(&mut self, curr_player: usize, input: Input) {
        let mut player = &mut self.players[curr_player];
        match input {
            Input::GoLeft => player.go(Go::Left),
            Input::GoForward => player.go(Go::Forward),
            Input::GoRight => player.go(Go::Right),
            Input::GoBack => player.go(Go::Back),
            Input::StopLeft => player.stop(Go::Left),
            Input::StopForward => player.stop(Go::Forward),
            Input::StopRight => player.stop(Go::Right),
            Input::StopBack => player.stop(Go::Back),
            Input::Jump => player.go(Go::Jump),
            Input::Cast => player.cast_grapple(),
            Input::Pull => player.pull_grapple(),
            Input::Release => player.release_grapple(),
        }
    }

    pub fn cast_grapple(&mut self) {
        self.players[0].cast_grapple();
    }

    pub fn pull_grapple(&mut self) {
        self.players[0].pull_grapple();
    }

    pub fn release_grapple(&mut self) {
        self.players[0].release_grapple();
    }

    pub fn go(&mut self, go: Go) {
        self.players[0].go(go);
    }

    pub fn stop(&mut self, go: Go) {
        self.players[0].stop(go);
    }

    pub fn mouse_look(&mut self, curr_player: usize, movement_x: f32, movement_y: f32) {
        self.players[curr_player].mouse_look(movement_x, movement_y);
    }

    pub fn graphics(&self) -> Graphics {
        self.graphics.clone()
    }
}