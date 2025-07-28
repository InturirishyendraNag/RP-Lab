fn main() {
    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice_2nd_3rd = &arr[1..3];
    println!("Slice of 2nd and 3rd elements: {:?}", slice_2nd_3rd);
    let omit_start = &arr[..5];
    println!("Omit the start index of the slice: {:?}", omit_start);
    let omit_end = &arr[5..];
    println!("Omit the end index of the slice: {:?}", omit_end);
    let omit_both = &arr[..];
    println!("Omit both start and end index of the slice: {:?}", omit_both);
}
