use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
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