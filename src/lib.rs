extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

mod gl_setup;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

#[wasm_bindgen]
pub struct Client {
	gl: WebGlRenderingContext,
}

#[wasm_bindgen]
impl Client {
	pub fn new() -> Client {
		let gl = gl_setup::initialize_webgl_context().unwrap();
		Client {
			gl: gl,
		}
	}

	pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
		Ok(())
	}

	pub fn render(&self) {
		self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
	}
}