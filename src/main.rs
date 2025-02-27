

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
    {}
    // let (enable_fib, max_threshold) = parse();
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
}
