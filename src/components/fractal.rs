// Source: https://yew.rs/docs/getting-started/build-a-sample-app
use gloo_console::log;
use num_complex::Complex;
use web_sys::CanvasRenderingContext2d;
use yew::prelude::*;

use crate::elements;
use crate::mathematics::julia_set::draw_julia_set;


// #[derive(PartialEq, Properties)]
#[derive(PartialEq)]
pub struct Fractal {
    x: Complex<f64>,
    y: Complex<f64>,
    zoom: f64,
    node_mandelbrot: NodeRef,
    node_julia:      NodeRef,
}

pub enum Msg {
}


impl Component for Fractal {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log!("Component::Fractal::create()");
        Self {
            x: Complex::new(0.0,0.0),
            y: Complex::new(0.0,0.0),
            zoom: 2.0,
            node_mandelbrot: NodeRef::default(),
            node_julia:      NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        log!("Component::Fractal::update()");
        true  // no rerender
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // let link = ctx.link();
        log!("Component::Fractal::view()");
        html! {
            <div class="fractal">
                <canvas id="mandelbrot" ref={self.node_mandelbrot.clone()}/>
                <canvas id="julia"      ref={self.node_julia.clone()}/>
            </div>
        }
    }

    #[allow(unused_must_use)]
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        // DOCS: https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2d
        let canvas = elements::canvas("mandelbrot").unwrap();
        let canvas_ctx: CanvasRenderingContext2d = elements::canvas_context_2d("mandelbrot").unwrap();
        let width  = canvas.width();
        let height = canvas.height();
        let c = Complex::new(0.25, 0.0);

        // warning: unused `Result` that must be used
        draw_julia_set(&canvas_ctx, width, height, c.re, c.im);

        log!(format!("Component::Fractal::rendered({width} x {height})").as_str());
    }
}
