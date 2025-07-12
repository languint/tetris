use wasm_bindgen::prelude::*;

use crate::{
    pieces::{PieceState, PieceType},
    utils,
};

#[wasm_bindgen]
pub struct Board {
    pub width: u32,
    pub height: u32,
    pub(crate) current_piece: PieceState,
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
            current_piece: PieceState::new(PieceType::Straight),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
