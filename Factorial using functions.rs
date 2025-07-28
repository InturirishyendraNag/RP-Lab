use std::io;
fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        (2..=n).product()
    }
}

fn main() {
    println!("Enter a number to find its factorial:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num: u64 = match input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Please enter a valid positive number.");
            return;
        }
    };
    let result = factorial(num);
    println!("The factorial of {} is: {}", num, result);
}
