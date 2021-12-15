use yew::prelude::*;

pub enum Msg {
    AddOne,
    RemoveOne,
}

pub struct Home {
    value: i64,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            },
            Msg::RemoveOne => {
                self.value -= 1;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="main">
                <div class="card">
                    <button onclick={link.callback(|_| Msg::AddOne)}>{ "Add 1" }</button>
                    <button onclick={link.callback(|_| Msg::RemoveOne)}>{ "Remove 1" }</button>
                    <p>{ self.value }</p>
                </div>
            </div>
        }
    }
}