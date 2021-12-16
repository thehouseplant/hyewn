use yew::prelude::*;

pub struct Jobs;

impl Component for Jobs {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ "Jobs" }</h1>
        }
    }
}