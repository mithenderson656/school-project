
// Rust program to calculate the factorial of a given number
fn main() {
    println!("Please enter a positive integer: ");

    // Read input from the user
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Convert the input to an integer
    let num = input.trim().parse::<i32>().unwrap();

    // Calculate the factorial of the number
    let result = factorial(num);

    // Print the result
    println!("The factorial of {} is {}", num, result);
}

// Function to calculate the factorial of a given number
fn factorial(n: i32) -> i32 {
    if n == 0 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}