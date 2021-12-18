use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    ask::Ask,
    jobs::Jobs,
    new::New,
    page_not_found::PageNotFound,
    show::Show,
    top::Top,
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Routes {
    #[at("/")]
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


pub fn switch(routes: &Routes) -> Html {
    match routes.clone() {
        Routes::Ask => {
            html! { <Ask /> }
        }
        Routes::Jobs => {
            html! { <Jobs /> }
        }
        Routes::New => {
            html! { <New /> }
        }
        Routes::Show => {
            html! { <Show /> }
        }
        Routes::Top => {
            html! { <Top /> }
        }
        Routes::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}