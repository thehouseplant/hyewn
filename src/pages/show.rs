use yew::prelude::*;

pub struct Show;

impl Component for Show {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ "Show" }</h1>
        }
    }
}