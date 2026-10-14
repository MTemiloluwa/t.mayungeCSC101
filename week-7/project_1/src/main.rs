
use std::io;

fn main() {
    println!("=== The Shape Calculator ===");
    println!("1. Trapezium (Area)");
    println!("2. Rhombus (Area)");
    println!("3. Parallelogram (Area)");
    println!("4. Cube (Surface Area)");
    println!("5. Cylinder (Volume)");
    println!("Enter your choice (1-5): ");

    // Read the user's menu choice
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

    // Using basic if/else statements 
    if choice == 1 {
        let result = calculate_trapezium();
        println!("The area of the trapezium is: {}", result);
    } else if choice == 2 {
        let result = calculate_rhombus();
        println!("The area of the rhombus is: {}", result);
    } else if choice == 3 {
        let result = calculate_parallelogram();
        println!("The area of the parallelogram is: {}", result);
    } else if choice == 4 {
        let result = calculate_cube();
        println!("The surface area of the cube is: {}", result);
    } else if choice == 5 {
        let result = calculate_cylinder();
        println!("The volume of the cylinder is: {}", result);
    } else {
        println!("Invalid choice! Please restart and select a number between 1 and 5.");
    }
}

// A simple helper function to read numbers from the user
fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse().expect("Please type a valid number!")
}

// 1. Trapezium Area: height / 2 * (base1 + base2)
fn calculate_trapezium() -> f64 {
    let height = read_input("Enter height:");
    let base1 = read_input("Enter base 1:");
    let base2 = read_input("Enter base 2:");
    
    height / 2.0 * (base1 + base2)
}

// 2. Rhombus Area: 1/2 * diagonal1 * diagonal2
fn calculate_rhombus() -> f64 {
    let diagonal1 = read_input("Enter diagonal 1:");
    let diagonal2 = read_input("Enter diagonal 2:");
    
    // Using 0.5 to avoid integer division issues
    0.5 * diagonal1 * diagonal2
}

// 3. Parallelogram Area: base * altitude
fn calculate_parallelogram() -> f64 {
    let base = read_input("Enter base:");
    let altitude = read_input("Enter altitude:");
    
    base * altitude
}

// 4. Cube Surface Area: 6 * side * side
fn calculate_cube() -> f64 {
    let side = read_input("Enter side length:");
    
    6.0 * side * side
}

// 5. Cylinder Volume: pi * radius * radius * height
fn calculate_cylinder() -> f64 {
    let radius = read_input("Enter radius:");
    let height = read_input("Enter height:");
    
    // Using standard approximation for pi
    3.1415926535 * radius * radius * height
}
