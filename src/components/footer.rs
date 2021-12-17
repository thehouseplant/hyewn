use yew::prelude::*;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer class="container">
                <small>
                    { "Built with " }<a href="https://www.rust-lang.org/">{ "Rust" }</a> 
                    { " and " }<a href="https://yew.rs/">{ "Yew" }</a>
                    { " • Styles by "}<a href="https://picocss.com/">{ "Pico" }</a>
                    { " • " }<a href="https://github.com/thehouseplant/hewn">{ "Source" }</a>
                </small>
            </footer>
        }
    }
}


