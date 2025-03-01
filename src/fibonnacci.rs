pub fn fibonacci1(num: u128) -> u128 {
    if num == 0 {
        return 0;
    } else if num  == 1 {
        return 1;
    }
    let mut num_a = 0;
    let mut num_b = 1;
    for _ in 2..=num {
        let temporal = num_a + num_b;
        num_a = num_b;
        num_b = temporal
    }
    num_b
}
pub fn fibonacci(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a: u128 = 0;
            let mut b: u128 = 1;
            for _ in 2..=n {
                match a.checked_add(b) {
                    Some(result) => {
                        a = b;
                        b = result;
                    }
                    None => {
                        
                        panic!("Fibonacci calculation overflowed for n={}", n);
                    }
                }
            }
            b
        }
    }
}
