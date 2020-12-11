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
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

#[wasm_bindgen]
pub struct Player {
    position: Point,
    theta: f32,
    phi: f32,

    look_velh: f32,
    look_velv: f32,
    move_velh: f32,
    move_velz: f32,
    move_velv: f32,

    look_speed: f32,
    look_sens: f32,
    move_speed: f32,
    jump_speed: f32,

    gravity: f32,

    universe: Universe,
}

#[wasm_bindgen]
pub enum Go {
    Left = 0,
    Forward = 1,
    Right = 2,
    Back = 3,
    Jump = 4,
}

#[wasm_bindgen]
pub enum Look {
    Left = 0,
    Up = 1,
    Right = 2,
    Down = 3,
}

#[wasm_bindgen]
impl Player {
    pub fn new() -> Self {
        log("Created Player!");
        Self {
            position: Point::new(2., 0., -5.),
            theta: 0.0,
            phi: 0.0,
            look_velh: 0.,
            look_velv: 0.,
            move_velh: 0.,
            move_velz: 0.,
            move_velv: 0.,
            look_speed: 0.02,
            look_sens: 0.0008,
            move_speed: 0.1,
            jump_speed: 0.25,
            gravity: -0.015,
            universe: Universe::new(),
        }
    }

    pub fn update(&mut self) {
        self.theta += self.look_velh;
        self.phi = {
            if (self.phi + self.look_velv).abs() < std::f32::consts::PI / 2. {
                self.phi + self.look_velv
            } else {
                self.phi
            }
        };
        

        let del_x = self.theta.sin() * self.move_velz + self.theta.cos() * self.move_velh;
        let del_z = self.theta.cos() * self.move_velz + -self.theta.sin() * self.move_velh;
        self.position.x += del_x; // theta only
        self.position.y += self.move_velv; // good
        self.position.z += del_z; // theta only
        
        if self.position.y > 0. {
            self.move_velv += self.gravity;
        } else {
            self.position.y = 0.;
        }
    }

    pub fn go(&mut self, go: Go) {
        match go {
            Go::Left => self.move_velh = -self.move_speed,
            Go::Forward => self.move_velz = self.move_speed,
            Go::Right => self.move_velh = self.move_speed,
            Go::Back => self.move_velz = -self.move_speed,
            Go::Jump => if self.position.y == 0. { self.move_velv = self.jump_speed } else { },
        }
    }

    pub fn stop(&mut self, go: Go) {
        match go {
            Go::Left => {
                if self.move_velh == -self.move_speed {
                    self.move_velh = 0.;
                }
            },
            Go::Forward => {
                if self.move_velz == self.move_speed {
                    self.move_velz = 0.;
                }
            },
            Go::Right => {
                if self.move_velh == self.move_speed {
                    self.move_velh = 0.;
                }
            },
            Go::Back => {
                if self.move_velz == -self.move_speed {
                    self.move_velz = 0.;
                }
            },
            _ => (),
        }
    }

    pub fn mouse_look(&mut self, movement_x: f32, movement_y: f32) {
        let del_theta = movement_x * self.look_sens;
        let del_phi = movement_y * self.look_sens;
        self.theta += del_theta;
        self.phi = {
            if (self.phi + del_phi).abs() < std::f32::consts::PI / 2. {
                self.phi + del_phi
            } else {
                self.phi
            }
        };
    }

    pub fn look(&mut self, look: Look) {
        match look {
            Look::Left => self.look_left(),
            Look::Up => self.look_up(),
            Look::Right => self.look_right(),
            Look::Down => self.look_down(),
        }
    }

    pub fn stop_look(&mut self, look: Look) {
        match look {
            Look::Left => self.stop_look_left(),
            Look::Up => self.stop_look_up(),
            Look::Right => self.stop_look_right(),
            Look::Down => self.stop_look_down(),
        }
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
        vec![self.position.x, self.position.y, self.position.z]
    }

    pub fn theta(&self) -> f32 {
        self.theta
    }

    pub fn phi(&self) -> f32 {
        self.phi
    }

    pub fn universe(&self) -> Universe {
        self.universe.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Universe {
    positions: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<u32>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        
        let positions = vec![
            // Front face
            -1.0, -1.0,  1.0,
             1.0, -1.0,  1.0,
             1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,
            
            // Back face
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
             1.0,  1.0, -1.0,
             1.0, -1.0, -1.0,
            
            // Top face
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0, -1.0,
            
            // Bottom face
            -1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0,
            
            // Right face
             1.0, -1.0, -1.0,
             1.0,  1.0, -1.0,
             1.0,  1.0,  1.0,
             1.0, -1.0,  1.0,
            
        ];
        let colors = vec![
            1.0,  1.0,  1.0,  1.0,    // Front face: white
            1.0,  0.0,  0.0,  1.0,    // Back face: red
            0.0,  1.0,  0.0,  1.0,    // Top face: green
            0.0,  0.0,  1.0,  1.0,    // Bottom face: blue
            1.0,  1.0,  0.0,  1.0,    // Right face: yellow
        ];

        let indices = vec![
            0,  1,  2,      0,  2,  3,    // front
            4,  5,  6,      4,  6,  7,    // back
            8,  9,  10,     8,  10, 11,   // top
            12, 13, 14,     12, 14, 15,   // bottom
            16, 17, 18,     16, 18, 19,   // right
            //20, 21, 22,     20, 22, 23,   // left
        ];
        Universe {
            positions,
            colors,
            indices,
        }
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