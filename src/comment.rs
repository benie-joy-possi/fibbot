use reqwest::{Client, Error};
use serde::Serialize;
#[derive(Serialize)]
struct Comment {
    body: String,
}

#[tokio::main]
pub async fn post_comment(
    owner: &str,
    repo: &str,
    pr_number: u128,
    github_token: String,
    comments: String,
) -> Result<String, Error> {
 

    let client = Client::new();

    let comment = Comment { body: comments };

    let response = client
        .post(format!(
            "https://api.github.com/repos/{owner}/{repo}/issues/{pr_number}/comments"
        ))
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", github_token).as_str())
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", format!("{}", owner).as_str())
        .json(&comment)
        .send()
        .await?;

    println!("{:#?}", response);

    Ok(comment.body)
}