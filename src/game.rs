use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::HtmlParagraphElement;

use crate::{
    board::Board,
    display::{self, Display},
    log,
    pieces::{PieceState, PieceType},
    utils,
};
use web_sys::js_sys::Math;

#[wasm_bindgen]
pub struct Game {
    board: Board,
    display: Display,
    time_since_last_drop: f64,
    cursor_x: i8,
    score: u32,
    held_piece: Option<PieceType>,
    can_hold_this_turn: bool,
    next_piece: PieceType,
    bag: Vec<PieceType>,
    bag_index: usize,
    drop_interval_ms: i32,
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
            next_piece: PieceType::Straight,
            bag: Vec::new(),
            bag_index: 0,
            drop_interval_ms: 1000,
        };

        game.new_bag();
        game.next_piece = game.get_next_piece();

        game.resize();
        wasm_bindgen_futures::spawn_local(async move {
            display::intro_animation().await.unwrap_or_else(|err| {
                log(&format!("Error during intro animation: {:?}", err));
            });
        });
        Ok(game)
    }

    pub fn tick(&mut self, delta_time: f64) {
        let mut ghost_piece = self.board.current_piece.clone();
        while self.board.is_valid_position(&ghost_piece.move_down()) {
            ghost_piece.row += 1;
        }

        self.display.draw(
            &self.board,
            &self.held_piece,
            &self.next_piece,
            &ghost_piece,
        );

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

        if self.time_since_last_drop >= self.drop_interval_ms.into() {
            self.time_since_last_drop -= self.drop_interval_ms as f64;
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

                self.board.current_piece = PieceState::new(self.next_piece.clone(), self.cursor_x);

                self.can_hold_this_turn = true;
                let new_piece_type = self.get_next_piece();
                self.next_piece = new_piece_type;
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
    fn new_bag(&mut self) {
        let mut pieces = vec![
            PieceType::Straight,
            PieceType::LLeft,
            PieceType::LRight,
            PieceType::Square,
            PieceType::S,
            PieceType::Z,
            PieceType::T,
        ];

        for i in (0..pieces.len()).rev() {
            let j = (Math::random() * (i + 1) as f64) as usize;
            pieces.swap(i, j);
        }

        self.bag = pieces;
        self.bag_index = 0;
    }

    fn get_next_piece(&mut self) -> PieceType {
        if self.bag_index >= self.bag.len() {
            self.new_bag();
        }
        let piece = self.bag[self.bag_index].clone();
        self.bag_index += 1;
        piece
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

        let kick_tests = [
            (0, 0), 
            (-1, 0),
            (1, 0), 
            (0, 1),
            (-2, 0),
            (2, 0), 
            (0, 2), 
        ];

        for (col_offset, row_offset) in kick_tests.iter() {
            let mut kicked_piece = next_piece.clone();
            kicked_piece.col += col_offset;
            kicked_piece.row += row_offset;

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
                self.board.current_piece = PieceState::new(self.next_piece.clone(), self.cursor_x);

                self.can_hold_this_turn = true;
                let new_piece_type = self.get_next_piece();
                self.next_piece = new_piece_type;
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
            self.board.current_piece = PieceState::new(held, self.cursor_x);
            self.held_piece = Some(current_piece_type);
        } else {
            self.held_piece = Some(current_piece_type);
            self.board.current_piece = PieceState::new(self.next_piece.clone(), self.cursor_x);

            self.can_hold_this_turn = true;
            let new_piece_type = self.get_next_piece();
            self.next_piece = new_piece_type;
        }
    }

    pub fn soft_drop(&mut self) {
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
            self.board.current_piece = PieceState::new(self.next_piece.clone(), self.cursor_x);

            self.can_hold_this_turn = true;
            let new_piece_type = self.get_next_piece();
            self.next_piece = new_piece_type;
        }
    }
}
