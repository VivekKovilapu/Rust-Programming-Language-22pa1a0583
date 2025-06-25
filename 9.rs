/*9. Write a program to create different types of constants print it in the output*/
const INTEGER_CONST: i32 = 100;
const FLOAT_CONST: f64 = 3.1415;
const CHAR_CONST: char = 'R';
const STRING_CONST: &str = "Rust Programming";
fn main() {
    println!("Int Const: {}", INTEGER_CONST);
    println!("Float Const: {}", FLOAT_CONST);
    println!("Char Const: {}", CHAR_CONST);
    println!("Str Const: {}", STRING_CONST);
}
