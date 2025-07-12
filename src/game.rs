use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    board::Board,
    display::{self, Display},
    log,
    pieces::{PieceState, PieceType},
    utils,
};

#[wasm_bindgen]
pub struct Game {
    board: Board,
    display: Display,
    time_since_last_drop: f64,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Game, JsValue> {
        utils::set_panic_hook();
        let mut game = Game {
            board: Board::new(),
            display: Display::new()?,
            time_since_last_drop: 0.0,
        };
        game.resize();
        wasm_bindgen_futures::spawn_local(async move {
            display::intro_animation().await.unwrap_or_else(|err| {
                log(&format!("Error during intro animation: {:?}", err));
            });
        });
        Ok(game)
    }

    pub fn tick(&mut self, delta_time: f64) {
        self.display.draw(&self.board);

        self.time_since_last_drop += delta_time;
        let drop_interval = 500.0;

        if self.time_since_last_drop >= drop_interval {
            self.time_since_last_drop -= drop_interval;
            let mut next_piece = self.board.current_piece.clone();
            next_piece.row += 1;
            if self.board.is_valid_position(&next_piece) {
                self.board.current_piece = next_piece;
            } else {
                self.board.lock_piece();
                let new_piece_type = self.get_next_piece();
                self.board.current_piece = PieceState::new(new_piece_type, 3); // Start at column 3
            }
        }
    }

    fn get_next_piece(&self) -> PieceType {
        match utils::get_random_int(0, 6) {
            0 => PieceType::Straight,
            1 => PieceType::LLeft,
            2 => PieceType::LRight,
            3 => PieceType::Square,
            4 => PieceType::S,
            5 => PieceType::Z,
            6 => PieceType::T,
            _ => unreachable!(),
        }
    }

    pub fn resize(&mut self) {
        self.display
            .resize(&self.board)
            .unwrap_or_else(|err| log(&format!("Error during resize: {:?}", err)));
    }
}
