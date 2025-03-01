use std::error::Error;

use regex::Regex;

pub fn extract_numerical_values(content: &str) -> Vec<u128> {
    // Define a regular expression to match numerical values
    let re = Regex::new(r"\b\d+\b").unwrap();

    // Find all matches and parse them to integers
    re.find_iter(content)
        .filter_map(|mat| mat.as_str().parse().ok())
        .collect()
}
pub async fn fetch_pr_numbers(
    repo: &str,
    pr_number: u128,
    github_token: &str,
) -> Result<Vec<u128>, Box<dyn Error>> {
    println!("Repository: {}", repo);
    println!("Pull Request Number: {}", pr_number);
    println!("GitHub Token: {}", github_token);
    let repo = repo.split("/").collect::<Vec<&str>>();
    let owner = repo[0];
    let repo = repo[1];
    println!("owner {} repo {}", owner, repo);

    let value = octocrab::instance()
        .pulls(owner, repo)
        .list_files(1)
        .await?;
    let octocrab = &value.items.first().unwrap().patch.clone().unwrap();
    // let body = octocrab.unwrap_or_default();

    let numbers = extract_numerical_values(&octocrab);
    println!("numbers {:?}", numbers);
    Ok(numbers)
}
