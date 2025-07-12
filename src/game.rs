use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::HtmlParagraphElement;

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
    cursor_x: i8,
    score: u32,
    held_piece: Option<PieceType>,
    can_hold_this_turn: bool,
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
            cursor_x: 3,
            score: 0,
            held_piece: None,
            can_hold_this_turn: true,
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
        self.display.draw(&self.board, &self.held_piece);

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let score_element = document
            .query_selector("#score")
            .expect("Expected `#score` element")
            .unwrap()
            .dyn_into::<HtmlParagraphElement>()
            .expect("Expected cast into `HtmlParagraphElement` to succeed");
        score_element.set_inner_text(format!("{}", self.score).as_str());

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
                let lines_cleared = self.board.clear_lines();
                log(format!("{}", lines_cleared).as_str());
                self.score += match lines_cleared {
                    1 => 100,
                    2 => 300,
                    3 => 500,
                    4 => 800,
                    _ => 0,
                };
                let new_piece_type = self.get_next_piece();
                self.board.current_piece = PieceState::new(new_piece_type, self.cursor_x);
                self.can_hold_this_turn = true;
            }
        }
    }

    pub fn resize(&mut self) {
        self.display
            .resize(&self.board)
            .unwrap_or_else(|err| log(&format!("Error during resize: {:?}", err)));
    }
}

impl Game {
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
}

#[wasm_bindgen]
impl Game {
    pub fn move_cursor_left(&mut self) {
        let mut next_piece = self.board.current_piece.clone();
        next_piece.col -= 1;
        if self.board.is_valid_position(&next_piece) {
            self.board.current_piece = next_piece;
        }
    }

    pub fn move_cursor_right(&mut self) {
        let mut next_piece = self.board.current_piece.clone();
        next_piece.col += 1;
        if self.board.is_valid_position(&next_piece) {
            self.board.current_piece = next_piece;
        }
    }

    pub fn rotate_current_piece(&mut self) {
        let mut next_piece = self.board.current_piece.clone();
        next_piece.rotate();

        let kick_offsets = [0, -1, 1, -2, 2];

        for offset in kick_offsets.iter() {
            let mut kicked_piece = next_piece.clone();
            kicked_piece.col += offset;
            if self.board.is_valid_position(&kicked_piece) {
                self.board.current_piece = kicked_piece;
                return;
            }
        }
    }

    pub fn hard_drop_current_piece(&mut self) {
        loop {
            let mut next_piece = self.board.current_piece.clone();
            next_piece.row += 1;
            if self.board.is_valid_position(&next_piece) {
                self.board.current_piece = next_piece;
            } else {
                self.board.lock_piece();
                let lines_cleared = self.board.clear_lines();
                self.score += match lines_cleared {
                    1 => 100,
                    2 => 300,
                    3 => 500,
                    4 => 800,
                    _ => 0,
                };
                let new_piece_type = self.get_next_piece();
                self.board.current_piece = PieceState::new(new_piece_type, self.cursor_x);
                self.can_hold_this_turn = true;
                break;
            }
        }
    }

    pub fn hold_piece(&mut self) {
        if !self.can_hold_this_turn {
            return;
        }

        self.can_hold_this_turn = false;

        let current_piece_type = self.board.current_piece.piece_type.clone();

        if let Some(held) = self.held_piece.take() {
            // Swap: current piece becomes held, held piece becomes current
            self.board.current_piece = PieceState::new(held, self.cursor_x);
            self.held_piece = Some(current_piece_type);
        } else {
            // No held piece: current piece becomes held, new random piece becomes current
            self.held_piece = Some(current_piece_type);
            let new_piece_type = self.get_next_piece();
            self.board.current_piece = PieceState::new(new_piece_type, self.cursor_x);
        }
    }
}
