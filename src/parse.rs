use std::{env, format};

pub fn parse() -> Result<(bool, u128), String> {
    let enable_fib = env::var("enable_fib").unwrap_or_else(|_| "false".to_string());

    let max_threshold = env::var("max_threshold").unwrap_or_else(|_| "1000".to_string());

    let enable_fib: bool = match enable_fib.parse() {
        Ok(val) => val,
        Err(_) => {
           return Err(format!("Invalid input of enable_fib {}", enable_fib));
        }
    };

    let max_threshold: u128 = match max_threshold.parse() {
        Ok(value) => value,
        Err(_) => {
            return Err(format!("Invalid value of max_threshold {}", max_threshold));
         }
    };
    Ok((enable_fib, max_threshold))
}
