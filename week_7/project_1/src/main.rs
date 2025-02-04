use std::io;
use std::io::Write; // For flush() if needed

// 1. Area of a trapezium
//    Formula: A = ( (base1 + base2) / 2 ) * height
fn area_trapezium(base1: f64, base2: f64, height: f64) -> f64 {
    ((base1 + base2) / 2.0) * height
}

// 2. Area of a rhombus
//    Formula: A = (diagonal1 * diagonal2) / 2
fn area_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    (diagonal1 * diagonal2) / 2.0
}

// 3. Area of a parallelogram
//    Formula: A = base * height
fn area_parallelogram(base: f64, height: f64) -> f64 {
    base * height
}

// 4. Surface area of a cube
//    Formula: A = 6 * side^2
fn area_cube(side: f64) -> f64 {
    6.0 * side * side
}

// 5. Volume of a cylinder
//    Formula: V = π * r^2 * height
fn volume_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}

fn main() {
    loop {
        println!("\n--- Geometry Calculator ---");
        println!("1) Area of Trapezium");
        println!("2) Area of Rhombus");
        println!("3) Area of Parallelogram");
        println!("4) Surface Area of Cube");
        println!("5) Volume of Cylinder");
        println!("6) Exit");

        // Prompt the user to choose a calculation
        print!("Enter your choice: ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Read choice from user
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number (1-6).");
                continue;
            }
        };

        match choice {
            1 => {
                // Trapezium
                let base1 = read_value("Enter base1: ");
                let base2 = read_value("Enter base2: ");
                let height = read_value("Enter height: ");
                let area = area_trapezium(base1, base2, height);
                println!("Area of Trapezium = {}", area);
            }
            2 => {
                // Rhombus
                let d1 = read_value("Enter diagonal1: ");
                let d2 = read_value("Enter diagonal2: ");
                let area = area_rhombus(d1, d2);
                println!("Area of Rhombus = {}", area);
            }
            3 => {
                // Parallelogram
                let base = read_value("Enter base: ");
                let height = read_value("Enter height: ");
                let area = area_parallelogram(base, height);
                println!("Area of Parallelogram = {}", area);
            }
            4 => {
                // Cube surface area
                let side = read_value("Enter side length of the cube: ");
                let area = area_cube(side);
                println!("Surface Area of the Cube = {}", area);
            }
            5 => {
                // Cylinder volume
                let radius = read_value("Enter radius of the cylinder: ");
                let height = read_value("Enter height of the cylinder: ");
                let volume = volume_cylinder(radius, height);
                println!("Volume of the Cylinder = {}", volume);
            }
            6 => {
                // Exit the loop
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}

// Helper function to read a floating-point value from standard input
fn read_value(prompt: &str) -> f64 {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    match input.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number. Defaulting to 0.0");
            0.0
        }
    }
}

