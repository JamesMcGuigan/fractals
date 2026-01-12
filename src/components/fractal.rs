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
    center: Complex<f32>,
    zoom: f32,
    limit: u32,
    colorscheme: ColorScheme,
    node_canvas: NodeRef,
    listener: Option<EventListener>,
    is_dragging: bool,
    last_mouse_pos: Option<(i32, i32)>,
}

pub enum Msg {
    Resize,
    Color(ColorScheme),
    CRe(f32),
    CIm(f32),
    Zoom(f32),
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
            _z: Complex::new(0.0,0.0),
            c:  Complex::new(0.25,0.25),
            center: Complex::new(0.0, 0.0),
            zoom: 2.0,
            limit: 32,
            colorscheme: ColorScheme::Green,
            node_canvas: NodeRef::default(),
            listener: None,
            is_dragging: false,
            last_mouse_pos: None,
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
                        let min_side = std::cmp::min(width, height) as f32;
                        let scale = 2. * self.zoom / min_side;

                        // Panning logic: move center in opposite direction of mouse movement
                        // Coordinate mapping: re: (y - offset_y) * scale + center_re
                        // So dy in pixels corresponds to dy * scale in complex plane for 're' (y maps to re in julia_set??)
                        // Wait, looking at julia_set:
                        // re: (y as f32 - offset_y) * scale + center_re,
                        // im: (x as f32 - offset_x) * scale + center_im,
                        // This means 'y' (vertical) maps to 're' and 'x' (horizontal) maps to 'im'.
                        // Usually it's the other way around, but I'll stick to the existing implementation.

                        self.center.re -= dy as f32 * scale;
                        self.center.im -= dx as f32 * scale;

                        self.last_mouse_pos = Some((x, y));
                        return true;
                    }
                }
                false
            }
            Msg::Wheel(delta_y) => {
                // delta_y is positive for scrolling down (zoom out), negative for scrolling up (zoom in)
                let zoom_factor = 1.1f32;
                if delta_y > 0.0 {
                    self.zoom *= zoom_factor;
                } else if delta_y < 0.0 {
                    self.zoom /= zoom_factor;
                }
                // Clamp zoom to reasonable range
                self.zoom = self.zoom.clamp(0.001, 10.0);
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
                        options={  ColorScheme::values() }
                        selected={ self.colorscheme.to_string() }
                        onchange={ colorscheme_onchange }
                    />
                    <label><span>{"C Real: "}{format!("{:.3}", self.c.re)}</span>
                        <input type="range" min="-1" max="1" step="0.001" value={self.c.re.to_string()} oninput={on_cre_input} />
                    </label>
                    <label><span>{"C Imag: "}{format!("{:.3}", self.c.im)}</span>
                        <input type="range" min="-1" max="1" step="0.001" value={self.c.im.to_string()} oninput={on_cim_input} />
                    </label>
                    <label><span>{"Zoom: "}{format!("{:.3}", self.zoom)}</span>
                        <input type="range" min="0.001" max="4" step="0.001" value={self.zoom.to_string()} oninput={on_zoom_input} />
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

        let _time_taken = (now() - time_start) / 1000.0;
        log!(format!("Fractal::rendered({width} x {height}) = {_time_taken:.3}s").as_str());
    }
}
