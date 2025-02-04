use std::io;

fn main() {
    // Input values of a, b, and c
    let mut input = String::new();

    println!("Enter the coefficient a:");
    io::stdin().read_line(&mut input).unwrap();
    let a: f64 = input.trim().parse().unwrap();
    input.clear();

    println!("Enter the coefficient b:");
    io::stdin().read_line(&mut input).unwrap();
    let b: f64 = input.trim().parse().unwrap();
    input.clear();

    println!("Enter the coefficient c:");
    io::stdin().read_line(&mut input).unwrap();
    let c: f64 = input.trim().parse().unwrap();

    // Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Determine the nature of the roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The equation has two distinct roots: root1 = {:.2}, root2 = {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The equation has one real root: root = {:.2}", root);
    } else {
        println!("The equation has no real roots.");
    }
}

