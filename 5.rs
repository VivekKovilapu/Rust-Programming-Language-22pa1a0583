/*5. Write a program to implement the following
a. Implicit type declaration
b. Explicit type declaration*/
fn main() {
    let a = 10;
    let b = 3.14;
    println!("Implicit types: a = {}, b = {}", a, b);
    let x: i32 = 20;
    let y: f32 = 6.28;
    println!("Explicit types: x = {}, y = {}", x, y);
}
