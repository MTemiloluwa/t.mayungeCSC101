// Making a rust program to find the roots of a quadratic equation

use std::io;
// Read values of a, b and c be given from UI
fn main() {
    println!("\nQuadratic Equation Solver");
    

    println!("Enter a value for a: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Failed to input");


    println!("Enter a value for b: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Failed to input");

    println!("Enter a value for c: ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input3.trim().parse().expect("Failed to input");

    if a==0.0{
        println!("Error: 'a' cannot be 0");
    }
    if b==0.0{
        println!("Error: 'b' cannot be 0");

    }
    if c==0.0{
        println!("Error: 'c' cannot be 0");
    }

    //Calculate the discriminant d
    let d:f32 = (b * b) - (4.0 * a * c);

    println!("\nDiscriminant (d) = {}", d);

    //Decision making mainly based on discriminant

    if d > 0.0{
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("The equation has two distinct roots");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);

    } 
    else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("The equation has exactly one real root");
        println!("Root = {}", root);

    }
    else {
        println!("The equation has no real roots");
    }

}

