use std::io;
fn add(a: i8, b:i8) {
    let sum = a + b;

    println!("Sum of a and b = {}", sum);
}

fn main() {
    let mut input1 = String::new();
    println!("Enter input for parameter A:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i8 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter input for parameter B:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i8 = input2.trim().parse().expect("Invalid input");

    //call add function with arguments
    add(a, b);


}
