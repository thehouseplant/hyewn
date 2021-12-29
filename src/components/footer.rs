use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="container">
            <small>
                { "Built with " }<a href="https://www.rust-lang.org/">{ "Rust" }</a> 
                { " and " }<a href="https://yew.rs/">{ "Yew" }</a>
                { " • Styles by "}<a href="https://picocss.com/">{ "Pico" }</a>
                { " • " }<a href="https://github.com/thehouseplant/hyewn">{ "Source" }</a>
            </small>
        </footer>
    }
}