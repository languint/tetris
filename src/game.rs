use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{board::Board, display::{self, Display}, log, utils};

#[wasm_bindgen]
pub struct Game {
    board: Board,
    display: Display,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Game, JsValue> {
        utils::set_panic_hook();
        let mut game = Game {
            board: Board::new(),
            display: Display::new()?,
        };
        game.resize();
        wasm_bindgen_futures::spawn_local(async move {
            display::intro_animation().await.unwrap_or_else(|err| {
                log(&format!("Error during intro animation: {:?}", err));
            });
        });
        Ok(game)
    }

    pub fn tick(&mut self) {
        self.display.draw(&self.board);
        // self.board.current_piece.row += 1;
    }

    pub fn resize(&mut self) {
        self.display
            .resize(&self.board)
            .unwrap_or_else(|err| log(&format!("Error during resize: {:?}", err)));
    }
}
