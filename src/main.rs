use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::{
    ask::Ask,
    home::Home,
    jobs::Jobs,
    new::New,
    page_not_found::PageNotFound,
    show::Show,
    top::Top,
};
use yew::html::Scope;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/top")]
    Top,
    #[at("/new")]
    New,
    #[at("/show")]
    Show,
    #[at("/ask")]
    Ask,
    #[at("/jobs")]
    Jobs,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
    ToggleNavbar,
}

struct Model {
    navbar_active: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main class="container">
                    <Switch<Route> render={Switch::render(switch)} />
                </main>

                <footer class="container">
                    <small>
                        { "Built with " }<a href="https://www.rust-lang.org/">{ "Rust" }</a> 
                        { " and " }<a href="https://yew.rs/">{ "Yew" }</a>
                        { " • Styles by "}<a href="https://picocss.com/">{ "Pico" }</a>
                        { " • " }<a href="https://github.com/thehouseplant/hewn">{ "Source" }</a>
                    </small>
                </footer>
            </BrowserRouter>
        }
    }
}

impl Model {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        html! {
            <nav class="container-fluid">
                <ul>
                    <li>
                        <strong>{ "HEWN" }</strong>
                    </li>
                </ul>
                <ul>
                    <li>
                        <Link<Route> to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Top}>
                            { "top" }
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::New}>
                            { "new" }
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Show}>
                            { "show" }
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Ask}>
                            { "ask" }
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Jobs}>
                            { "jobs" }
                        </Link<Route>>
                    </li>
                </ul>
            </nav>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Ask => {
            html! { <Ask /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::Jobs => {
            html! { <Jobs /> }
        }
        Route::New => {
            html! { <New /> }
        }
        Route::Show => {
            html! { <Show /> }
        }
        Route::Top => {
            html! { <Top /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}