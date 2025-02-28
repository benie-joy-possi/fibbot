
mod extract;
use extract::extract_numerical_values;
use parse::parse;
mod parse;
fn main() {
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
        assert_eq!(numbers, vec![1, 2, 1,2, 8]);
    }
}
