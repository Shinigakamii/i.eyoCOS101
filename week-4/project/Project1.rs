use std::io;

fn main() {
    Let mut input1 = String::new();
    Let mut input2 = String::new();

    println!("\nEnter the distance covered by the car (mph): ");

    io::stdin().read_line(&mut input1).expect("Not a valid string");
    Let mut d:f32 = input1.trim().parse().expect("Not a valid number");

     // converting to kilometres

     d = d * 1.609334;
    println!("The distance in kilometres is {}km", d)

    println!("\nEnter the time taken by the car to cover the distance (hrs): ");

    io::stdin().read_line(&mut input2).expect("Not a valid string");
    Let mut t:f32 = input2.trim().parse().expect("Not a valid number");

    let s:f32 = d / t;
    let area = s;

    println!("The speed of the car is {}km/h", area);
}





