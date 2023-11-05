// Rust program to calculate the area of a triangle given three sides

use std::io;

fn main()
{
	Let mut input1 = String::new();
	Let mut input2 = String::new();
	Let mut input3 = String::new();

	println!("Enter first edge of triangle: ");
	Let a:f32 = input1.trim().parse().expect("Not a valid number");

	println!("Enter second edge of triangle: ");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	Let b:f32 = input2.trim().parse().expect("Not a valid number"); 

	println!("Enter third edge of triangle: ");
	io::stdin().read_line(&mut input3).expect("Not a valid string"); 
	Let c:f32 = input3.trim().parse().expect("Not a valid number");

	Let s:f32 = (a + b + c) / 2.0;
	Let mut area:f32 = s * (s - a) (s - b) (s - c);
	area = area.sqrt();

	println!("Area of a triangle: {}", area);
}
