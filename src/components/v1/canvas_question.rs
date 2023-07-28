// DOCS: https://yew.rs/docs/concepts/html/events
// DOCS: https://github.com/yewstack/yew/issues/1258
// QUESTION: https://stackoverflow.com/questions/72790427/type-mismatch-in-closure-arguments-expected-signature-of-forr-fnr-yewe

use gloo_console::log;
use gloo_events::EventListener;
use web_sys::CanvasRenderingContext2d;
use yew::prelude::*;

use crate::elements;

pub struct CanvasQuestion {
    node_canvas: NodeRef,
    listener: Option<EventListener>,
}

pub enum Msg {
    Resize
}


impl Component for CanvasQuestion {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log!("Component::Canvas::create()");
        Self {
            node_canvas: NodeRef::default(),
            listener: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!("Component::Canvas::update()");
        match msg {
            Msg::Resize => {
                let window = elements::window().unwrap();
                let canvas = elements::canvas("canvas").unwrap();
                let width  = window.inner_width().unwrap().as_f64().unwrap();
                let height = window.inner_height().unwrap().as_f64().unwrap();
                canvas.set_width( width  as u32);
                canvas.set_height(height as u32);
                true    // rerender
            }
        }
    }


    fn view(&self, _ctx: &Context<Self>) -> Html {
        log!("Component::Canvas::view()");
        html! {
            <canvas id="canvas" ref={self.node_canvas.clone()}/>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::Resize);
            let onresize = ctx.link().callback(|_: Event| Msg::Resize);
            let listener = EventListener::new(
                &web_sys::window().unwrap(),
                "resize",
                move |e| onresize.emit(e.clone())
            );
            self.listener = Some(listener);

            // // BUG: the trait `stdweb::unstable::TryFrom<stdweb::Value>` is not implemented for `yew::Event`
            // js! {
            //     console.log("JS::rendered()");
            //     var onWindowResize = @{onresize};
            //     window.addEventListener("resize", onWindowResize);
            // }
        }

        let canvas_elm = elements::canvas("canvas").unwrap();
        let _canvas_ctx: CanvasRenderingContext2d =
            elements::canvas_context_2d(&canvas_elm).unwrap();
        let _width  = canvas_elm.width();
        let _height = canvas_elm.height();
        log!(format!("Component::Fractal::rendered({_width} x {_height})").as_str());
    }
}