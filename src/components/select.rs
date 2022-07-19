// Example: https://docs.rs/yew/0.6.0/src/yew/components/select.rs.html

use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew::prelude::Properties;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Select {
    node_select: NodeRef,
}

// DOCS: https://yew.rs/docs/concepts/components/properties
// TODO: Unsure how to mark (options + onchange) as required
//       Removing `#[prop_or_default]` causes:
//       options: method not found in `SelectPropsBuilder<SelectPropsBuilderStep_missing_required_prop_onchange>`
#[derive(Properties, PartialEq, Debug, Clone)]
pub struct SelectProps {
    #[prop_or_default] pub options:  Vec<String>,
    #[prop_or_default] pub onchange: Callback<String>,
    #[prop_or_default] pub name:     Option<String>,
    #[prop_or_default] pub selected: Option<String>,
    #[prop_or_default] pub disabled: bool,
}
impl Default for SelectProps {
    fn default() -> Self {
        SelectProps {
            options:  Vec::new(),
            onchange: Callback::from(|_selected: String| {}),
            name:     None,
            selected: None,
            disabled: false,
        }
    }
}

pub enum SelectMsg {
    Selected(Option<String>),
}

impl Component for Select {
    type Message = SelectMsg;
    type Properties = SelectProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_select: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SelectMsg::Selected(value) => {
                if let Some(value) = value {
                    let selected = ctx.props().options
                        .iter()
                        .find(|&option| *option == value)
                    ;
                    if let Some(selected) = selected {
                        ctx.props().onchange.emit(selected.clone());
                    }
                }
            }
        }
        true  // rerender
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // DOCS: https://yew.rs/docs/concepts/html/events
        // DOCS: https://docs.rs/web-sys/latest/web_sys/struct.Event.html
        // DOCS: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlSelectElement.html
        let onchange = ctx.link().callback(move |event: Event| {
            let value: String = event
                .target()
                .expect("event.target()")
                .dyn_ref::<HtmlSelectElement>()
                .expect("event.target().dyn_ref::<HtmlSelectElement>()")
                .value()
            ;
            SelectMsg::Selected(Some(value))
        });
        html! {
            <select
                ref={  self.node_select.clone() }
                name={ ctx.props().name.clone() }
                disabled={ ctx.props().disabled }
                onchange={ onchange }
            >
            { if ctx.props().disabled { html! {
                <option disabled=true selected={ ctx.props().selected.is_none() }/>
            } } else { html! {} }}
            {
                ctx.props().options.iter().map(|option| html!{
                    <option
                        value={ option.to_string() }
                        selected={ ctx.props().selected == Some(option.to_string()) }
                    >
                        { option.to_string() }
                    </option>
                })
                .collect::<Vec<Html>>()
            }
            </select>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
}
