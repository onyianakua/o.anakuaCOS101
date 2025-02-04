fn main() {
    // Mutable array
    let mut colors = ["red", "green", "yellow", "white"];
    println!("\nOriginal array = {:?}", colors);

    // Mutable slice
    let sliced_colors = &mut colors[1..3];
    println!("First slice = {:?}", sliced_colors);

    // Change the value of the mutable slice at the first index
    sliced_colors[1] = "purple";
    println!("Changed slice = {:?}", sliced_colors);

    // Display the original array to show the change
    println!("Modified original array = {:?}", colors);
}
