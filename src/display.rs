use std::cmp;

use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, HtmlHtmlElement};

use crate::{board::Board, utils::sleep};

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

    pub fn draw(&self, board: &Board) {
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

    use wasm_bindgen::JsCast;
    use web_sys::HtmlElement;

    let game_container = document
        .query_selector(".game-container")
        .expect("Expected `.game-container` element")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    game_container.class_list().add_1("fade-in")?;

    Ok(())
}
