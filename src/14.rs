fn main() {
    let mut count = 0;
    loop {
        println!("Hello, world!");
        count += 1;
        if count == 3 {
            break;
        }
    }
}
