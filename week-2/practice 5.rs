fn my_grade(){

	// function body
	println!("Greetings from function my_grade!");
}

fn main()	{

	//calling a function
	my_grade();
}
use std::io

fn checker(){

   let mut input = String::new();
   println!("Enter a character:");
   io::stdin().read_line(&mut input).expect("Failed to read input");
   let ch:char = input.trim().parse().expect("Invalid input");

   if ch >= '0' && ch <= '9'
   {
   	  println!("Character '{}' is a digit",ch);
   }
   else 
   {
   	   println!("Character '{}' is not a digit",ch); 	
   	  }	
}
fn main() {
	// calling function
	println!("Welcome! This program checks whether a character variable contsins a digit or not");
	checker()
} 
fn main(){
   println!("pi value is {}",get_pi());  
}

fn get_pi()->f64 {
   let a:f64 = 22.0;
   let b:f64 = 7.0;
   let c:f64 = a/b;
   return c;
}
use std::io;

fn add(a: i32, b: i32) {
	let sum = a = b;
	

