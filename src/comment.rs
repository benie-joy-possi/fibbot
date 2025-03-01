use octocrab::{Octocrab};
use crate::fibonacci;
use tokio;

pub async fn post_fibonacci_comment(
    octocrab: &Octocrab,
    owner: &str,
    repo: &str,
    pr_number: u64,
    n: u128,
) -> Result<(), Box<dyn std::error::Error>> {
    // Calculate Fibonacci number
    let fibo_number = fibonacci(n);

    // Prepare the comment text
    let comment_text = format!("Fibonacci number at position {} is: {}", n, fibo_number);
    let pr_number_u64: u64 = pr_number.try_into()?;
        

    // Post the comment on the PR (PRs are considered issues in GitHub API)
    octocrab
        .issues(owner, repo)
        .create_comment(pr_number_u64, &comment_text)
        .await?;

    println!("Comment posted: {}", comment_text);

    Ok(())
}
