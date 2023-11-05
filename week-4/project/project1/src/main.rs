use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nEnter the distance covered by the car (mph): ");

    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let mut d:f32 = input1.trim().parse().expect("Not a valid number");

     // converting to kilometres

     d = d * 1.609334;
    println!("The distance in kilometres is {}km", d);

    println!("\nEnter the time taken by the car to cover the distance (hrs): ");

    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let t:f32 = input2.trim().parse().expect("Not a valid number");

    let s:f32 = d / t;
    let area = s;

    println!("The speed of the car is {}km/h", area);
}





