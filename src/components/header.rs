use yew::prelude::*;
use yew_router::prelude::*;

use crate::util::routes::{Routes};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class="container-fluid">
            <ul>
                <li>
                    <strong>{ "hyewn" }</strong>
                </li>
            </ul>
            <ul>
                <li>
                    <Link<Routes> to={Routes::Top}>
                        { "top" }
                    </Link<Routes>>
                </li>
                <li>
                    <Link<Routes> to={Routes::New}>
                        { "new" }
                    </Link<Routes>>
                </li>
                <li>
                    <Link<Routes> to={Routes::Show}>
                        { "show" }
                    </Link<Routes>>
                </li>
                <li>
                    <Link<Routes> to={Routes::Ask}>
                        { "ask" }
                    </Link<Routes>>
                </li>
                <li>
                    <Link<Routes> to={Routes::Jobs}>
                        { "jobs" }
                    </Link<Routes>>
                </li>
            </ul>
        </nav>
    }
}