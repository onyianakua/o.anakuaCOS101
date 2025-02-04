use std::io;

fn main() {
    let mut input = String::new();

    // Input experience
    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut input).unwrap();
    let experienced = input.trim().to_lowercase() == "yes";
    input.clear();

    // Input age
    println!("Enter the age of the employee:");
    io::stdin().read_line(&mut input).unwrap();
    let age: u32 = input.trim().parse().unwrap();

    // Determine the incentive
    let incentive = if experienced {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            0 // Default in case of unexpected conditions
        }
    } else {
        100_000
    };

    println!("The annual incentive of the employee is: ₦{:.2}", incentive);
}

