// DOCS: https://yew.rs/docs/getting-started/build-a-sample-app

use enum_iterator;
use enum_iterator::all;
use gloo_console::log;
use gloo_events::EventListener;
use num_complex::Complex;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlSelectElement};
use web_sys::EventTarget;
use yew::prelude::*;

use crate::elements;
use crate::mathematics::julia_set::julia_set_canvas;
#[allow(unused_imports)]
use crate::services::colorschemes::{colorscheme_grayscale, colorscheme_hsl};
use crate::services::colorschemes::ColorScheme;
use crate::services::timer::now;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Fractal {
    z: Complex<f32>,
    c: Complex<f32>,
    zoom: f32,
    limit: u32,
    colorscheme: ColorScheme,
    node_canvas: NodeRef,
    listener: Option<EventListener>,
}

pub enum Msg {
    Resize,
    ColorScheme(ColorScheme),
}


impl Component for Fractal {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log!("Fractal::create()");
        Self {
            z: Complex::new(0.0,0.0),
            c: Complex::new(0.25,0.25),
            zoom: 2.0,
            limit: 32,
            colorscheme: ColorScheme::Green,
            node_canvas: NodeRef::default(),
            listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!("Fractal::view()");
        let colorschemes: Vec<ColorScheme> = all::<ColorScheme>().collect();
        html! {
            <div class="fractal">
                <canvas id="mandelbrot" ref={self.node_canvas.clone()}/>
                <div class="controls">
                    <select name="colorscheme"
                        onchange={
                            ctx.link().callback(move |event: Event| {
                                // DOCS: https://yew.rs/docs/concepts/html/events
                                // DOCS: https://docs.rs/web-sys/latest/web_sys/struct.Event.html
                                // DOCS: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlSelectElement.html
                                let target: EventTarget = event.target().unwrap();
                                let select: HtmlSelectElement = target
                                    .dyn_ref::<HtmlSelectElement>().unwrap().clone();
                                let value: String = select.value();
                                let colorscheme: ColorScheme = value.parse().unwrap();
                                Msg::ColorScheme(colorscheme)
                            })
                        }
                    >
                    {
                        colorschemes.iter().map(|colorscheme| html!{
                            <option
                                value={ colorscheme.to_string() }
                                selected={ self.colorscheme == *colorscheme }
                            >
                                { colorscheme.to_string() }
                            </option>
                        })
                        .collect::<Vec<Html>>()
                    }
                    </select>
                </div>
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
            let onresize = ctx.link().callback(|_: Event| Msg::Resize);
            let listener = EventListener::new(
                &web_sys::window().unwrap(),
                "resize",
                move |e| onresize.emit(e.clone())
            );
            self.listener = Some(listener);
        } else {
            let canvas_ctx: CanvasRenderingContext2d =
                elements::canvas_context_2d(&canvas_elm)
                .unwrap();

            julia_set_canvas(
                &canvas_ctx,
                width, height,
                self.c.re, self.c.im,
                self.zoom,
                self.limit,
                self.colorscheme,
            );
        }

        let _time_taken = (now() - time_start) / 1000.0;
        log!(format!("Fractal::rendered({width} x {height}) = {_time_taken:.3}s").as_str());
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!("Fractal::update()");
        match msg {
            Msg::ColorScheme(colorscheme) => {
                self.colorscheme = colorscheme;
                true  // rerender
            },
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
                true  // rerender
            }
        }
    }
}
