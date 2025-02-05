use std::io::{self, Read};

fn main() {
    let role: u32 = get_input("
What is your role? (Enter 1-5)
1. Administrator
2. Project Manager
3. Employee
4. Customer
5. Vendor
").parse().unwrap();
    match role {
        1 => print_admin(),
        2 => print_project_manager(),
        3 => print_emp(),
        4 => print_customer(),
        5 => print_vendor(),
        _ => panic!("Invalid role"),
    }
}

fn print_admin() {
    let admin_pass = get_input("Enter administrator password:");
    if admin_pass != "elsie"{
        println!("Invalid password");
    }
    println!("Database Structure:");
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn print_project_manager() {
    println!("Project Table Structure:");
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn print_emp() {
    println!("Staff Table Structure:");
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn print_customer() {
    println!("Customer Table Structure:");
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn print_vendor() {
    println!("Data Plan Table Structure:");
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
