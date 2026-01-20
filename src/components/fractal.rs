// DOCS: https://yew.rs/docs/getting-started/build-a-sample-app

use gloo_console::log;
use gloo_events::EventListener;
use num_complex::Complex;
use web_sys::CanvasRenderingContext2d;
use yew::prelude::*;

use crate::components::select::Select;
use crate::elements;
use crate::mathematics::julia_set::julia_set_canvas;
use crate::mathematics::mandelbrot::mandelbrot_set_canvas;
use crate::services::colorschemes::ColorScheme;
use crate::services::timer::now;

#[derive(Copy, Clone, Debug, PartialEq, Eq, strum_macros::Display, enum_utils::IterVariants, enum_utils::FromStr)]
pub enum FractalType {
    Julia,
    Mandelbrot,
}

impl FractalType {
    pub fn values() -> Vec<String> {
        FractalType::iter()
            .map(|f| f.to_string())
            .collect()
    }
}

#[derive(Debug)]
pub struct Fractal {
    fractal_type: FractalType,
    _z: Complex<f64>,
    c:  Complex<f64>,
    center: Complex<f64>,
    zoom: f64,
    limit: u32,
    colorscheme: ColorScheme,
    node_canvas: NodeRef,
    listener: Option<EventListener>,
    is_dragging: bool,
    last_mouse_pos: Option<(i32, i32)>,
}

pub enum Msg {
    Resize,
    Type(FractalType),
    Color(ColorScheme),
    CRe(f64),
    CIm(f64),
    Zoom(f64),
    Limit(u32),
    MouseDown(i32, i32),
    MouseMove(i32, i32),
    MouseUp,
    Wheel(f64),
}

impl Component for Fractal {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log!("Fractal::create()");
        Self {
            fractal_type: FractalType::Mandelbrot,
            _z: Complex::new(0.0,0.0),
            c:  Complex::new(-1.0,0.0),
            center: Complex::new(0.0, 0.0),
            zoom: 2.0,
            limit: 32,
            colorscheme: ColorScheme::Ultra,
            node_canvas: NodeRef::default(),
            listener: None,
            is_dragging: false,
            last_mouse_pos: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!("Fractal::update()");
        match msg {
            Msg::Type(fractal_type) => {
                self.fractal_type = fractal_type;
                true
            },
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
            Msg::CRe(re) => {
                self.c.re = re;
                true
            }
            Msg::CIm(im) => {
                self.c.im = im;
                true
            }
            Msg::Zoom(zoom) => {
                self.zoom = zoom;
                true
            }
            Msg::Limit(limit) => {
                self.limit = limit;
                true
            }
            Msg::MouseDown(x, y) => {
                self.is_dragging = true;
                self.last_mouse_pos = Some((x, y));
                false
            }
            Msg::MouseUp => {
                self.is_dragging = false;
                self.last_mouse_pos = None;
                false
            }
            Msg::MouseMove(x, y) => {
                if self.is_dragging {
                    if let Some((last_x, last_y)) = self.last_mouse_pos {
                        let dx = x - last_x;
                        let dy = y - last_y;

                        let canvas_element = self.node_canvas
                            .cast::<web_sys::HtmlCanvasElement>()
                            .expect("HtmlCanvasElement");
                        let width = canvas_element.width();
                        let height = canvas_element.height();
                        let min_side = std::cmp::min(width, height) as f64;
                        let scale = 2. * self.zoom / min_side;

                        // Panning logic: move center in opposite direction of mouse movement
                        // Coordinate mapping: 
                        // re: (x - offset_x) * scale + center_re
                        // im: (y - offset_y) * scale + center_im
                        // This means 'x' (horizontal) maps to 're' and 'y' (vertical) maps to 'im'.

                        self.center.re -= dx as f64 * scale;
                        self.center.im -= dy as f64 * scale;

                        self.last_mouse_pos = Some((x, y));
                        return true;
                    }
                }
                false
            }
            Msg::Wheel(delta_y) => {
                // delta_y is positive for scrolling down (zoom out), negative for scrolling up (zoom in)
                let zoom_factor = 1.1f64;
                if delta_y > 0.0 {
                    self.zoom *= zoom_factor;
                } else if delta_y < 0.0 {
                    self.zoom /= zoom_factor;
                }
                // Clamp zoom to reasonable range
                self.zoom = self.zoom.clamp(1e-15, 10.0);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!("Fractal::view()");
        // BUGFIX: declare .callback() outside DOM to avoid ctx lifetimes issue
        let colorscheme_onchange = ctx.link().callback(|color: String|
            Msg::Color(ColorScheme::from_string(color))
        );
        let fractal_type_onchange = ctx.link().callback(|fractal_type: String| {
            use std::str::FromStr;
            Msg::Type(FractalType::from_str(&fractal_type).unwrap_or(FractalType::Julia))
        });
        let on_cre_input = ctx.link().callback(|e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            Msg::CRe(input.value().parse().unwrap_or(0.0))
        });
        let on_cim_input = ctx.link().callback(|e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            Msg::CIm(input.value().parse().unwrap_or(0.0))
        });
        let on_zoom_input = ctx.link().callback(|e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            Msg::Zoom(input.value().parse().unwrap_or(1.0))
        });
        let on_limit_input = ctx.link().callback(|e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            Msg::Limit(input.value().parse().unwrap_or(32))
        });

        let on_mousedown = ctx.link().callback(|e: MouseEvent| {
            Msg::MouseDown(e.client_x(), e.client_y())
        });
        let on_mousemove = ctx.link().callback(|e: MouseEvent| {
            Msg::MouseMove(e.client_x(), e.client_y())
        });
        let on_mouseup = ctx.link().callback(|_| {
            Msg::MouseUp
        });
        let on_wheel = ctx.link().callback(|e: WheelEvent| {
            e.prevent_default(); // Prevent page scroll
            Msg::Wheel(e.delta_y())
        });

        html! {
            <div class="fractal">
                <canvas 
                    id="mandelbrot" 
                    ref={self.node_canvas.clone()}
                    onmousedown={on_mousedown}
                    onmousemove={on_mousemove}
                    onmouseleave={on_mouseup.clone()}
                    onmouseup={on_mouseup}
                    onwheel={on_wheel}
                />
                <div class="controls">
                    <Select
                        options={  FractalType::values() }
                        selected={ self.fractal_type.to_string() }
                        onchange={ fractal_type_onchange }
                    />
                    <Select
                        options={  ColorScheme::values() }
                        selected={ self.colorscheme.to_string() }
                        onchange={ colorscheme_onchange }
                    />
                    { if self.fractal_type == FractalType::Julia {
                        html! {
                            <>
                                <label><span>{"C Real: "}</span>
                                    <input type="number" step="0.001" value={self.c.re.to_string()} oninput={on_cre_input.clone()} />
                                    <input type="range" min="-1" max="1" step="0.001" value={self.c.re.to_string()} oninput={on_cre_input} />
                                </label>
                                <label><span>{"C Imag: "}</span>
                                    <input type="number" step="0.001" value={self.c.im.to_string()} oninput={on_cim_input.clone()} />
                                    <input type="range" min="-1" max="1" step="0.001" value={self.c.im.to_string()} oninput={on_cim_input} />
                                </label>
                            </>
                        }
                    } else { html! {} } }
                    <label><span>{"Zoom: "}</span>
                        <input type="number" step="1e-15" value={self.zoom.to_string()} oninput={on_zoom_input.clone()} />
                        <input type="range" min="1e-15" max="4" step="1e-15" value={self.zoom.to_string()} oninput={on_zoom_input} />
                    </label>
                    <label><span>{"Limit: "}</span>
                        <input type="number" step="1" value={self.limit.to_string()} oninput={on_limit_input.clone()} />
                        <input type="range" min="1" max="10000" step="1" value={self.limit.to_string()} oninput={on_limit_input} />
                    </label>
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

            match self.fractal_type {
                FractalType::Julia => {
                    julia_set_canvas(
                        &canvas_ctx,
                        width, height,
                        self.c.re, self.c.im,
                        self.center.re, self.center.im,
                        self.zoom,
                        self.limit,
                        self.colorscheme,
                    );
                }
                FractalType::Mandelbrot => {
                    mandelbrot_set_canvas(
                        &canvas_ctx,
                        width, height,
                        self.center.re, self.center.im,
                        self.zoom,
                        self.limit,
                        self.colorscheme,
                    );
                }
            }
        }

        let _time_taken = (now() - time_start) / 1000.0;
        log!(format!("Fractal::rendered({width} x {height}) = {_time_taken:.3}s").as_str());
    }
}
