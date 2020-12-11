mod utils;

use wasm_bindgen::prelude::*;

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
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Clone, Copy)]
pub struct Point {
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

    fn to_vec(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }
}

#[wasm_bindgen]
pub struct Player {
    position: Point,
    theta: f32,
    phi: f32,

    move_velh: f32,
    move_velz: f32,
    move_velv: f32,

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
            move_velh: 0.,
            move_velz: 0.,
            move_velv: 0.,
            look_sens: 0.0008,
            move_speed: 0.1,
            jump_speed: 0.25,
            gravity: -0.015,
            universe: Universe::new(),
        }
    }

    pub fn update(&mut self) {
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

    pub fn position(&self) -> Vec<f32> {
        self.position.to_vec()
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

#[derive(Clone)]
pub enum Surface {
    Tri(Tri),
    Quad(Quad),
}

#[derive(Clone)]
pub struct Tri {
    vertices: [Point; 3],
}

impl Tri {
    pub fn new(v1: Point, v2: Point, v3: Point) -> Self {
        Self {
            vertices: [v1, v2, v3],
        }
    }
    pub fn to_vertices(&self) -> Vec<f32> {
        let mut vertices = Vec::new();
        vertices.append(&mut self.vertices[0].to_vec().clone());
        vertices.append(&mut self.vertices[1].to_vec().clone());
        vertices.append(&mut self.vertices[2].to_vec().clone());
        vertices
    }

    pub fn to_indices(&self) -> Vec<u32> {
        vec![0, 1, 2]
    }
}

#[derive(Clone)]
pub struct Quad {
    tris: [Tri; 2],
}

impl Quad {
    pub fn new(v1: Point, v2: Point, v3: Point, v4: Point) -> Self {
        Self {
            tris: [Tri::new(v1, v2, v3), Tri::new(v1, v3, v4)]
        }
    }
    pub fn to_vertices(&self) -> Vec<f32> {
        let mut vertices = Vec::new();
        vertices.append(&mut self.tris[0].vertices[0].to_vec().clone());
        vertices.append(&mut self.tris[0].vertices[1].to_vec().clone());
        vertices.append(&mut self.tris[0].vertices[2].to_vec().clone());
        vertices.append(&mut self.tris[1].vertices[2].to_vec().clone());
        vertices
    }

    pub fn to_indices(&self) -> Vec<u32> {
        vec![0, 1, 2, 0, 2, 3]
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Universe {
    surfaces: Vec<Surface>,
    positions: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<u32>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let surfaces = vec![
            Surface::Quad(Quad::new(
                Point::new(-1.0, -1.0,  1.0),
                Point::new(1.0, -1.0,  1.0),
                Point::new(1.0,  1.0,  1.0),
                Point::new(-1.0,  1.0,  1.0),
            )),
            Surface::Quad(Quad::new(
                Point::new(-1.0, -1.0,  -1.0),
                Point::new(-1.0, 1.0,  -1.0),
                Point::new(1.0,  1.0,  -1.0),
                Point::new(1.0,  -1.0,  -1.0),
            )),
            Surface::Quad(Quad::new(
                Point::new(-1.0, 1.0,  -1.0),
                Point::new(-1.0, 1.0,  1.0),
                Point::new(1.0,  1.0,  1.0),
                Point::new(1.0,  1.0,  -1.0),
            )),
            Surface::Quad(Quad::new(
                Point::new(-1.0, -1.0,  -1.0),
                Point::new(1.0, -1.0,  -1.0),
                Point::new(1.0, -1.0,  1.0),
                Point::new(-1.0,  -1.0,  1.0),
            )),
            Surface::Quad(Quad::new(
                Point::new(1.0, -1.0,  -1.0),
                Point::new(1.0, 1.0,  -1.0),
                Point::new(1.0,  1.0,  1.0),
                Point::new(1.0,  -1.0,  1.0),
            )),
            Surface::Quad(Quad::new(
                Point::new(1.0, 0.1, -1.0),
                Point::new(1.0, -0.1, 1.0),
                Point::new(3.0, -0.1, 1.0),
                Point::new(3.0, 0.1, -1.0),
            ))
        ];

        let mut positions = vec![];
        let mut indices = vec![];
        let mut next_index = 0;
        for surface in &surfaces {
            let mut new_vertices = match surface {
                Surface::Quad(quad) => quad.to_vertices(),
                Surface::Tri(tri) => tri.to_vertices(),
            };
            positions.append(&mut new_vertices);
            let mut index_add = 0;
            let mut new_indices = match surface {
                Surface::Quad(quad) => {
                    index_add = 4;
                    quad.to_indices()
                },
                Surface::Tri(tri) => {
                    index_add = 3;
                    tri.to_indices()
                },
            };
            for index in &mut new_indices {
                *index += next_index;
                log_u32(*index);
            }
            indices.append(&mut new_indices);
            next_index += index_add;
        }
        
        /*let positions = vec![
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
            
        ];*/
        let colors = vec![
            1.0,  1.0,  1.0,  1.0,    // Front face: white
            1.0,  0.0,  0.0,  1.0,    // Back face: red
            0.0,  1.0,  0.0,  1.0,    // Top face: green
            0.0,  0.0,  1.0,  1.0,    // Bottom face: blue
            1.0,  1.0,  0.0,  1.0,    // Right face: yellow
            1.0,  0.0,  1.0,  1.0,    // Right face: yellow
        ];

        /*let indices = vec![
            0,  1,  2,      0,  2,  3,    // front
            4,  5,  6,      4,  6,  7,    // back
            8,  9,  10,     8,  10, 11,   // top
            12, 13, 14,     12, 14, 15,   // bottom
            16, 17, 18,     16, 18, 19,   // right
            //20, 21, 22,     20, 22, 23,   // left
        ];*/

        Universe {
            surfaces,
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