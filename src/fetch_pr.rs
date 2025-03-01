use std::error::Error;

use tokio;

use crate::extract::fetch_pr_numbers;
#[tokio::main]

pub async   fn fetching_pr(owner:&str, repo: &str, pr_number: u128, github_token: &str)-> Result<Vec<u128>, Box<dyn Error>> {
    let pr_numbers_fetch = fetch_pr_numbers(owner, repo, pr_number, &github_token).await?;
    Ok(pr_numbers_fetch)
}