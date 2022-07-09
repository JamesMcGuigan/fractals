// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
// #![cfg_attr(debug_assertions, allow(dead_code))]

extern crate fractals;
extern crate yew;
// import lib.rs

// This is the Yew Entry Point
#[allow(dead_code)]
fn main() {
    // DOCS: https://yew.rs/docs/next/concepts/html/events#event-bubbling
    // yew::set_event_bubbling(false);

    yew::Renderer::<fractals::components::Fractal>::new().render();
    // yew::Renderer::<fractals::components::CanvasQuestion>::new().render();
}
