fn main() {
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen_range(0..100);
    println!("The number is {}", x);
}
