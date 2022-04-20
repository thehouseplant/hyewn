use yew::prelude::*;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
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