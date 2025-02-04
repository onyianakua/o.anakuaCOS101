use std::collections::HashMap;
use std::io;

fn main() {
    // Define the menu and prices
    let mut menu: HashMap<&str, i32> = HashMap::new();
    menu.insert("P", 3200); // Poundo Yam/Edinkaiko Soup
    menu.insert("F", 3000); // Fried Rice & Chicken
    menu.insert("A", 2500); // Amala & Ewedu Soup
    menu.insert("E", 2000); // Eba & Egusi Soup
    menu.insert("W", 2500); // White Rice & Stew

    // Display the menu
    println!("Welcome to the Restaurant!");
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    let mut total = 0;

    loop {
        let mut food_type = String::new();
        let mut quantity = String::new();

        // Get the type of food
        println!("\nEnter the type of food (P, F, A, E, W) or 'done' to finish:");
        io::stdin().read_line(&mut food_type).unwrap();
        let food_type = food_type.trim().to_uppercase();

        if food_type == "DONE" {
            break;
        }

        if !menu.contains_key(food_type.as_str()) {
            println!("Invalid food type. Please try again.");
            continue;
        }

        // Get the quantity
        println!("Enter the quantity:");
        io::stdin().read_line(&mut quantity).unwrap();
        let quantity: i32 = match quantity.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid quantity. Please enter a valid number.");
                continue;
            }
        };

        // Calculate the cost for this order
        let price = menu.get(food_type.as_str()).unwrap();
        total += price * quantity;

        println!(
            "Added {} of {} to the order. Current total: N{}",
            quantity, food_type, total
        );
    }

    // Apply discount if applicable
    if total > 10_000 {
        let discount = total as f64 * 0.05;
        total = (total as f64 - discount) as i32;
        println!("A discount of 5% has been applied!");
    }

    println!("\nFinal total: N{}", total);
    println!("Thank you for your order!");
}

