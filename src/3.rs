fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(1..=100);
    println!("Random number: {}", random_number);
}
