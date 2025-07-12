use std::cmp;

use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Element, HtmlCanvasElement, HtmlHtmlElement};

use crate::{board::Board, log, utils::sleep};

#[wasm_bindgen]
pub fn resize(board: &Board) -> Result<(), JsValue> {
    log("resize");
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let game_container = document
        .query_selector(".game-container")?
        .expect("Expected `.game-container` element");

    let available_width = game_container.client_width() as u32;
    let available_height = game_container.client_height() as u32;

    let cell_size = cmp::min(
        available_width / board.width,
        available_height / board.height,
    );

    let canvas: HtmlCanvasElement = document
        .query_selector(".game-canvas")?
        .expect("Expected `.game-canvas` element")
        .dyn_into::<HtmlCanvasElement>()?;

    canvas.set_width(board.width * cell_size);
    canvas.set_height(board.height * cell_size);

    let html_element: HtmlHtmlElement = document.document_element().unwrap().dyn_into::<HtmlHtmlElement>()?;

    html_element.style().set_property("--cell-size", format!("{cell_size}px").as_str())?;

    Ok(())
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