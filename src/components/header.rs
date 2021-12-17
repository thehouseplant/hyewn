use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="container-fluid">
                <ul>
                    <li>
                        <strong>{ "HEWN" }</strong>
                    </li>
                </ul>
                <ul>
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