extern crate yew;
extern crate fractals;

use yew::Renderer;  // import lib.rs
use crate::fractal::Fractal;
mod fractal;

fn main() {
    Renderer::<Fractal>::new().render();
}
