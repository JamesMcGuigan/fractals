// BROKEN: expected struct `CanvasRenderingContext2d`, found enum `Result`

use wasm_bindgen::JsCast;
// use wasm_bindgen::prelude::*;

// Source: https://gist.github.com/loloof64/85a794d120cffe347535ad147cc4a7b8#file-main-rs
fn canvas_context_2d(id_selector: &str) -> web_sys::CanvasRenderingContext2d {
    let window   = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas_element: web_sys::Element = document
        .get_element_by_id(id_selector)
        .expect(format!("#{id_selector} not found").as_str())
        // .unwrap()
        // .dyn_into::<web_sys::HtmlCanvasElement>()
        // .expect(format!("#{id_selector} not HtmlCanvasElement").as_str())
        // .unwrap()
    ;
    let canvas: web_sys::HtmlCanvasElement = canvas_element
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect(format!("#{id_selector} not HtmlCanvasElement").as_str())
        // .unwrap()
    ;
    // DOCS: https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext
    // HTMLCanvasElement.getContext( "2d" | "webgl" | "webgl2" | "bitmaprenderer" )
    let context = canvas.get_context("2d")
        // .dyn_into::<web_sys::CanvasRenderingContext2d>()
        // .expect(format!("#{id_selector} failed to get_context()").as_str())
        // .unwrap().unwrap()
    ;
    context
}