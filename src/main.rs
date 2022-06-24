extern crate yew;
mod _wee_alloc;
mod components;
mod html;
mod mathematics;

#[allow(dead_code)]
fn main() {
    yew::Renderer::<components::Fractal>::new().render();
}
