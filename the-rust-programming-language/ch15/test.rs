fn main() {
    // let mut x: i32 = 5;
    // let y: &i32 = &x;
    // *y + 1;
    // println!("x: {}, y: {}", x, y);
    // I want to declare a variable `x` of type `i32`, and then create a reference to it,
    // and then try to change the value of `x` through the reference.
    let mut x: i32 = 5;
    let y: &mut i32 = &mut x;
    *y = 10;
    println!("{}", y);
    // println!("x: {}, y: {}", x, y);
}
