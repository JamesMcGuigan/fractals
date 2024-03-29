// DOCS: https://yew.rs/docs/getting-started/build-a-sample-app

use gloo_console::log;
use gloo_events::EventListener;
use num_complex::Complex;
use web_sys::CanvasRenderingContext2d;
use yew::prelude::*;

use crate::components::select::Select;
use crate::elements;
use crate::mathematics::julia_set::julia_set_canvas;
use crate::services::colorschemes::ColorScheme;
use crate::services::timer::now;

#[derive(Debug)]
pub struct Fractal {
    _z: Complex<f32>,
    c:  Complex<f32>,
    zoom: f32,
    limit: u32,
    colorscheme: ColorScheme,
    node_canvas: NodeRef,
    listener: Option<EventListener>,
}

pub enum Msg {
    Resize,
    Color(ColorScheme),
}

impl Component for Fractal {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log!("Fractal::create()");
        Self {
            _z: Complex::new(0.0,0.0),
            c:  Complex::new(0.25,0.25),
            zoom: 2.0,
            limit: 32,
            colorscheme: ColorScheme::Green,
            node_canvas: NodeRef::default(),
            listener: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!("Fractal::update()");
        match msg {
            Msg::Color(colorscheme) => {
                self.colorscheme = colorscheme;
                true  // rerender
            },
            Msg::Resize => {
                let window = elements::window().unwrap();
                // let canvas_elm = elements::canvas("mandelbrot").unwrap();
                let canvas_element = self.node_canvas
                    .cast::<web_sys::HtmlCanvasElement>()
                    .expect("HtmlCanvasElement");
                let width  = window.inner_width().unwrap().as_f64().unwrap();
                let height = window.inner_height().unwrap().as_f64().unwrap();
                canvas_element.set_width( width  as u32);
                canvas_element.set_height(height as u32);
                true  // rerender
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!("Fractal::view()");
        // BUGFIX: declare .callback() outside DOM to avoid ctx lifetimes issue
        let colorscheme_onchange = ctx.link().callback(|color: String|
            Msg::Color(ColorScheme::from_string(color))
        );
        html! {
            <div class="fractal">
                <canvas id="mandelbrot" ref={self.node_canvas.clone()}/>
                <div class="controls">
                    <Select
                        options={  ColorScheme::values() }
                        selected={ self.colorscheme.to_string() }
                        onchange={ colorscheme_onchange }
                    />
                </div>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, is_first_render: bool) {
        let time_start = now();

        // let canvas_elm = canvas("mandelbrot").unwrap();
        let canvas_element = self.node_canvas
            .cast::<web_sys::HtmlCanvasElement>()
            .expect("HtmlCanvasElement");
        let width  = canvas_element.width();
        let height = canvas_element.height();

        if is_first_render {
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
                elements::canvas_context_2d(&canvas_element)
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
}
