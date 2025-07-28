fn main() {
    let int_num: i32 = 10;
    let float_num: f64 = int_num as f64; // Casting i32 to f64

    let float_num2: f64 = 3.14;
    let int_num2: i32 = float_num2 as i32; // Casting f64 to i32 (fraction part is lost)

    println!("The integer {} as float: {}", int_num, float_num);
    println!("The float {} as integer: {}", float_num2, int_num2);
}
