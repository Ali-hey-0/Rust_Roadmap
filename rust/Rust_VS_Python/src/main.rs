use std::io::Write;
use std::io::{stdin, stdout};
use std::time::{Duration, Instant};
use num_bigint::BigUint;

fn fib(n: u32) -> BigUint {
    let mut n1: BigUint = BigUint::from(1u32);
    let mut n2: BigUint = BigUint::from(1u32);

    for _ in 3..=n {
        let temp = n2.clone();
        n2 = &n1 + &n2;
        n1 = temp;
    }
    n2
}

fn main() {
    let mut buf: String = String::new();
    print!("N: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut buf).expect("failed to read input");

    let n: u32 = buf.trim().parse().expect("failed to convert to integer");
    let start: Instant = Instant::now();
    let r: BigUint = fib(n);
    let duration: Duration = start.elapsed();
    println!("{}", r);
    println!("{:?}", duration);
}