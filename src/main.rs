#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#![cfg_attr(debug_assertions, allow(dead_code))]
extern crate yew;

mod _wee_alloc;
mod components;
mod elements;
mod services;
mod mathematics;

#[allow(dead_code)]
fn main() {
    // DOCS: https://yew.rs/docs/next/concepts/html/events#event-bubbling
    // yew::set_event_bubbling(false);

    yew::Renderer::<components::Fractal>::new().render();
    // yew::Renderer::<components::CanvasQuestion>::new().render();
}
