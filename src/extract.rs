use std::error::Error;

use octocrab::Octocrab;
use regex::Regex;

pub fn extract_numerical_values(content: &str) -> Vec<u64> {
    // Define a regular expression to match numerical values
    let re = Regex::new(r"\b\d+\b").unwrap();

    // Find all matches and parse them to integers
    re.find_iter(content)
        .filter_map(|mat| mat.as_str().parse().ok())
        .collect()
}
pub async fn fetch_pr_numbers(owner:&str, repo: &str, pr_number: u64) -> Result<Vec<u64>, Box<dyn Error>> {
    let octocrab = Octocrab::builder().build()?;
    let pr = octocrab.pulls(owner,repo).get(pr_number).await?;
    let body = pr.body.unwrap_or_default();

    let numbers = extract_numerical_values(&body);
    Ok(numbers)
}

