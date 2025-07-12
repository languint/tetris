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
    pub(crate) placed_pieces: Vec<Vec<Option<String>>>,
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
            current_piece: PieceState::new(PieceType::Straight, 0),
            placed_pieces: vec![vec![None; 10]; 20],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Board {
    pub(crate) fn is_valid_position(&self, piece: &PieceState) -> bool {
        piece.iter_blocks().all(|(r, c)| {
            if r < 0 || r >= self.height as i8 || c < 0 || c >= self.width as i8 {
                return false;
            }
            self.placed_pieces[r as usize][c as usize].is_none()
        })
    }

    pub(crate) fn lock_piece(&mut self) {
        let color = self.current_piece.color();
        for (r, c) in self.current_piece.iter_blocks() {
            self.placed_pieces[r as usize][c as usize] = Some(color.to_string());
        }
    }

    pub(crate) fn clear_lines(&mut self) -> u32 {
        let mut lines_cleared = 0;
        let mut r = self.height as usize;

        while r > 0 {
            r -= 1;
            if self.placed_pieces[r].iter().all(Option::is_some) {
                lines_cleared += 1;

                for i in (1..=r).rev() {
                    self.placed_pieces[i] = self.placed_pieces[i - 1].clone();
                }

                self.placed_pieces[0] = vec![None; self.width as usize];
                r += 1;
            }
        }
        lines_cleared
    }
}
