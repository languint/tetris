use wasm_bindgen::prelude::*;

use crate::utils;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Board {
    pub width: u32,
    pub height: u32,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Board {
        utils::set_panic_hook();
        Board {
            width: 10,
            height: 20,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}