use std::io::{self, Write};

// A simple struct to hold developer information
#[derive(Debug)]
struct Developer {
    name: String,
    years_of_experience: u32,
}

fn main() {
    println!("=== EY Project: Find the Most Experienced Developer ===");

    // Prompt the user for how many developers to input
    let mut num_devs_input = String::new();
    print!("How many developers do you want to enter? ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut num_devs_input)
        .expect("Failed to read line");

    let num_devs: usize = match num_devs_input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number, defaulting to 0 developers.");
            0
        }
    };

    // If no developers are entered, we simply exit
    if num_devs == 0 {
        println!("No developers entered. Exiting program...");
        return;
    }

    // Vector to store all Developer records
    let mut developers = Vec::new();

    // Read each developer's data from user input
    for i in 1..=num_devs {
        println!("\n--- Enter details for Developer #{} ---", i);

        // Read name
        print!("Enter name: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut name_input = String::new();
        io::stdin()
            .read_line(&mut name_input)
            .expect("Failed to read line");
        let name = name_input.trim().to_string();

        // Read years of experience
        print!("Enter years of experience (integer): ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut years_input = String::new();
        io::stdin()
            .read_line(&mut years_input)
            .expect("Failed to read line");

        let years_of_experience: u32 = match years_input.trim().parse() {
            Ok(y) => y,
            Err(_) => {
                println!("Invalid input, defaulting to 0 years of experience.");
                0
            }
        };

        // Push a new Developer struct into the vector
        developers.push(Developer {
            name,
            years_of_experience,
        });
    }

    // Find the maximum years of experience among all developers
    let max_experience = developers
        .iter()
        .map(|dev| dev.years_of_experience)
        .max()
        .unwrap_or(0);

    // Filter to get all developers who match that max experience
    let most_experienced: Vec<&Developer> = developers
        .iter()
        .filter(|dev| dev.years_of_experience == max_experience)
        .collect();

    // Display results
    if most_experienced.is_empty() {
        println!("\nNo developers found (or all have 0 years).");
    } else {
        println!("\n=== Result: Most Experienced Developer(s) ===");
        for dev in &most_experienced {
            println!(
                "{} with {} years of experience",
                dev.name, dev.years_of_experience
            );
        }
    }

    println!("\nThank you for using the EY Project Finder!");
}

