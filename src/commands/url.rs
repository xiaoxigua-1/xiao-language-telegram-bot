use crate::{Cxt, reply};
use teloxide::{RequestError};
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, USER_AGENT};

#[derive(Deserialize, Debug, Serialize)]
struct RepoData {
    name: String,
    full_name: String,
    language: String,
    forks_count: i16,
    open_issues_count: i16,
    stargazers_count: i16,
    subscribers_count: i16,
}

pub async fn github_url(cx: &Cxt) -> Result<(), reqwest::Error> {
    let url = "https://api.github.com/repos/xiaoxigua-1/xiao-Language";
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "".parse().unwrap());
    let res = client.get(url)
        .headers(headers)
        .send().await?
        .json::<RepoData>().await?;

    let output_text = format!("https://github.com/{}\nstar: {}\nopen issues: {}\nsubscribers: {}\nforks: {}",
                              res.full_name,
                              res.stargazers_count,
                              res.open_issues_count,
                              res.subscribers_count,
                              res.forks_count
    );

    reply(&cx, output_text).await;

    Ok(())
}

pub async fn author_info(cx: &Cxt) -> Result<(), RequestError> {
    cx.reply_to("Website: https://www.xiaoxigua.art/\n\
    GitHub: https://github.com/xiaoxigua-1\n\
    Twitter: https://twitter.com/XiguaXiao").await?;

    Ok(())
}
