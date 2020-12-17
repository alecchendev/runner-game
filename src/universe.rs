use wasm_bindgen::prelude::*;
use super::utils::Vec3;
use super::player::Player;

use super::block::Block;
use super::graphics::Graphics;

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
        let blocks = vec![
            Block::new(Vec3::new(-10., -2., -9.), Vec3::new(20., 1., 20.)),
            Block::new(Vec3::new(-1., -1., -1.), Vec3::new(2., 1.5, 2.)),
            Block::new(Vec3::new(0., 0., 0.), Vec3::new(2., 1.5, 2.)),
            Block::new(Vec3::new(1.75, 0.5, 4.5), Vec3::new(0.5, 2.0, 0.5)),
            Block::new(Vec3::new(4.5, 0.0, -1.5), Vec3::new(0.5, 3.0, 3.5)),
        ];
        Self {
            players: vec![Player::new()],
            gravity: -0.015,
            blocks,
            graphics: Graphics::new(),
        }
    }

    pub fn update(&mut self) {
        self.players[0].update(&self.blocks, self.gravity);
        self.update_graphics();
    }

    fn update_graphics(&mut self) {
        let mut positions = vec![];
        let mut colors = vec![];
        let mut indices = vec![];

        let mut index = 0;
        for block in &self.blocks {
            let vertices: Vec<Vec3> = vec![
                block.origin,
                block.origin + Vec3::new(block.dims.x, 0., 0.),
                block.origin + Vec3::new(block.dims.x, block.dims.y, 0.),
                block.origin + Vec3::new(0., block.dims.y, 0.),

                block.origin,
                block.origin + Vec3::new(block.dims.x, 0., 0.),
                block.origin + Vec3::new(block.dims.x, 0., block.dims.z),
                block.origin + Vec3::new(0., 0., block.dims.z),

                block.origin,
                block.origin + Vec3::new(0., 0., block.dims.z),
                block.origin + Vec3::new(0., block.dims.y, block.dims.z),
                block.origin + Vec3::new(0., block.dims.y, 0.),

                block.origin + block.dims,
                block.origin + block.dims - Vec3::new(block.dims.x, 0., 0.),
                block.origin + block.dims - Vec3::new(block.dims.x, block.dims.y, 0.),
                block.origin + block.dims - Vec3::new(0., block.dims.y, 0.),

                block.origin + block.dims,
                block.origin + block.dims - Vec3::new(block.dims.x, 0., 0.),
                block.origin + block.dims - Vec3::new(block.dims.x, 0., block.dims.z),
                block.origin + block.dims - Vec3::new(0., 0., block.dims.z),

                block.origin + block.dims,
                block.origin + block.dims - Vec3::new(0., 0., block.dims.z),
                block.origin + block.dims - Vec3::new(0., block.dims.y, block.dims.z),
                block.origin + block.dims - Vec3::new(0., block.dims.y, 0.),
            ];
            for vertex in vertices {
                positions.append(&mut vertex.to_vec().clone());
            }
            for face in 0..6 {
                let mut new_indices = vec![0, 1, 2, 0, 2, 3];
                for new_index in &mut new_indices {
                    *new_index += index;
                }
                indices.append(&mut new_indices);
                index += 4;
            }
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
        let mut block_colors = vec![];
        for block in 0..(self.blocks.len() - 1) {
            for index in 0..color_pattern.len() {
                colors.push(color_pattern[(block * 4 + index) % color_pattern.len()]);
            }
        }

        self.graphics.update(positions, colors, indices);
    }

    pub fn graphics(&self) -> Graphics {
        self.graphics.clone()
    }
}