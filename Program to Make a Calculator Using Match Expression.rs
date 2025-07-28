fn main() {
    let num1 = 10;
    let num2 = 5;
    let operation = '+';

    let result = match operation {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 != 0 {
                num1 / num2
            } else {
                println!("Cannot divide by zero!");
                return;
            }
        },
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    println!("The result of {} {} {} is: {}", num1, operation, num2, result);
}
