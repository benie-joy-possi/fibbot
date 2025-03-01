// mod comment;
mod extract;
mod fibonnacci;
use fetch_pr::fetching_pr;
use std::env;
mod comment;
use comment::post_comment;
use extract::extract_numerical_values;
use fibonnacci::fibonacci;
use parse::parse;
mod fetch_pr;
mod parse;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 4 {
        eprintln!(
            "Usage: {} <owner> <repo> <pr_number> <github_token>",
            args[0]
        );
        return;
    }
    let owner = &args[5];
    let repo = &args[2];
    let pr_number: u128 = args[3].parse().expect("Failed to parse pr_number");
    let github_token = args[4].as_str();
    println!("{:?} args", args);

    let repo = repo.split("/").collect::<Vec<&str>>();
    let repo = repo[1];

    let mut comments = String::new();
    let pr_numbers_fetch = fetching_pr(owner, repo, pr_number, github_token).unwrap();

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
                    if num < max_threshold {
                        let fib_numb = fibonacci(num);
                        let comment = format!("The fibonnaci number of {} is  : {}", num, fib_numb);

                        comments.push_str(format!("{}\n", comment).as_str());

                        let fibonnacci_result = fibonacci(num);
                        println!("the fibonacci is : {}", fibonnacci_result);
                    }
                }
                let _ = post_comment(
                    &owner.to_string(),
                    &repo.to_string(),
                    pr_number,
                    github_token.to_string(),
                    comments,
                );
            } else {
                println!("fibonacci generation has been disabled");
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    let sample_content = "This extract function extract 1, 2 or many numbers in a string 1 2 8";
    let numbers = extract_numerical_values(sample_content);
    println!("Extracted numerical values: {:?}", numbers);
}
