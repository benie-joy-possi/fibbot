use regex::Regex;

pub fn extract_numerical_values(content: &str) -> Vec<i32> {
    // Define a regular expression to match numerical values
    let re = Regex::new(r"\b\d+\b").unwrap();

    // Find all matches and parse them to integers
    re.find_iter(content)
        .filter_map(|mat| mat.as_str().parse().ok())
        .collect()
}
