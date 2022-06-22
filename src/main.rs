extern crate yew;
mod _wee_alloc;
mod fractal;
mod html;
mod julia_set;

#[allow(dead_code)]
fn main() {
    yew::Renderer::<fractal::Fractal>::new().render();
}
