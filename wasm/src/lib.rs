use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn js_log(s: &str);
}


#[wasm_bindgen]
pub async fn get_settings() -> Result<String, String> {
    // TODO this is not working
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("http://localhost:8000/settings", &opts).unwrap();

    request
        .headers()
        .set("Accept", "application/json");

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
    // let resp: Response = resp_value.dyn_into().unwrap();

    // let json = JsFuture::from(resp.json()?).await?;

    js_log(format!("get_settings {:?}", resp_value).as_str());

    Ok("asd".into())
}

#[wasm_bindgen]
pub fn set_settings(settings: String) {
    // TODO this is not making a request
    js_log(format!("set_settings {:?}", settings).as_str());
}
