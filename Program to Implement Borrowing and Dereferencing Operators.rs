fn main() {
    let x = 10; // Declare a variable
    let y = &x; // Borrowing a reference to `x`

    println!("Value of x: {}", x);  // Direct access
    println!("Value of y (borrowed reference to x): {}", *y); // Dereferencing y to get the value of x

    let z = Box::new(20);  // Heap-allocated value
    let w = &z;  // Borrowing a reference to the boxed value

    println!("Value inside box z: {}", *w); // Dereferencing the reference w
}
