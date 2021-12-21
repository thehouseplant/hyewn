use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub by: String,
    pub id: usize,
    pub parent: usize,
    pub text: String,
    pub time: usize,
    pub post_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PollPart {
    pub by: String,
    pub id: usize,
    pub poll: usize,
    pub score: usize,
    pub text: String,
    pub time: usize,
    pub post_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub about: String,
    pub created: usize,
    pub delay: usize,
    pub id: String,
    pub karma: usize,
    pub submitted: Vec<Post>,
}