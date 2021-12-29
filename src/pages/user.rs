use yew::{Callback, function_component, html, Properties};

use crate::util::future::handle_future;
use crate::util::models::{IUser, Status};
use crate::util::users::get_user;

#[derive(Properties, PartialEq)]
pub struct UserProps {
    #[prop_or_default]
    pub id: String,
    pub dispatch_user: Callback<IUser>,
    pub dispatch_error: Callback<bool>,
}

#[function_component(User)]
pub fn user(
    UserProps {
        id, 
        dispatch_user, 
        dispatch_error,
    }: &UserProps,
) -> Html {
    let future = async move { get_user("cryptogogue").await };
    handle_future(future, move |data: Result<IUser, Status>| {
        match data {
            Ok(user) => dispatch_user.emit(user),
            Err(_) => dispatch_error.emit(true),
        };
    });

    html! {
        <h1>{ "User" }</h1>
    }
}