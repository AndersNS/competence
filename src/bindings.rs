use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::graph::Config;

#[wasm_bindgen]
extern "C" {

    pub type Window;

    #[wasm_bindgen(method, getter, js_name = "wasmBindgenSnippetsPath")]
    pub fn wasm_bindgen_snippets_path(this: &Window) -> String;
}

#[wasm_bindgen(module = "/js/chart.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn renderChart(element: HtmlElement, data: JsValue) -> HtmlElement;
}
