// Source:   https://github.com/jessaimaya/rust_wasm/blob/89cea2a7aea240f6bd3ec8722e539aca35cee73c/src/browser.rs
// QUESTION: https://stackoverflow.com/questions/72647778/stdwebwebhtml-elementcanvaselement-from-document-query-selector-in-rus

use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

#[allow(dead_code)]
// DOCS: https://developer.mozilla.org/en-US/docs/Web/API/Window
pub fn window() -> Result<Window> {
    web_sys::window()
        .ok_or_else(|| anyhow!("No Window Found"))
}

#[allow(dead_code)]
// DOCS: https://developer.mozilla.org/en-US/docs/Web/API/Document
pub fn document() -> Result<Document> {
    window()?
        .document()
        .ok_or_else(|| anyhow!("No Document Found"))
}

#[allow(dead_code)]
// DOCS: https://developer.mozilla.org/en-US/docs/Web/API/HtmlCanvasElement
pub fn canvas(id_selector: &str) -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id(id_selector)
        .ok_or_else(|| anyhow!("No Canvas Element found with ID 'canvas'"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement", element))
}

#[allow(dead_code)]
// DOCS: https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2d
pub fn canvas_context_2d(id_selector: &str) -> Result<CanvasRenderingContext2d> {
    canvas(id_selector)?
        .get_context("2d")
        .map_err(|js_value| anyhow!("Error getting 2d context {:#?}", js_value))?
        .ok_or_else(|| anyhow!("No 2d context found"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|element| {
            anyhow!(
                "Error converting {:#?} to CanvasRenderingContext2d",
                element
            )
        })
}