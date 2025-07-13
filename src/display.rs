use std::cmp;

use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, HtmlElement, HtmlHtmlElement};

use crate::{
    board::Board,
    pieces::{PieceState, PieceType},
    utils::sleep,
};

#[wasm_bindgen]
pub struct Display {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    cell_size: u32,
}

#[wasm_bindgen]
impl Display {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Display, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document
            .query_selector(".game-canvas")?
            .expect("Expected `.game-canvas` element")
            .dyn_into::<HtmlCanvasElement>()?;
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Display {
            canvas,
            context,
            cell_size: 20,
        })
    }
}

impl Display {
    pub fn draw(
        &self,
        board: &Board,
        held_piece: &Option<PieceType>,
        next_piece_type: &PieceType,
        ghost_piece: &PieceState,
    ) {
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width().into(),
            self.canvas.height().into(),
        );
        self.draw_board(board)
            .expect("Expected `draw_board` call to succeed");
        self.draw_piece(board)
            .expect("Expected `draw_piece` call to succeed");

        if let Some(piece_type) = held_piece {
            self.draw_held_piece(piece_type)
                .expect("Expected `draw_held_piece` call to succeed");
        }

        self.draw_ghost_piece(ghost_piece)
            .expect("Expected `draw_ghost_piece` call to succeed");
        self.draw_next_piece(next_piece_type)
            .expect("Expected `draw_next_piece` call to succeed");
    }

    fn draw_ghost_piece(&self, piece_state: &PieceState) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        self.context.begin_path();

        let color = format!("--{}", piece_state.color());

        let fill_color = window.get_computed_style(&document.document_element().unwrap())?;

        self.context.set_fill_style_str(
            fill_color
                .unwrap()
                .get_property_value(color.as_str())
                .unwrap()
                .as_str(),
        );
        self.context.set_global_alpha(0.3);

        for (r, c) in piece_state.iter_blocks() {
            if r >= 0 {
                self.context.fill_rect(
                    c as f64 * self.cell_size as f64,
                    r as f64 * self.cell_size as f64,
                    self.cell_size as f64,
                    self.cell_size as f64,
                )
            }
        }
        self.context.fill();
        self.context.set_global_alpha(1.0);

        Ok(())
    }

    fn draw_board(&self, board: &Board) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let root = document.document_element().unwrap();
        let style = window.get_computed_style(&root).unwrap().unwrap();

        self.context.begin_path();
        for (r, row) in board.placed_pieces.iter().enumerate() {
            for (c, color) in row.iter().enumerate() {
                if let Some(color) = color {
                    let fill_color = style.get_property_value(&format!("--{}", color)).unwrap();
                    self.context.set_fill_style_str(&fill_color);
                    self.context.fill_rect(
                        (c as u32 * self.cell_size) as f64,
                        (r as u32 * self.cell_size) as f64,
                        self.cell_size as f64,
                        self.cell_size as f64,
                    );
                }
            }
        }
        self.context.fill();
        Ok(())
    }

    fn draw_piece(&self, board: &Board) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        self.context.begin_path();

        let color = format!("--{}", board.current_piece.color());

        let fill_color = window.get_computed_style(&document.document_element().unwrap())?;

        self.context.set_fill_style_str(
            fill_color
                .unwrap()
                .get_property_value(color.as_str())
                .unwrap()
                .as_str(),
        );

        for (r, c) in board.current_piece.iter_blocks() {
            if r >= 0 {
                self.context.fill_rect(
                    c as f64 * self.cell_size as f64,
                    r as f64 * self.cell_size as f64,
                    self.cell_size as f64,
                    self.cell_size as f64,
                )
            }
        }
        self.context.fill();

        Ok(())
    }

    fn draw_held_piece(&self, held_piece_type: &PieceType) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let held_canvas = document
            .query_selector(".held-canvas")?
            .expect("Expected `.held-canvas` element")
            .dyn_into::<HtmlCanvasElement>()
            .expect("Expected `dyn_into` cast to succeed");

        let held_context = held_canvas
            .get_context("2d")?
            .expect("Expected 2d context")
            .dyn_into::<CanvasRenderingContext2d>()?;

        held_context.clear_rect(
            0.0,
            0.0,
            held_canvas.width().into(),
            held_canvas.height().into(),
        );

        let piece_state = PieceState::new(held_piece_type.clone(), 0);

        held_context.begin_path();

        let color = format!("--{}", piece_state.color());

        let fill_color = window.get_computed_style(&document.document_element().unwrap())?;

        held_context.set_fill_style_str(
            fill_color
                .unwrap()
                .get_property_value(color.as_str())
                .unwrap()
                .as_str(),
        );

        let mut min_r = 4;
        let mut max_r = 0;
        let mut min_c = 4;
        let mut max_c = 0;

        for (r, c) in piece_state.iter_blocks() {
            min_r = cmp::min(min_r, r);
            max_r = cmp::max(max_r, r);
            min_c = cmp::min(min_c, c);
            max_c = cmp::max(max_c, c);
        }

        let piece_width_cells = max_c - min_c + 1;
        let piece_height_cells = max_r - min_r + 1;

        let scale_x =
            held_canvas.width() as f64 / (piece_width_cells as f64 * self.cell_size as f64);
        let scale_y =
            held_canvas.height() as f64 / (piece_height_cells as f64 * self.cell_size as f64);
        let scale_factor = scale_x.min(scale_y);

        let scaled_cell_size = self.cell_size as f64 * scale_factor;

        let scaled_piece_width = piece_width_cells as f64 * scaled_cell_size;
        let scaled_piece_height = piece_height_cells as f64 * scaled_cell_size;

        let offset_x = (held_canvas.width() as f64 - scaled_piece_width) / 2.0;
        let offset_y = (held_canvas.height() as f64 - scaled_piece_height) / 2.0;

        for (r, c) in piece_state.iter_blocks() {
            held_context.fill_rect(
                ((c - min_c) as f64 * scaled_cell_size) + offset_x,
                ((r - min_r) as f64 * scaled_cell_size) + offset_y,
                scaled_cell_size,
                scaled_cell_size,
            )
        }
        held_context.fill();

        Ok(())
    }

    fn draw_next_piece(&self, next_piece_type: &PieceType) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let next_canvas = document
            .query_selector(".next-canvas")?
            .expect("Expected `.next-canvas` element")
            .dyn_into::<HtmlCanvasElement>()
            .expect("Expected `dyn_into` cast to succeed");

        let held_context = next_canvas
            .get_context("2d")?
            .expect("Expected 2d context")
            .dyn_into::<CanvasRenderingContext2d>()?;

        held_context.clear_rect(
            0.0,
            0.0,
            next_canvas.width().into(),
            next_canvas.height().into(),
        );

        let piece_state = PieceState::new(next_piece_type.clone(), 0);

        held_context.begin_path();

        let color = format!("--{}", piece_state.color());

        let fill_color = window.get_computed_style(&document.document_element().unwrap())?;

        held_context.set_fill_style_str(
            fill_color
                .unwrap()
                .get_property_value(color.as_str())
                .unwrap()
                .as_str(),
        );

        let mut min_r = 4;
        let mut max_r = 0;
        let mut min_c = 4;
        let mut max_c = 0;

        for (r, c) in piece_state.iter_blocks() {
            min_r = cmp::min(min_r, r);
            max_r = cmp::max(max_r, r);
            min_c = cmp::min(min_c, c);
            max_c = cmp::max(max_c, c);
        }

        let piece_width_cells = max_c - min_c + 1;
        let piece_height_cells = max_r - min_r + 1;

        let scale_x =
            next_canvas.width() as f64 / (piece_width_cells as f64 * self.cell_size as f64);
        let scale_y =
            next_canvas.height() as f64 / (piece_height_cells as f64 * self.cell_size as f64);
        let scale_factor = scale_x.min(scale_y);

        let scaled_cell_size = self.cell_size as f64 * scale_factor;

        let scaled_piece_width = piece_width_cells as f64 * scaled_cell_size;
        let scaled_piece_height = piece_height_cells as f64 * scaled_cell_size;

        let offset_x = (next_canvas.width() as f64 - scaled_piece_width) / 2.0;
        let offset_y = (next_canvas.height() as f64 - scaled_piece_height) / 2.0;

        for (r, c) in piece_state.iter_blocks() {
            held_context.fill_rect(
                ((c - min_c) as f64 * scaled_cell_size) + offset_x,
                ((r - min_r) as f64 * scaled_cell_size) + offset_y,
                scaled_cell_size,
                scaled_cell_size,
            )
        }
        held_context.fill();

        Ok(())
    }

    pub fn resize(&mut self, board: &Board) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let game_container = document
            .query_selector(".game-container")?
            .expect("Expected `.game-container` element");

        let available_width = game_container.client_width() as u32;
        let available_height = game_container.client_height() as u32;

        self.cell_size = cmp::min(
            available_width / board.width,
            available_height / board.height,
        );

        self.canvas.set_width(board.width * self.cell_size);
        self.canvas.set_height(board.height * self.cell_size);

        let html_element: HtmlHtmlElement = document
            .document_element()
            .unwrap()
            .dyn_into::<HtmlHtmlElement>()?;

        html_element
            .style()
            .set_property("--cell-size", format!("{}px", self.cell_size).as_str())?;

        Ok(())
    }
}

pub async fn intro_animation() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let splashscreen = document
        .query_selector(".splashscreen")
        .expect("Expected `.splashscreen` element")
        .unwrap();

    let splashscreen_text_container = document
        .query_selector(".splashscreen-text")
        .expect("Expected `.splashscreen-text` element")
        .unwrap();

    let splashscreen_text_elements = splashscreen_text_container.children();
    let mut elements: Vec<Element> = Vec::new();
    for i in 0..splashscreen_text_elements.length() {
        if let Some(element) = splashscreen_text_elements.item(i) {
            elements.push(element);
        }
    }
    elements.reverse();

    JsFuture::from(sleep(1000)).await?;

    for i in 0..elements.len() + 1 {
        for (j, element) in elements.iter().enumerate() {
            if j < i {
                element.class_list().set_value("splashscreen-active");
            }
        }
        JsFuture::from(sleep(500)).await?;
    }

    splashscreen.class_list().add_1("fade-out")?;
    JsFuture::from(sleep(500)).await?;
    splashscreen.class_list().add_1("hidden")?;

    fade_in_menu()?;
    JsFuture::from(sleep(500)).await?;

    Ok(())
}

pub fn fade_in_menu() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let menu_container = document
        .query_selector(".menu")
        .expect("Expected `.menu` element")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    menu_container.class_list().set_value("menu fade-in");

    Ok(())
}

pub fn fade_out_menu() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let menu_container = document
        .query_selector(".menu")
        .expect("Expected `.menu` element")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    menu_container.class_list().set_value("menu fade-out");

    Ok(())
}

pub fn fade_in_game() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let menu_container = document
        .query_selector(".game-container")
        .expect("Expected `.game-container` element")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    menu_container.class_list().set_value("game-container fade-in");

    Ok(())
}

pub fn fade_out_game() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let menu_container = document
        .query_selector(".game-container")
        .expect("Expected `.game-container` element")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    menu_container.class_list().set_value("game-container fade-out");

    Ok(())
}
