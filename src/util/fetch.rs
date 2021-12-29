use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub enum Method {
    GET,
}

pub async fn fetch(url: String, method: String) -> Result<JsValue, JsValue> {
    let mut req_opts = RequestInit::new();
    
    req_opts.method(&method);
    req_opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &req_opts)?;
    
    request.headers().set("Accept", "application/json");
    request.headers().set("Content-Type", "application/json");

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

pub struct Fetch();

impl Fetch {
    async fn fetch(url: String, method: Method) -> Result<JsValue, JsValue> {
        let method = match method {
            Method::GET => "GET",
        };

        fetch(url, method.to_string()).await
    }

    pub async fn get(url: String) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::GET).await
    }
}