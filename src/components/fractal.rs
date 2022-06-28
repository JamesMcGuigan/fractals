// Source: https://yew.rs/docs/getting-started/build-a-sample-app

use anyhow::{anyhow, Result};
use gloo_console::log;
use gloo_events::EventListener;
use num_complex::Complex;
use stdweb::js;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use crate::elements;
use crate::elements::canvas;
use crate::mathematics::julia_set::draw_julia_set;
use crate::services::timer::now;


#[derive(PartialEq, Debug)]
pub struct Fractal {
    z: Complex<f64>,
    c: Complex<f64>,
    zoom: f64,
    node_canvas: NodeRef,
}

pub enum Msg {
    Resize
}


impl Component for Fractal {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log!("Component::Fractal::create()");
        Self {
            z: Complex::new(0.0,0.0),
            c: Complex::new(0.25,0.25),
            zoom: 2.0,
            node_canvas: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log!("Component::Fractal::view()");
        html! {
            <div class="fractal">
                <canvas id="mandelbrot" ref={self.node_canvas.clone()}/>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let time_start = now();

        // let canvas_elm = canvas("mandelbrot").unwrap();
        let canvas_elm = self.node_canvas
            .cast::<web_sys::HtmlCanvasElement>()
            .expect("HtmlCanvasElement");
        let width  = canvas_elm.width();
        let height = canvas_elm.height();

        if first_render {
            ctx.link().send_message(Msg::Resize);
        } else {
            let canvas_ctx: CanvasRenderingContext2d =
                elements::canvas_context_2d(&canvas_elm)
                .unwrap();

            draw_julia_set(&canvas_ctx, width, height, self.c.re, self.c.im);
        }

        let time_taken = (now() - time_start) as i32;
        log!(format!("Component::Fractal::rendered({width} x {height}) = {time_taken}ms").as_str());
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!("Component::Fractal::update()");
        match msg {
            Msg::Resize => {
                let window = elements::window().unwrap();
                // let canvas_elm = elements::canvas("mandelbrot").unwrap();
                let canvas_elm = self.node_canvas
                    .cast::<web_sys::HtmlCanvasElement>()
                    .expect("HtmlCanvasElement");
                let width  = window.inner_width().unwrap().as_f64().unwrap();
                let height = window.inner_height().unwrap().as_f64().unwrap();
                canvas_elm.set_width( width  as u32);
                canvas_elm.set_height(height as u32);
                true    // rerender
            }
        }
    }
}
