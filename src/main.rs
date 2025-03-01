// mod comment;
mod extract;
mod fibonnacci;
use std::{env, error::Error};
mod comment;
use comment::post_comment;
use extract::{extract_numerical_values, fetch_pr_numbers};
use fibonnacci::{fibonacci1, fibonacci};
use octocrab::Octocrab;
use parse::parse;
mod parse;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let args: Vec<String> = vec![
    //     "fibonacci".to_string(),
    //     "benie-joy-possi".to_string(),
    //     "benie-joy-possi/fibbot".to_string(),
    //     "3".to_string(),
    //     "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".to_string(),
    // ];
    let args: Vec<String> = env::args().skip(1).collect();
  if args.len() < 4 {
        eprintln!(
            "Usage: {} <owner> <repo> <pr_number> <github_token>",
            args[0]
        );
        return Err("Invalid arguments".into());
    }
    let owner = &args[1];
    let repo = &args[2];
    let pr_number: u128 = args[3].parse()?;
    let github_token = args[4].as_str();

    
    let octocrab = Octocrab::builder()
    .personal_token(github_token)
    .build()?;
    println!("{}token",github_token );

    let pr_numbers_fetch = fetch_pr_numbers(repo, pr_number, &github_token).await?;
    let pr_number_u64: u64 = pr_number.try_into()?;
    let mut comments = String::new();
    
    
    println!("Hello, World!");

    match parse() {
        Ok((enable_fib, max_threshold)) => {
            println!("Hello, World!");

            println!(
                "enable_fib: {:?}, max_threshold: {:?}",
                enable_fib, max_threshold
            );

            if enable_fib {
                println!("fibbonacci genration is up to {}", max_threshold);
                for &num in &pr_numbers_fetch {
                    if num < max_threshold
                     {
                        let fib_numb = fibonacci(num);
                        let comment = format!("The fibonnaci number of {} is  : {}", num, fib_numb);
            
                        comments.push_str(format!("{}\n", comment).as_str());
            
                         let fibonnacci_result = fibonacci(num);
                         println!("the fibonacci is : {}", fibonnacci_result);
                        }
                    }   
                } else {
                    println!("fibonacci generation has been disabled");
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        
        let _ = post_comment(
            &owner.to_string(),
            &repo.to_string(),
            pr_number,
            github_token.to_string(),
            comments,
        );
        let sample_content = "This extract function extract 1, 2 or many numbers in a string 1 2 8";
    let numbers = extract_numerical_values(sample_content);
    println!("Extracted numerical values: {:?}", numbers);
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parse_inputs_true_1000() {
        use std::env;

        env::set_var("enable_fib", "true");
        env::set_var("max_threshold", "1000");
        let result = parse();
        assert!(result.is_ok());

        let (enable_fib, max_threshold) = result.unwrap();
        assert!(enable_fib);
        assert_eq!(max_threshold, 1000);
    }

    #[test]
    fn test_parse_inputs_false_200() {
        use std::env;

        env::set_var("enable_fib", "false");
        env::set_var("max_threshold", "200");
        let result = parse();
        assert!(result.is_ok());

        let (enable_fib, max_threshold) = result.unwrap();
        assert!(!enable_fib);
        assert_eq!(max_threshold, 200);
    }
    #[test]
    fn test_extract_numerical_values() {
        let sample_content = "This extract function extract 1, 2 or many numbers in a string 1 2 8";
        let numbers = extract_numerical_values(sample_content);
        assert_eq!(numbers, vec![1, 2, 1, 2, 8]);
    }
}
