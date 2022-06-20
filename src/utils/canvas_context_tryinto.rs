// BROKEN: the trait `From<stdweb::web::Element>` is not implemented for `CanvasElement`

use stdweb::web::{CanvasRenderingContext2d, document, IParentNode};
use stdweb::web::html_element::CanvasElement;
// use web_sys;

// BUGFIX: https://stackoverflow.com/questions/72647778/stdwebwebhtml-elementcanvaselement-from-document-query-selector-in-rus
// EXAMPLE: https://github.com/veeenu/protein-wasm/blob/cfb4bd2ec8aea45078dc2c4f5c6b7e8baa59e754/src/lib.rs
fn canvas_context_2d(selector: &str) -> CanvasRenderingContext2d {
    let element = document().query_selector(selector)
        .expect(format!("query_selector({selector}) not found").as_str())
        .unwrap()
    ;
    // BUG: the trait `From<stdweb::web::Element>` is not implemented for `CanvasElement`
    let canvas_element: CanvasElement = element
        .try_into()
        // .dyn_into::<web_sys::HtmlCanvasElement>()
        // .dyn_into::<CanvasElement>()
        .expect(format!("query_selector({selector}) not CanvasElement").as_str())
    ;
    let context: CanvasRenderingContext2d = canvas_element
        .get_context()
        .expect(format!("query_selector({selector}) failed to get_context()").as_str())
    ;
    context
}

// EXAMPLE: https://github.com/veeenu/protein-wasm/blob/cfb4bd2ec8aea45078dc2c4f5c6b7e8baa59e754/src/lib.rs
// fn get_canvas_context(selector: &str) -> CanvasRenderingContext2d {
//     let canvas = // window()
//         // .unwrap()
//         document()
//         .unwrap()
//         .create_element("canvas")
//         .unwrap()
//         .dyn_into::<HtmlCanvasElement>()
//         .unwrap();
//
//     let context: CanvasRenderingContext2d = canvas
//         .get_context()
//         .expect(format!("query_selector({selector}) failed to get_context()").as_str())
//     ;
//     context
// }