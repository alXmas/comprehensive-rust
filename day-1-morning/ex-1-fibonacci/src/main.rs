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
