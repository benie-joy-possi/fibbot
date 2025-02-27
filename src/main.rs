use std::env;

fn main() {
    println!("Hello, World!");
    let enable_fib = env::args().nth(1).unwrap_or_else(|| "false".to_string());

    let max_threshold = env::args().nth(2).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1000);

   
    println!("enable_fib: {:?}, max_threshold: {:?}", enable_fib, max_threshold)
}
