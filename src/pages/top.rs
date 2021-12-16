use yew::prelude::*;

pub struct Top;

impl Component for Top {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ "Top" }</h1>
        }
    }
}