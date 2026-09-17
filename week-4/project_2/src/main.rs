//Using Rust to determine annual incentive

use std::io;

fn main() {
    println!("\nThe Incentive Calculator");

    // Know experience status of employees
    println!("Is the employee experienced? (yes/no): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let exp_input = input.trim().to_lowercase();


    

    if exp_input == "yes" || exp_input == "y"{
        // Input age for experied employees
        println!("\nEnter your age.");
        let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
         let age:u8 = age.trim().parse().expect("Failed to input");
         println!("Your age is: {}", age);  

        if age >= 40{
        println!("Incentive: N1_560_000 per month");
    }
    else if age >= 30 && age <= 39 
    {
        println!("Incentive: N1_480_000");
    }
    else if age < 28 {
        println!("Incentive: N1_300_000");
    }
    else {
        println!("Incentive: default experienced teir rate applies");
    }
    }
        

    
    
    else {
        println!("Incentive: N100_000");
    }

}



