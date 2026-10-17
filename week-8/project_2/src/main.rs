use std::io;
//Create a simple help guide for our application
fn show_help(){
    println!("   WELCOME TO THE EY NIGERIAN TOOL USER HELP GUIDE   ");
    println!("To add candidates typr 'add'. This will allow you input developer's name and years of work ");
    println!("to check the top devoloper type 'find'. This will scan the input and find the most experienced devoloper.");
    println!("To view all candidates type 'list'");
    println!("If you get stuck while operating the program type 'help' to summon me");
    println!("To exit program and any point simply type 'quit'");
    println!("I will leave you to it now. Remember if you get stuck at any point I am here");
    println!("I hope you understood. Bye");
}
//Like we did in our last project we are going to create a slice that walks with a loop
// This will keep track of the most experienced candidate so far
fn find_top_developer(candidates_slice: &[(String, u8)]){
    if candidates_slice.is_empty() {
        println!("\n No candidates currently in the system to evaluate.\n");
        return; // this takes you back to the home page
    }
    //Initialize tracking variables with the first candidate
    //We do this so that the we have a baseline and a number we can use to compare with the other developer
    //By doing this the loop has an accurate and realistic baseline to start comparing against right away
    let mut top_name = &candidates_slice[0].0;
    let mut max_years = candidates_slice[0].1;
     // walk it through a loop Thos is the loop I was talking about above before you start the main function
     //a loop ought to be running already as we go on you will see where we would need this
     for candidate in candidates_slice{
        if candidate.1 > max_years{
            max_years = candidate.1;
            top_name = &candidate.0;
        }
     }
     println!("\n ------------------------------------------------");
     println!("MOST EXPERIENCED DEVELOPER FOUND!");
     println!("Candiate's name is {}", top_name);
     println!("Number of years of experience is {}", max_years);
     println!("Current Status: Selected for the EY Nigeria Senior Lead Role");
     println!("Goal has been achieved");
     println!("----------------------------------------------------");

}
// Now we start our main function
fn main(){
    println!("----------------------------------------------------");
    println!("   WELCOME USER TO THE EY INTERVIEW TOOL   ");
    println!("Our main project is to find the devoloper with the hightest years of experience");
    println!("Let's get into it");
    println!("----------------------------------------------------");

    //Creating a vector of tuples for the list of candidates
    let mut candidates: Vec<(String, u8)> = vec![
        (String::from("Adebayo Oluwaseun"), 5),
        (String::from("Chinedu Eze"), 8),
        (String::from("Fatima Abdullahi"), 12),
        (String::from("Blessing Okon"), 3),
        (String::from("Tunde Bakare"), 10),
        (String::from("Abdul Cassandra"),2),
        (String::from("Mayungbe Temiloluwa"), 15),
    ];
    //Now we are going to create our main loop which would serve as our Home page
    loop{
        println!("--------------------------------------------------");
        println!("Hello there, User. This is the EY recruitment tool.");
        println!("Choose from the menu bar below");
        println!("Menu: Choose or type 'add', 'find', 'list', 'help', or 'quit': ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice = input.trim().to_lowercase();

        //To exit program
        if choice == "quit" {
            println!("\nThank you for using the EY Recruitment Tool. Goodbye!");
            std::process::exit(0);
        }

        if choice == "help"{
            show_help();
            continue;
        }
        if choice == "list"{
            println!("\n--- CURRENT REGISTERED CANDIDATES FOR ROLE ---");
            for (idx, c) in candidates.iter().enumerate() {
                println!("{}. {:<20} | {} years experience", idx + 1, c.0, c.1); 
            } 
         println!();
            continue 
        }

         if choice == "find"{
        //We are going to pass the vector as a slice into our custom made helper function above 
        //Recall find_to_developer tool function that we made above
        find_top_developer(&candidates);
        continue;
        }
        
        if choice == "add"{
            println!("\nEnter Candidate's Name (or type 'help', 'quit'): ");
            let mut name_in = String::new();
            io::stdin().read_line(&mut name_in).expect("Failed to read");
            let name = name_in.trim().to_string();

            if name.to_lowercase() == "quit"{
                println!("\nThank you for using the EY Recruitment Tool. Goodbye!");
                std::process::exit(0);
            }
            if name.to_lowercase() == "help" {
                show_help();
                continue;
            }
            println!("Enter Years of Experience (number): ");
            let mut years_in = String::new();
            io::stdin().read_line(&mut years_in).expect("Failed to read");
            let years_str = years_in.trim();

            if years_str.to_lowercase() == "quit" {
                println!("\nThank you for using the EY Recruitment Tool. Goodbye!");
                std::process::exit(0);
            }
            if years_str.to_lowercase() == "help" {
                show_help();
                continue;
            }
            // We are using if and else functions
            let years: u8;
            if years_str.parse::<u8>().is_ok() {
                years = years_str.parse().expect("Input a valid number");
            } else {
                println!("\nInvalid number entered. Returning to menu...\n");
                continue;
            }

            // Push name(string) and years(u8) directly into the vector created for candidates above
            candidates.push((name.clone(), years));
            println!("\n Candidate '{}' added successfully!\n", name);
            continue;
        }
       println!("\n Option not recognized. Type 'help' for instructions.\n"); 
    }
}

