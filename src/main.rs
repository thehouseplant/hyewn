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

#[function_component(App)]
fn app() -> Html {
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

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}