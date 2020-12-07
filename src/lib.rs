mod utils;

use wasm_bindgen::prelude::*;

use std::fmt;
extern crate js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Point(f32, f32, f32);

#[wasm_bindgen]
pub struct Player {
    position: Point,
    theta: f32,
    phi: f32,

    look_velh: f32,
    look_velv: f32,
    move_vel: f32,

    look_speed: f32,
    move_speed: f32,
}

#[wasm_bindgen]
impl Player {
    pub fn new() -> Self {
        Self {
            position: Point(0., 0., -6.),
            theta: 0.1,
            phi: 0.1,
            look_velh: 0.,
            look_velv: 0.,
            move_vel: 0.,
            look_speed: 0.02,
            move_speed: 0.02,
        }
    }

    pub fn update(&mut self) {
        self.theta += self.look_velh;
        self.phi += self.look_velv;
    }

    pub fn look_right(&mut self) {
        self.look_velh = self.look_speed;
    }

    pub fn look_left(&mut self) {
        self.look_velh = -self.look_speed;
    }

    pub fn look_up(&mut self) {
        self.look_velv = -self.look_speed;
    }

    pub fn look_down(&mut self) {
        self.look_velv = self.look_speed;
    }

    pub fn stop_look_right(&mut self) {
        if self.look_velh == self.look_speed {
            self.look_velh = 0.;
        }
    }

    pub fn stop_look_left(&mut self) {
        if self.look_velh == -self.look_speed {
            self.look_velh = 0.;
        }
    }

    pub fn stop_look_up(&mut self) {
        if self.look_velv == -self.look_speed {
            self.look_velv = 0.;
        }
    }

    pub fn stop_look_down(&mut self) {
        if self.look_velv == self.look_speed {
            self.look_velv = 0.;
        }
    }

    pub fn position(&self) -> Vec<f32> {
        vec![self.position.0, self.position.1, self.position.2]
    }

    pub fn theta(&self) -> f32 {
        self.theta
    }

    pub fn phi(&self) -> f32 {
        self.phi
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                /*if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }*/
                let rand_num = js_sys::Math::random();
                if rand_num < 0.5 { Cell::Dead } else { Cell::Alive }
            })
            .collect();
        
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, x) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.height - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col) as usize;
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}