use markdown::to_html;
use wasm_bindgen::prelude::*;

#[cfg(feature = "multi-threading")]
use rayon::prelude::*;

#[cfg(feature = "multi-threading")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[cfg(not(feature = "multi-threading"))]
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn initThreadPool(_hardwareConcurrency: JsValue) {}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[cfg(not(feature = "multi-threading"))]
macro_rules! chunks_mut {
    ($data:expr, $size:expr) => {
        $data.chunks_mut($size)
    };
}

#[cfg(feature = "multi-threading")]
macro_rules! chunks_mut {
    ($data:expr, $size:expr) => {
        $data.par_chunks_mut($size)
    };
}

const README: &str = include_str!("../README.md");

#[wasm_bindgen]
pub fn main() {
    log("Hello from Rust!");

    let html = to_html(README);

    let app = web_sys::window()
        .expect("no global `window` exists")
        .document()
        .expect("no `document` exists")
        .get_element_by_id("app")
        .expect("no element with id `app` exists");

    app.set_inner_html(&html);

}
