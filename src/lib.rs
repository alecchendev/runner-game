/*use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use web_sys::*;
use js_sys::WebAssembly;
use nalgebra::{Matrix4,Perspective3};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("runner-game-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec4 aVertexPosition;
        attribute vec4 aVertexColor;

        uniform mat4 uModelViewMatrix;
        uniform mat4 uProjectionMatrix;

        varying lowp vec4 vColor;

        void main() {
            gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
            vColor = aVertexColor;
        }
        "#,
    )?;
    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        varying lowp vec4 vColor;

        void main() {
            gl_FragColor = vColor;
        }
        "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let positions = [
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

    let positions_memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    let vertices_location = positions.as_ptr() as u32 / 4;
    let position_buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));
    context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::new(&positions_memory_buffer).subarray(
            vertices_location,
            vertices_location + positions.len() as u32,
        ),
        WebGlRenderingContext::STATIC_DRAW,
    );

    /*let colors = [
        1.0,  1.0,  1.0,  1.0,   // Front face: white
        1.0,  1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,  1.0,

        1.0,  0.0,  0.0,  1.0,    // Back face: red
        1.0,  0.0,  0.0,  1.0,
        1.0,  0.0,  0.0,  1.0,
        1.0,  0.0,  0.0,  1.0,

        0.0,  1.0,  0.0,  1.0,    // Top face: green
        0.0,  1.0,  0.0,  1.0,
        0.0,  1.0,  0.0,  1.0,
        0.0,  1.0,  0.0,  1.0,

        0.0,  0.0,  1.0,  1.0,    // Bottom face: blue
        0.0,  0.0,  1.0,  1.0,
        0.0,  0.0,  1.0,  1.0,
        0.0,  0.0,  1.0,  1.0,

        1.0,  1.0,  0.0,  1.0,    // Right face: yellow
        1.0,  1.0,  0.0,  1.0,
        1.0,  1.0,  0.0,  1.0,
        1.0,  1.0,  0.0,  1.0,

        //[1.0,  0.0,  1.0,  1.0],    // Left face: purple
    ];

    let colors_memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    let colors_location = colors.as_ptr() as u32;
    let color_buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::new(&colors_memory_buffer).subarray(
            colors_location,
            colors_location + colors.len() as u32,
        ),
        WebGlRenderingContext::STATIC_DRAW,
    );
    */

    let indices = [
        0,  1,  2,      0,  2,  3,    // front
        4,  5,  6,      4,  6,  7,    // back
        8,  9,  10,     8,  10, 11,   // top
        12, 13, 14,     12, 14, 15,   // bottom
        16, 17, 18,     16, 18, 19,   // right
        //20, 21, 22,     20, 22, 23,   // left
    ];

    let indices_memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    let indices_location = indices.as_ptr() as u32 / 2;
    let indices_buffer = context.create_buffer().unwrap();
    context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&indices_buffer));
    context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        &js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
            indices_location,
            indices_location + indices.len() as u32,
        ),
        WebGlRenderingContext::STATIC_DRAW
    );

    /*let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }*/

    // CONSTATNS
    let canvas_width = 720.;
    let canvas_height = 480.;
    let half_display_size = 0.9 * canvas_height / 2.0;
    let bottom = canvas_height / 2.0 - half_display_size;
    let top = canvas_height / 2.0 + half_display_size;
    let left = canvas_width / 2.0 - half_display_size;
    let right = canvas_width / 2.0 + half_display_size;

    let FIELD_OF_VIEW = 45.0 * std::f32::consts::PI / 180.0;
    let aspect = canvas_width / canvas_height;
    let Z_NEAR = 0.1;
    let Z_FAR = 100.0;
    let Z_PLANE = (std::f32::consts::PI / 8.0).tan();

    let theta: f32 = -0.0;
    let phi: f32 = -0.0;

    let position = vec![0.0, 0.0, -10.0];

    // UNIFORM LOCATIONS
    let projection_location = context.get_uniform_location(&program, "uProjectionMatrix").unwrap();
    let modelview_location = context.get_uniform_location(&program, "uModelViewMatrix").unwrap();

    // PROJECTION
    let projection_matrix_tmp: Perspective3<f32> = Perspective3::new(aspect, FIELD_OF_VIEW, Z_NEAR, Z_FAR);
    let mut projection_matrix: [f32; 16] = [0.; 16];
    projection_matrix.copy_from_slice(projection_matrix_tmp.as_matrix().as_slice());

    // MODELVIEW
    let rotate_x_axis: [f32; 16] = [
        1., 0., 0., 0.,
        0., phi.cos(), -phi.sin(), 0.,
        0., phi.sin(), phi.cos(), 0.,
        0., 0., 0., 1.,
    ];

    let rotate_y_axis: [f32; 16] = [
        theta.cos(),  0., theta.sin(), 0.,
        0., 1., 0., 0.,
        -theta.sin(), 0., theta.cos(), 0.,
        0., 0., 0., 1.,
    ];

    let translation_matrix: [f32; 16] = translation_matrix(
        -position[0],
        -position[1],
        position[2]
    );

    let rotation_matrix = mult_matrix_4(rotate_x_axis, rotate_y_axis);
    let modelview_matrix = mult_matrix_4(translation_matrix, rotation_matrix);

    context.uniform_matrix4fv_with_f32_array(Some(&projection_location), false, &projection_matrix);
    context.uniform_matrix4fv_with_f32_array(Some(&modelview_location), false, &modelview_matrix);

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.vertex_attrib_pointer_with_i32(1, 4, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(1);

    context.clear_color(0.012, 0.647, 0.988, 1.0);
    context.clear_depth(1.0);
    context.enable(WebGlRenderingContext::DEPTH_TEST);
    context.depth_func(WebGlRenderingContext::LEQUAL);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

    //context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));
    //context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&indices_buffer));

    context.draw_elements_with_i32(
        WebGlRenderingContext::TRIANGLES,
        (positions.len() / 3) as i32,
        WebGlRenderingContext::UNSIGNED_SHORT,
        0
    );
    Ok(())
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}


// UTILS
pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
    let mut return_var = [0.; 16];

    return_var[0] = 1.;
    return_var[5] = 1.;
    return_var[10] = 1.;
    return_var[15] = 1.;

    return_var[12] = tx;
    return_var[13] = ty;
    return_var[14] = tz;

    return_var
}

pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
    let mut return_var = [0.; 16];

    return_var[0] = sx;
    return_var[5] = sy;
    return_var[10] = sz;
    return_var[15] = 1.;

    return_var
}

pub fn mult_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut return_var = [0.; 16];

    return_var[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
    return_var[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
    return_var[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
    return_var[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

    return_var[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
    return_var[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
    return_var[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
    return_var[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

    return_var[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
    return_var[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
    return_var[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
    return_var[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

    return_var[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
    return_var[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
    return_var[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
    return_var[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

    return_var
}
*/
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