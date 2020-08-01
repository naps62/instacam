use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_alocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, cenas!");
}
