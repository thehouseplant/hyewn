use serde_json::json;
use std::collections::HashMap;

use super::fetch::Fetch;
use super::models::{Status, IUser};

use crate::API_URL;

pub async fn get_user(id: &str) -> Result<IUser, Status> {
    let mut url = String::with_capacity(API_URL.len() + 6 + id.len());
    url.push_str(API_URL);
    url.push_str("/user/");
    url.push_str(&id);

    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(json.into_serde().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}