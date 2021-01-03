use wasm_bindgen::prelude::*;
use super::Universe;
use super::level::Level;
use super::utils::Vec3;
use super::block::Block;
use super::log;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Mode {
    Menu,
    Play,
    WonLevel,
}

#[wasm_bindgen]
pub struct Master {
    levels: Vec<Level>,
    mode: Mode,
}

#[wasm_bindgen]
impl Master {
    pub fn new() -> Self {
        let level1_block_data = vec![
            -10., -1., -10., 20., 1., 25.,

            -10., 0., 10., 20., 5., 2.,
            -4., 0., 6., 5., 1.5, 4.,
            -3.25, 1.5, 7., 3., 1.5, 3.,
            -2.25, 3., 8., 1.5, 1., 2.,

            -10., 2., 25., 20., 1., 25.,
            -5., 11., 18., 10., 1., 8.,

            -7., 4., 35., 3., 0.3, 3.,
            -7., 5.5, 39.5, 3., 0.3, 3.,
            -7., 9., 47., 3., 3., 0.3,
            -7., 13., 49.5, 3., 0.3, 3.,

            -1.5, 20., 55., 3., 0.3, 3.,

            4.5, 15., 60., 3., 0.3, 6.,
        ];
        let level2_block_data = vec![
            -10., -1., -10., 20., 1., 25.,
        ];
        Self {
            levels: vec![
                Level::new(
                    level1_block_data,
                    Block::new(Vec3::new(4.5, 15.3, 64.), Vec3::new(3., 3., 2.)),
                    Vec3::new(2., 1.5, -5.),
                ),
                Level::new(
                    level2_block_data,
                    Block::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.)),
                    Vec3::new(2., 1.5, -5.),
                ),
            ],
            mode: Mode::Menu,
        }
    }

    pub fn start(&self, level: usize) -> Universe {
        let curr_level = &self.levels[level];
        Universe::new(curr_level.block_data(), curr_level.win_block(), curr_level.start_pos())
    }

    pub fn set_mode(&mut self, new_mode: Mode) {
        log(&format!("switched!")[..]);
        self.mode = new_mode;
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }
}