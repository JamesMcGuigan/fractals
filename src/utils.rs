use stdweb::web::{CanvasRenderingContext2d, document, Element, IParentNode};
use stdweb::web::html_element::CanvasElement;

// Source: https://gist.github.com/loloof64/85a794d120cffe347535ad147cc4a7b8#file-main-rs
fn get_canvas_context(selector: &str) -> CanvasRenderingContext2d {
    let element: Element = document().query_selector(selector)
        .expect(format!("query_selector({selector}) not found").as_str())
        .unwrap()
    ;
    // BUG: the trait `From<stdweb::web::Element>` is not implemented for `CanvasElement`
    let canvas_element: CanvasElement = element
        .try_into()
        .expect(format!("query_selector({selector}) not CanvasElement").as_str())
    ;
    let context: CanvasRenderingContext2d = canvas_element
        .get_context()
        .expect(format!("query_selector({selector}) failed to get_context()").as_str())
    ;
    context
}