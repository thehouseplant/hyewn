use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::JsFuture;

use crate::util::{models::Post};

#[derive(Clone, Debug, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

pub enum FetchState<T> {
    Fetching,
    NotFetching,
    Success(T),
    Failed(FetchError),
}

pub async fn fetch_data(req_type: String, id: Option<String>) -> Result<JsValue, FetchError> {
    let mut req_opts = RequestInit::new();

    req_opts.method("GET");
    req_opts.mode(RequestMode::Cors);

    let base_url = format!("https://hacker-news.firebaseio.com/v0/");
    
    let url_path = match req_type {
        ask => "askstories",
        best => "beststories/",
        item => "item/",
        job => "jobstories/",
        new => "newstories/",
        show => "showstories/",
        top => "topstories/",
        user => "user/",
    };

    let url = base_url + url_path;
    
    let request = Request::new_with_str_and_init(&url, &req_opts)?;

    request.headers().set("Accept", "application/json");

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    let post_info: Post = json.into_serde().unwrap();

    Ok(JsValue::from_serde(&post_info).unwrap())
}