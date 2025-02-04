fn main() {
    let a: i32 = 10;
    let b: i32 = 11;

    let mut result: i32;

    result = a & b;
    println!("a & b = {}", result);

    result = a | b;
    println!("a | b = {}", result);

    result = a ^ b;
    println!("a ^ b = {}", result);

    result = !a;
    println!("!a = {}", result);

    result = a << b;
    println!("a << b = {}", result);

    result = a >> b;
    println!("a >> b = {}", result);
}
