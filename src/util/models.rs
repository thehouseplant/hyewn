use serde::Deserialize;

#[derive(Deserialize)]
pub struct Ask {
    pub by: String,
    pub descendants: usize,
    pub id: usize,
    pub score: usize,
    pub text: String,
    pub time: usize,
    pub title: String,
    pub post_type: String,
}

#[derive(Deserialize)]
pub struct Comment {
    pub by: String,
    pub id: usize,
    pub parent: usize,
    pub text: String,
    pub time: usize,
    pub post_type: String,
}

#[derive(Deserialize)]
pub struct Job {
    pub by: String,
    pub id: usize,
    pub score: usize,
    pub text: String,
    pub time: usize,
    pub title: usize,
    pub post_type: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct Poll {
    pub by: String,
    pub descendants: usize,
    pub id: usize,
    pub parts: Vec<PollPart>,
    pub score: usize,
    pub text: String,
    pub time: usize,
    pub title: String,
    pub post_type: String,
}

#[derive(Deserialize)]
pub struct PollPart {
    pub by: String,
    pub id: usize,
    pub poll: usize,
    pub score: usize,
    pub text: String,
    pub time: usize,
    pub post_type: String,
}

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

#[derive(Deserialize)]
pub struct Story {
    pub by: String,
    pub descendants: usize,
    pub id: usize,
    pub score: usize,
    pub time: usize,
    pub title: String,
    pub post_type: String,
    pub url: String,
}