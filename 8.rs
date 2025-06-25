/*Write a program to create an array of 10 elements and implement the following
a. Create a of 2nd and 3rd element
b. Omit the start index of the slice
c. Omit the End Index of the Slice
d. Omit both Start and End Index of the Slice*/
fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("a. 2nd and 3rd element slice: {:?}", &arr[1..3]);
    println!("b. Omit start index: {:?}", &arr[..5]);
    println!("c. Omit end index: {:?}", &arr[5..]);
    println!("d. Omit both start and end: {:?}", &arr[..]);
}
