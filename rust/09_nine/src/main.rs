use rand::distributions::{Distribution, Uniform};
use std::thread::JoinHandle;

fn main() {
    let rng = rand::thread_rng(); // Fixed random number generator
    let dist = Uniform::from(1..101);
    let values: Vec<i32> = dist.sample_iter(rng).take(1000).collect(); // Removed reference to rng

    let t: JoinHandle<i32> = std::thread::spawn(move || { // Specified type for JoinHandle
        values.iter().sum()
    });

    let result = t.join().unwrap();
}