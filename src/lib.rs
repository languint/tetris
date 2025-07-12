mod board;
mod display;
mod utils;

use wasm_bindgen::prelude::*;

use crate::display::intro_animation;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() {
    utils::set_panic_hook();
    wasm_bindgen_futures::spawn_local(async move {
        intro_animation().await.unwrap_or_else(|err| {
            log(&format!("Error during intro animation: {:?}", err));
        });
    });
}
