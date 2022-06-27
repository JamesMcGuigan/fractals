extern crate yew;
mod _wee_alloc;
mod components;
mod elements;
mod mathematics;

#[allow(dead_code)]
fn main() {
    yew::Renderer::<components::Fractal>::new().render();
}
