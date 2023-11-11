use std::io;

fn main()
{
    println!("Enter lower bound");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let lowerbound:i32 = input1.trim().parse().expect("Failed to input");

    println!("Enter upper bound");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let upperbound:i32 = input2.trim().parse().expect("Failed to input");

    for x in lowerbound..upperbound{

        println!("Count Level is {}", x);
    }


}