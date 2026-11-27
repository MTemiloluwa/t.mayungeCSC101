use std::io::Read;
use std::io;
fn show_help(){
    println!("\n------Application User Guide------");
    println!("It's pretty simple really.");
    println!("This app just basically shows your table in the company's database based on your role in the company.");
    println!("The Administrator can access the entire database structure.");
    println!("If you are a Project Manager you only have access to the project table");
    println!("If you are an Employee you have access to the staff table");
    println!("If you are a Customer you have access to the data-plan table");
    println!("Just choose one of the letters based on your respected role");
    println!("Okay, you can get into it now.");
}
fn main() {
    println!("Hello there");
    println!("This is this Database Structures app");
    
    loop{
       println!("----Once again, Welcome to the Database Structures app----");
       println!("Please indicate what role you play in the company (or type 'help', or 'quit')");
       println!("Type A if you are aan administrator");
       println!("Type P if you are a project manager");
       println!("Type E if you are an employee");
       println!("Type C if you are a customer");
       println!("Type V if you are a vendor");
       let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read");
        let choice = user_input.trim();

        if choice == "help" {
        show_help();
        continue;

         } 

        if choice == "quit" {
            println!("\nThank you for using our app. Exiting the program now... Goodbye!👋");
            break; 
        }    

         if choice == "A" {
            println!("This is the entire database structure");
            let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            print!("{}", contents);
         } 

         if choice == "P" {
            println!("This is the project table");
            let mut file = std::fs::File::open("project_tb.sql").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            print!("{}", contents);
         } 

         if choice == "C" {
            println!("This is the customer table");
            let mut file = std::fs::File::open("customer_tb.sql").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            print!("{}", contents);
         }

         if choice == "E" {
            println!("This is the employee table");
            let mut file = std::fs::File::open("staff_tb.sql").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            print!("{}", contents);
         }

         if choice == "V"{
            println!("This is the dataplan table for vendors");
            let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            print!("{}", contents);
         }

         else{
            println!("Role not found. Please go confirm your role");
            println!("Returning you to the homepage now");
            continue;
         }


    }
    
     
}
