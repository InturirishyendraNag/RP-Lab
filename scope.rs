fn main() {
    let x = 10;
    println!("Global x: {}", x);
    {
        let x = 20;
        println!("Inner scope x: {}", x);
        {
            let x = 30;
            println!("Innermost scope x: {}", x);
        }
        println!("Inner scope x after innermost: {}", x);
    }
    println!("Global x after inner scopes: {}", x);
}
