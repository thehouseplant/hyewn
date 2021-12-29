use yew::prelude::*;
use yew_router::prelude::*;

pub mod components;
pub mod pages;
pub mod util;

use components::{
    footer::Footer,
    header::Header,
};

use util::routes::{
    Routes, switch
};

const API_URL: &str = "https://hacker-news.firebaseio.com/v0";

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