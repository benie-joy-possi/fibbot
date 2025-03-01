use num_bigint::{BigUint, ToBigUint};

/// Computes the nth Fibonacci number using BigInt.
pub fn fibonacci(num: u128) -> BigUint {
    let mut num_a: BigUint = 0.to_biguint().unwrap();
    let mut num_b = 1.to_biguint().unwrap();

    if num == 0 {
        num_a
    } else if num == 1 {
        num_b
    } else {
        for _ in 2..=num {
            let next = &num_a + &num_b;
            num_a = num_b;
            num_b = next;
        }

        num_b
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            fibonacci(100),
            BigUint::from_str("354224848179261915075").unwrap()
        );
        assert_eq!(
            fibonacci(1000),
            BigUint::from_str("6885276750752098298969649928516003704476137795166849228875")
                .unwrap()
        );
    }
}
