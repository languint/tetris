mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Board {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Board {
        utils::set_panic_hook();
        Board {
            width: 10,
            height: 20,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

fn sleep(ms: i32) -> Promise {
    Promise::new(&mut |resolve, _| {
        let window = web_sys::window().expect("no global `window` exists");
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    })
}

async fn intro_animation() -> Result<(), JsValue> {
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

#[wasm_bindgen(start)]
pub fn run() {
    utils::set_panic_hook();
    wasm_bindgen_futures::spawn_local(async {
        intro_animation().await.unwrap_or_else(|err| {
            log(&format!("Error during intro animation: {:?}", err));
        });
    });
}
