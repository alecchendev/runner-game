mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, runner-game!");
}


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pixel {
    Black = 0,
    White = 1,
}

#[wasm_bindgen]
pub struct Screen {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

#[wasm_bindgen]
impl Screen {
    pub fn new() -> Screen {
        let width = 720;
        let height = 480;
        let pixels = (0..width * height)
            .map(|_i| Pixel::Black)
            .collect();

        Screen {
            width,
            height,
            pixels,
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.pixels.clone();
        
        for (idx, pixel) in self.pixels.iter().enumerate() {
            let next_pixel = if pixel == &Pixel::Black { Pixel::White } else { Pixel::Black };
            next[idx] = next_pixel;
        }

        self.pixels = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }
}