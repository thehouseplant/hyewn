use yew::prelude::*;

pub struct PageNotFound;

impl Component for PageNotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <article class="grid">
                <div>
                    <hgroup>
                        <h1>{ "Page Not Found" }</h1>
                        <h2>{ "Seems you may have taken a wrong turn..." }</h2>
                    </hgroup>
                </div>
            </article>
        }
    }
}