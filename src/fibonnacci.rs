pub fn fibonacci(num: u128) -> u128 {
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
