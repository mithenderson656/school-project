use std::io;
use std::io::prelude::*;
fn main() -> io::Result<()> {
let mut reader = io::BufReader::new(io::stdin());
let mut writer = io::stdout();
loop {
let mut line = String::new();
match reader.read_line(&mut line) {
Ok(_) => match line.trim().parse::<i32>() {
Ok(n) if n > 0 => println!("{}", n * 2),
Err(_) => println!("Invalid input"),
},
Err(e) => println!("Error reading line: {}", e),
},
Err(e) => println!("Error writing to stdout: {}", e),
}
break;
}
Ok(())
}