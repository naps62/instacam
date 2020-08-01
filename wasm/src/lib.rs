mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn js_log(s: &str);
}

#[wasm_bindgen]
pub fn getSettings(settings: &str) -> String {
    "{}".into()
}

#[wasm_bindgen]
pub fn setSettings(settings: &str) {
    js_log(format!("Hello, instacam-ui from wasm! {}", settings).as_str());
}
