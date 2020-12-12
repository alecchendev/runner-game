mod utils;
use utils::Vec3;

use std::cmp::min;

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

#[wasm_bindgen]
pub struct Player {
    position: Vec3,
    theta: f32,
    phi: f32,

    move_velh: f32,
    move_velz: f32,
    move_velv: f32,

    look_sens: f32,
    move_speed: f32,
    jump_speed: f32,

    gravity: f32,

    bounding_box: BoundingBox,

    universe: Universe,

    on_ground: bool,
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
impl Player {
    pub fn new() -> Self {
        log("Created Player!");
        let position = Vec3::new(2., 1.5, -5.);
        let player_dims = Vec3::new(0.1, 2., 0.1);
        let bounding_box = BoundingBox::new(position - (player_dims / 2.), player_dims);
        Self {
            position,
            theta: 0.0,
            phi: 0.0,
            move_velh: 0.,
            move_velz: 0.,
            move_velv: 0.,
            look_sens: 0.0008,
            move_speed: 0.1,
            jump_speed: 0.25,
            gravity: -0.015,
            bounding_box,
            universe: Universe::new(),
            on_ground: false,
        }
    }

    pub fn update(&mut self) {
        self.move_velv += self.gravity;

        let del_x = self.theta.sin() * self.move_velz + self.theta.cos() * self.move_velh;
        let del_z = self.theta.cos() * self.move_velz + -self.theta.sin() * self.move_velh;
        let del_y = self.move_velv;

        let mut vel = Vec3::new(del_x, del_y, del_z);

        for block in &(self.universe.blocks) {
            
    
            let a = &self.bounding_box;
            let b = &block.bounding_box;//BoundingBox::new(Vec3::new(-1., -1., -1.), Vec3::new(2., 2., 2.));
    
            let a_max = a.get_max();
            let a_min = a.get_min();
            let b_max = b.get_max();
            let b_min = b.get_min();
    
            let a_max_new = a_max + vel;
            let a_min_new = a_min + vel;
            let x_overlap = (a_min_new.x <= b_max.x) && (a_max_new.x >= b_min.x);
            let y_overlap = (a_min_new.y <= b_max.y) && (a_max_new.y >= b_min.y);
            let z_overlap = (a_min_new.z <= b_max.z) && (a_max_new.z >= b_min.z);
            let will_collide = x_overlap && y_overlap && z_overlap;
    
            if will_collide {
                let x_overlap = (a_min.x <= b_max.x) && (a_max.x >= b_min.x);
                let y_overlap = (a_min.y <= b_max.y) && (a_max.y >= b_min.y);
                let z_overlap = (a_min.z <= b_max.z) && (a_max.z >= b_min.z);

                if !x_overlap && y_overlap && z_overlap {
                    vel.x = 0.;
                } else if x_overlap && !y_overlap && z_overlap {
                    vel.y = 0.;
                    self.move_velv = 0.;
                    self.on_ground = true;
                } else if x_overlap && y_overlap && !z_overlap {
                    vel.z = 0.;
                } else {
                    let x_time_collide = (a_min.x - b_max.x).abs().min((a_max.x - b_min.x).abs()) / vel.x;
                    let y_time_collide = (a_min.y - b_max.y).abs().min((a_max.y - b_min.y).abs()) / vel.y;
                    let z_time_collide = (a_min.z - b_max.z).abs().min((a_max.z - b_min.z).abs()) / vel.z;
                    if x_time_collide <= y_time_collide && x_time_collide <= z_time_collide {
                        vel.x = 0.;
                    }
                    if y_time_collide <= x_time_collide && y_time_collide <= z_time_collide {
                        vel.y = 0.;
                        self.move_velv = 0.;
                        self.on_ground = true;
                    }
                    if z_time_collide <= x_time_collide && z_time_collide <= y_time_collide {
                        vel.z = 0.;
                    }
                }
            }
        }

        self.position = self.position + vel;

        self.bounding_box.origin = self.position - (self.bounding_box.dims / 2.);
        
    }

    pub fn collide(&mut self, block: Block) {
        
    }

    pub fn go(&mut self, go: Go) {
        match go {
            Go::Left => self.move_velh = -self.move_speed,
            Go::Forward => self.move_velz = self.move_speed,
            Go::Right => self.move_velh = self.move_speed,
            Go::Back => self.move_velz = -self.move_speed,
            Go::Jump => if self.on_ground { self.move_velv = self.jump_speed; self.on_ground = false; } else { },
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
pub struct BoundingBox {
    origin: Vec3,
    dims: Vec3,
}

impl BoundingBox {
    pub fn new(origin: Vec3, dims: Vec3) -> Self {
        Self {
            origin,
            dims,
        }
    }

    fn get_max(&self) -> Vec3 {
        self.origin + self.dims
    }

    fn get_min(&self) -> Vec3 {
        self.origin
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        let a_max = self.get_max();
        let a_min = self.get_min();
        let b_max = other.get_max();
        let b_min = other.get_min();

        return (a_min.x <= b_max.x) && (a_max.x >= b_min.x) &&
               (a_min.y <= b_max.y) && (a_max.y >= b_min.y) &&
               (a_min.z <= b_max.z) && (a_max.z >= b_min.z);
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Block {
    origin: Vec3,
    dims: Vec3,
    bounding_box: BoundingBox,
}

impl Block {
    pub fn new(origin: Vec3, dims: Vec3) -> Self {
        let bounding_box = BoundingBox::new(origin, dims);
        Self {
            origin,
            dims,
            bounding_box,
        }
    }
}

#[derive(Clone)]
pub enum Surface {
    Tri(Tri),
    Quad(Quad),
}

#[derive(Clone)]
pub struct Tri {
    vertices: [Vec3; 3],
}

impl Tri {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
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
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3, v4: Vec3) -> Self {
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
    blocks: Vec<Block>,
    surfaces: Vec<Surface>,
    positions: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<u32>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let blocks = vec![
            Block::new(Vec3::new(-1., -1., -1.), Vec3::new(1., 1., 1.)),
            Block::new(Vec3::new(-10., -2., -10.), Vec3::new(20., 1., 20.)),
        ];

        let mut surfaces: Vec<Surface> = Vec::new();
        for block in &blocks {
            surfaces.push(Surface::Quad(Quad::new(
                block.origin,
                block.origin + Vec3::new(block.dims.x, 0., 0.),
                block.origin + Vec3::new(block.dims.x, block.dims.y, 0.),
                block.origin + Vec3::new(0., block.dims.y, 0.),
            )));
            surfaces.push(Surface::Quad(Quad::new(
                block.origin,
                block.origin + Vec3::new(block.dims.x, 0., 0.),
                block.origin + Vec3::new(block.dims.x, 0., block.dims.z),
                block.origin + Vec3::new(0., 0., block.dims.z),
            )));
            surfaces.push(Surface::Quad(Quad::new(
                block.origin,
                block.origin + Vec3::new(0., 0., block.dims.z),
                block.origin + Vec3::new(0., block.dims.y, block.dims.z),
                block.origin + Vec3::new(0., block.dims.y, 0.),
            )));

            surfaces.push(Surface::Quad(Quad::new(
                block.origin + block.dims,
                block.origin + block.dims - Vec3::new(block.dims.x, 0., 0.),
                block.origin + block.dims - Vec3::new(block.dims.x, block.dims.y, 0.),
                block.origin + block.dims - Vec3::new(0., block.dims.y, 0.),
            )));
            surfaces.push(Surface::Quad(Quad::new(
                block.origin + block.dims,
                block.origin + block.dims - Vec3::new(block.dims.x, 0., 0.),
                block.origin + block.dims - Vec3::new(block.dims.x, 0., block.dims.z),
                block.origin + block.dims - Vec3::new(0., 0., block.dims.z),
            )));
            surfaces.push(Surface::Quad(Quad::new(
                block.origin + block.dims,
                block.origin + block.dims - Vec3::new(0., 0., block.dims.z),
                block.origin + block.dims - Vec3::new(0., block.dims.y, block.dims.z),
                block.origin + block.dims - Vec3::new(0., block.dims.y, 0.),
            )));
        }

        /*
        let surfaces = vec![
            Surface::Quad(Quad::new(
                Vec3::new(-1.0, -1.0,  1.0),
                Vec3::new(1.0, -1.0,  1.0),
                Vec3::new(1.0,  1.0,  1.0),
                Vec3::new(-1.0,  1.0,  1.0),
            )),
            Surface::Quad(Quad::new(
                Vec3::new(-1.0, -1.0,  -1.0),
                Vec3::new(-1.0, 1.0,  -1.0),
                Vec3::new(1.0,  1.0,  -1.0),
                Vec3::new(1.0,  -1.0,  -1.0),
            )),
            Surface::Quad(Quad::new(
                Vec3::new(-1.0, 1.0,  -1.0),
                Vec3::new(-1.0, 1.0,  1.0),
                Vec3::new(1.0,  1.0,  1.0),
                Vec3::new(1.0,  1.0,  -1.0),
            )),
            Surface::Quad(Quad::new(
                Vec3::new(-1.0, -1.0,  -1.0),
                Vec3::new(1.0, -1.0,  -1.0),
                Vec3::new(1.0, -1.0,  1.0),
                Vec3::new(-1.0,  -1.0,  1.0),
            )),
            Surface::Quad(Quad::new(
                Vec3::new(1.0, -1.0,  -1.0),
                Vec3::new(1.0, 1.0,  -1.0),
                Vec3::new(1.0,  1.0,  1.0),
                Vec3::new(1.0,  -1.0,  1.0),
            )),
            Surface::Quad(Quad::new(
                Vec3::new(1.0, 0.1, -1.0),
                Vec3::new(1.0, -0.1, 1.0),
                Vec3::new(3.0, -0.1, 1.0),
                Vec3::new(3.0, 0.1, -1.0),
            ))
        ];*/

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
            blocks,
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