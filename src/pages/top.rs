use yew::prelude::*;

use crate::util::{models::Post};

#[function_component(Top)]
pub fn top() -> Html {
    let posts = vec![
        Post {
            by: "hasheddan".to_string(),
            descendants: 80,
            id: 29594389,
            score: 505,
            time: 1639758512,
            title: "Tokio Console".to_string(),
            post_type: "story".to_string(),
            url: "https://tokio.rs/blog/2021-12-announcing-tokio-console".to_string(),
        },
        Post {
            by: "anonu".to_string(),
            descendants: 38,
            id: 29599132,
            score: 72,
            time: 1639780055,
            title: "Postgres is a great pub/sub and job server (2019)".to_string(),
            post_type: "story".to_string(),
            url: "https://webapp.io/blog/postgres-is-the-answer/".to_string(),
        },
        Post {
            by: "DrHilarius".to_string(),
            descendants: 430,
            id: 29595598,
            score: 1026,
            time: 1639763359,
            title: "Open letter from the BMJ to Mark Zuckerberg".to_string(),
            post_type: "story".to_string(),
            url: "https://www.bmj.com/content/375/bmj.n2635/rr-80".to_string(),
        },
        Post {
            by: "indosauros".to_string(),
            descendants: 62,
            id: 29598860,
            score: 133,
            time: 1639778313,
            title: "I bought 1000 meters of wire to settle a physics debate [video]".to_string(),
            post_type: "story".to_string(),
            url: "https://www.youtube.com/watch?v=2Vrhk5OjBP8".to_string(),
        },
        Post {
            by: "ndrake".to_string(),
            descendants: 210,
            id: 29594983,
            score: 256,
            time: 1639760824,
            title: "Kinesis Advantage 360".to_string(),
            post_type: "story".to_string(),
            url: "https://kinesis-ergo.com/keyboards/advantage360/".to_string(),
        },
    ];

    let posts = posts.iter().map(|post| html! {
        <p>
            <strong>
                <a href={format!("{}", post.url)}>
                    {format!("{}", post.title)}
                </a>
            </strong>
            <br />
            <small>
                {format!("{} points by ", post.score)}
                <a href={format!("/user/{}", post.by)}>
                    {format!("{}", post.by)}
                </a>
                { " | " }
                <a href={format!("/item/{}", post.id)}>
                    {format!("{} comments", post.descendants)}
                </a>
            </small>
        </p>
    }).collect::<Html>();

    html! {
        <>
            <h1>{ "Top" }</h1>

            { posts }
        </>
    }
}