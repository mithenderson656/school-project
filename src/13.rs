
use std::rand::{Rng};

fn main() {
    let mut rng = rand::thread_rng();
    println!("The generated number is: {}", rng.gen_range(1, 10));
}