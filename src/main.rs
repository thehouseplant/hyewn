use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod util;

use components::{
    footer::Footer,
    header::Header,
};

use util::routes::{
    Routes, switch
};

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Header />

                <main class="container">
                    <Switch<Routes> render={Switch::render(switch)} />
                </main>

                <Footer />
            </BrowserRouter>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}