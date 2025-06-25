/*4. Write a program to implement the Scope and Shadowing*/
fn main() {
    let x = 10;
    println!("Outside block: x = {}",x);
    {
        let x = 20;
        println!("Inside block: x = {}",x);
    }
    println!("main scope: x = {}",x);
    let x = x+5;
    println!("shadowing: x = {}",x);
}
