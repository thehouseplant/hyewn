use serde::Deserialize;

#[derive(Deserialize)]
pub struct Post {
    pub by: String,
    pub descendants: usize,
    pub id: usize,
    pub score: usize,
    pub time: usize,
    pub title: String,
    pub post_type: String,
    pub url: String,
}