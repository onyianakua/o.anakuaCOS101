use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    // Create a lookup table (APS level + Profession -> Role)
    let mut role_table: HashMap<(&str, &str), &str> = HashMap::new();

    // APS-1
    role_table.insert(("APS-1", "Office Administrator"), "Administrator");
    role_table.insert(("APS-1", "Academic"), "Research Assistant");
    role_table.insert(("APS-1", "Lawyer"), "Junior Associate");
    role_table.insert(("APS-1", "Teacher"), "Classroom Teacher");

    // APS-2
    role_table.insert(("APS-2", "Office Administrator"), "Administrator");
    role_table.insert(("APS-2", "Academic"), "PhD Candidate");
    role_table.insert(("APS-2", "Lawyer"), "Associate");
    role_table.insert(("APS-2", "Teacher"), "Instructor");

    // EL19.3
    role_table.insert(("EL19.3", "Office Administrator"), "Secretary");
    role_table.insert(("EL19.3", "Academic"), "Head of Department");
    role_table.insert(("EL19.3", "Lawyer"), "Principal Associate");
    role_table.insert(("EL19.3", "Teacher"), "Senior Teacher");

    // BSA23.1
    role_table.insert(("BSA23.1", "Office Administrator"), "Director");
    role_table.insert(("BSA23.1", "Academic"), "Dean");
    role_table.insert(("BSA23.1", "Lawyer"), "Senior Lawyer");
    role_table.insert(("BSA23.1", "Teacher"), "Lead Teacher");

    // SES
    role_table.insert(("SES", "Office Administrator"), "CEO");
    role_table.insert(("SES", "Academic"), "Dean");
    role_table.insert(("SES", "Lawyer"), "Senior Lawyer");
    role_table.insert(("SES", "Teacher"), "Director Teacher");

    // Prompt user for APS level
    print!("Enter your APS level (e.g. APS-1, APS-2, EL19.3, BSA23.1, SES): ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut aps_input = String::new();
    io::stdin().read_line(&mut aps_input).expect("Failed to read APS level");
    let aps_input = aps_input.trim();

    // Prompt user for profession
    print!("Enter your profession (Office Administrator, Academic, Lawyer, Teacher): ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut profession_input = String::new();
    io::stdin().read_line(&mut profession_input).expect("Failed to read profession");
    let profession_input = profession_input.trim();

    // Lookup the role
    match role_table.get(&(aps_input, profession_input)) {
        Some(role) => println!("\nFor APS level '{}' as a '{}', the role is: {}", aps_input, profession_input, role),
        None => println!("\nNo matching role found for APS level '{}' and profession '{}'.", aps_input, profession_input),
    }
}

