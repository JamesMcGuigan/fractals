// Source: https://yew.rs/docs/getting-started/build-a-sample-app
use yew::prelude::*;
// use num::complex::Complex;
use num_complex::Complex;
// use gloo_console::log;

// #[derive(PartialEq, Properties)]
#[derive(PartialEq)]
pub struct Fractal {
    x: Complex<f64>,
    y: Complex<f64>,
    zoom: f64,
}

pub enum Msg {
}


impl Component for Fractal {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            x: Complex::new(0.0,0.0),
            y: Complex::new(0.0,0.0),
            zoom: 2.0,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true  // rerender
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        // let link = ctx.link();
        html! {
            <div class="fractal">
                <canvas id="mandelbrot"/>
                <canvas id="julia"/>
            </div>
        }
    }
}