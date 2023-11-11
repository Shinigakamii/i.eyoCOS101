// Rust program to output name and age

use std::10;

fn main() {
    println("\nStudent Information Management System!");

    // input name
    println!("\nPlease Enter your name.");
    Let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input"); println!("Your name is: {}", name);

    // input age
    println("\nEnter your age.");
    Let mut age = String::new();
    io::stdin().read_Line(&mut age).expect("Failed to read input");
    Let age:i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is: {}", age);
}