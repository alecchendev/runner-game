extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod common_funcs;
mod gl_setup;
mod programs;
mod shaders;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

#[wasm_bindgen]
pub struct Client {
	gl: WebGlRenderingContext,
	program_color_2d: programs::Color2D,
	program_color_2d_gradient: programs::Color2DGradient,
}

#[wasm_bindgen]
impl Client {
	pub fn new() -> Client {
		let gl = gl_setup::initialize_webgl_context().unwrap();
		Client {
			program_color_2d: programs::Color2D::new(&gl),
			program_color_2d_gradient: programs::Color2DGradient::new(&gl),
			gl: gl,
		}
	}

	pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
		app_state::update_dynamic_data(time, height, width);
		Ok(())
	}

	pub fn render(&self) {
		self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

		let curr_state = app_state::get_current_state();

		self.program_color_2d.render(
			&self.gl,
			curr_state.control_bottom, // bottom
			curr_state.control_top, // top
			curr_state.control_left, // left
			curr_state.control_right, // right
			curr_state.canvas_height, // canvas_height
			curr_state.canvas_width, //  canvas_width
		);

		self.program_color_2d_gradient.render(
			&self.gl,
			curr_state.control_bottom + 20., // bottom
			curr_state.control_top - 20., // top
			curr_state.control_left + 20., // left
			curr_state.control_right - 20., // right
			curr_state.canvas_height, // canvas_height
			curr_state.canvas_width, //  canvas_width
		);
	}
}