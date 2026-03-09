fn fib(n: u32) -> u32 {
    let mut a = 0u32;
    let mut b = 1u32;
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

fn main() {
    let n = 10;
    println!("fib({n}) = {}", fib(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_cases() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn test_sequence() {
        let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        for (n, &val) in expected.iter().enumerate() {
            assert_eq!(fib(n as u32), val);
        }
    }
}
