use std::io;
fn show_help() { // This makes it possible for the user to always access help when stuck
    println!("\n--- APS CHECKER HELP GUIDE ---");
    println!("It's pretty simple really. This app just checks your Nigerian Public Service APS level.");
    println!("Based on your job title, it tells you your grade band.\n");
    println!("Type 'transition' if you are moving from the private sector or want to check you equivalent APS.\n");
    println!("When asked for job title ensure to type 'Intern', 'Associate' etc exactly with capital letters. ");
    println!("There are 6 six fields and under which there are specific job titles");
    println!(" VALID JOB TITLES BY FIELD:");
    println!("--------------------------------------------------");
    println!("1. Lawyer Field:");
    println!("   Paralegal, Junior Associate, Associate");
    println!("    Senior Associate 1-2, Senior Associate 3-4, Partner\n");
    println!("2. Admin Field:");
    println!("    Intern, Administrator, Senior Administrator");
    println!("    Office Manager, Director, CEO\n");
    println!("3. Academic Field:");
    println!("    - (Entry), Research Assistant, PhD Candidate");
    println!("   Post-Doc Researcher, Senior Lecturer, Dean\n");
    println!("4. Teacher field:");
    println!("    Placement, Classroom Teacher, Senior Teacher");
    println!("   Leading Teacher, Deputy Principal, Principal\n");
    println!("5. Engineering Field:");
    println!("    Junior Developer, Software Engineer, Senior Software Engineer");
    println!("    Engineering Lead, Principal Engineer, CTO\n");
    println!("6. Medicine Field:");
    println!("    House Officer, Medical Officer, Senior Medical Officer");
    println!("    Consultant, Chief Consultant, Medical Director\n");
    println!("==================================================\n");
    println!("Oh and before I forget if you want to exit the app just type 'quit' ");
    println!("I hope you understood. If you did, it has been my honour to help you. Now you can get into it.");
    println!("Have fun😊😉! Oh and if you still need help along the way just type 'help'");

}

// Creating a reusable function that accepts any slice
// Prevents repeating search loops so that code follows golden rule DRY 
// This function takes any track slice and a role name, and returns the APS level.
// I also had to make this code extra flexible so anyone can understand
fn search_field(field_slice: &[(&str, &str)], role_to_find: &str) -> String {
    let mut result = String::from("Not Found");
    let role_lower = role_to_find.to_lowercase();//this will make the search flexible if u get u get

    for entry in field_slice {
        if entry.0.to_lowercase() == role_lower {
            result = String::from(entry.1);
            break; // This ensures that the code breaks once search found
        }
    }

    return result;
}

fn main() {
    println!("    Welcome to the APS Level Checker And Transition Tool     ");
    println!("Please type 'help' if you need to know how to use this tool.\n");

    // So like i made an arrays of Tuples for 6 tracks
    let lawyer_field = [
        ("Paralegal", "APS 1-2"),
        ("Junior Associate", "APS 3-5"),
        ("Associate", "APS 5-8"),
        ("Senior Associate 1-2", "EL1 8-10"),
        ("Senior Associate 3-4", "EL2 10-13"),
        ("Partner", "SES"),
    ];

    let admin_field = [
        ("Intern", "APS 1-2"),
        ("Administrator", "APS 3-5"),
        ("Senior Administrator", "APS 5-8"),
        ("Office Manager", "EL1 8-10"),
        ("Director", "EL2 10-13"),
        ("CEO", "SES"),
    ];

    let academic_field = [
        ("-", "APS 1-2"),
        ("Research Assistant", "APS 3-5"),
        ("PhD Candidate", "APS 5-8"),
        ("Post-Doc Researcher", "EL1 8-10"),
        ("Senior Lecturer", "EL2 10-13"),
        ("Dean", "SES"),
    ];

    let teacher_field = [
        ("Placement", "APS 1-2"),
        ("Classroom Teacher", "APS 3-5"),
        ("Senior Teacher", "APS 5-8"),
        ("Leading Teacher", "EL1 8-10"),
        ("Deputy Principal", "EL2 10-13"),
        ("Principal", "SES"),
    ];

    let engineering_field = [
        ("Junior Developer", "APS 1-2"),
        ("Software Engineer", "APS 3-5"),
        ("Senior Software Engineer", "APS 5-8"),
        ("Engineering Lead", "EL1 8-10"),
        ("Principal Engineer", "EL2 10-13"),
        ("CTO", "SES"),
    ];

    let medicine_field = [
        ("House Officer", "APS 1-2"),
        ("Medical Officer", "APS 3-5"),
        ("Senior Medical Officer", "APS 5-8"),
        ("Consultant", "EL1 8-10"),
        ("Chief Consultant", "EL2 10-13"),
        ("Medical Director", "SES"),
    ];

    // Using a main loop to create home page or tab
    loop {
        println!("\n=======================================================");
        println!("Hello👋 there, user. This is the APS Checker. ");
        println!("--------------------------------------------------");
        println!("Would you like to search for your APS");
        println!("Please enter your Job Title (or type 'help', or 'quit' to exit, or 'transition'): ");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read");
        let choice = user_input.trim().to_lowercase(); // makes the code flexible to any user

        if choice == "quit" {
            println!("\nThank you for using the APS Checker. Exiting the program now... Goodbye!👋");
            break; // this breaks the loop and ends the program
        } 
        
        if choice == "help" {
            show_help();
            continue;
        }

        //
        if choice == "transition" {
            println!("\n PRIVATE SECTOR TRANSITION TOOL ");
            println!("Please enter a number of years of professional work experience, or type 'unsure' or 'help' or 'quit': "); // Your number of work experince will be equal to your APS level
            let mut exp_input = String::new();
            io::stdin().read_line(&mut exp_input).expect("Failed");
            let experience = exp_input.trim().to_lowercase();

            if experience == "quit" {
                println!("\nThank you for using the APS Checker. Goodbye!👋");
                std::process::exit(0); //  This so that the user can instantly exit the program from anywhere
            }

            if experience == "help"{
                show_help();
                continue;
            }

            if experience == "unsure" {
                println!("\nNo problem! Would you like to search by Job Title instead? Type 'yes' or 'no' or 'help' or 'quit': ");
                let mut sub_input = String::new();
                io::stdin().read_line(&mut sub_input).expect("Failed to read input");
                let second_choice = sub_input.trim().to_lowercase();
                
                if second_choice == "quit" {
                    println!("\nThank you for using the APS Checker. Goodbye!👋");
                    std::process::exit(0);
                }

                if second_choice == "help"{
                    show_help();
                    continue;
                }
                
                if second_choice == "yes" {
                    println!("\nWhich field? ('Lawyer', 'Admin', 'Academic', 'Teacher', 'Engineering', 'Medicine') or  type 'help' or 'quit': ");
                    let mut field_input = String::new();
                    io::stdin().read_line(&mut field_input).expect("Failed to read");
                    let field = field_input.trim().to_lowercase();
                    
                    if field == "quit" {
                        println!("\nThank you for using the APS Checker. Goodbye!👋");
                        std::process::exit(0);
                    }
                    if field == "help" {
                        show_help();
                        continue;
                    }

                    println!("What was your exact job title? (or type 'help' or 'quit')");
                    let mut role_in = String::new();
                    io::stdin().read_line(&mut role_in).expect("Failed to read");
                    let role = role_in.trim();

                    if role.to_lowercase() == "quit" {
                        println!("\nThank you for using the APS Checker. Goodbye!👋");
                        std::process::exit(0);
                    }

                    if role.to_lowercase() == "help"{
                        show_help();
                        continue;
                    }

                    // Ensuring that code is flexible using if/else and lowercase
                    let result = if field == "lawyer" {
                        search_field(&lawyer_field, role)
                    } else if field == "admin" || field == "office admin" {
                        search_field(&admin_field, role)
                    } else if field == "academic" {
                        search_field(&academic_field, role)
                    } else if field == "teacher" {
                        search_field(&teacher_field, role)
                    } else if field == "engineering" {
                        search_field(&engineering_field, role)
                    } else if field == "medicine" {
                        search_field(&medicine_field, role)
                    } else {
                        String::from("Field not found. Please type in a correct field")
                    };

                    println!("\n-> Result: {}\n", result);
                } else {
                    println!("\nPlease go confirm your details and come back and come back with a valid answer.");
                    println!("Returning you back to the home page menu...\n");
                    continue;// skips to the home page (the main loop)
                }
            } else {
                // If they typed a number of years, check it using if/else
                let estimated_level;
                
                if experience == "0" || experience == "1" || experience == "2" {
                    estimated_level = "Likely APS 1-2 (Entry Level / 0-2 years)";
                } else if experience == "3" || experience == "4" || experience == "5" {
                    estimated_level = "Likely APS 3-5 (Mid-Level / 3-5 years)";
                } else if experience == "6" || experience == "7" || experience == "8" {
                    estimated_level = "Likely APS 5-8 (Senior Level / 6-8 years)";
                } else {
                    estimated_level = "Likely EL1, EL2, or SES (8+ years of experience)";
                }
                println!("\n-> Experience tier processed successfully.\n");
                println!("\n-> Experience Estimate for {} years: {}\n", experience, estimated_level);
            }
            continue;
        }

        // Direct search using functions to connect slices. This chains the search across all 6 tracks.
        // Instead of writing loops everywhere, we pass our track arrays as slices (`&...`) into the function!
        let mut direct_result = search_field(&lawyer_field, &choice);
        if direct_result == "Not Found" { direct_result = search_field(&admin_field, &choice); }
        if direct_result == "Not Found" { direct_result = search_field(&academic_field, &choice); }
        if direct_result == "Not Found" { direct_result = search_field(&teacher_field, &choice); }
        if direct_result == "Not Found" { direct_result = search_field(&engineering_field, &choice); }
        if direct_result == "Not Found" { direct_result = search_field(&medicine_field, &choice); }

        if direct_result != "Not Found" {
            println!("\n-> Result for '{}': {}\n", choice, direct_result);
        } else {
            println!("\n-> Role '{}' not recognized.", choice);
            println!("Just a tip: Try typing 'help' to check valid titles or check your spelling.\n");
        }
    }
}

