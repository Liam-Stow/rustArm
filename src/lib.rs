extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn update(_time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn render() {
}