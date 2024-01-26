use std::io;

fn main() {
    let mut input1 = String::new();
    println!("What formula are you looking for: \nArea of Trapezium = 1 \nArea of Rhombus = 2 \nArea of Parallelogram = 3 \nArea of Cube = 4 \nVolume of Cylinder = 5");
    io::stdin().read_line(&mut input1).expect("Failed to input");
    let decision:i32 = input1.trim().parse().expect("Invalid input");

    if decision == 1{
        println!("{} square",area_of_trapezium());
    }
    else if decision == 2{
        println!("{} square",area_of_rhombus());
    }
    else if decision == 3{
        println!("{} square",area_of_parallelogram());
    }
    else if decision == 4{
        println!("{} square",area_of_cube());
    }
    else if decision == 5{
        println!("{} cube",volume_of_cylinder());
    }
    else{
        println!("Pick from the numbers available");
    }
}

fn area_of_trapezium()->f32{
    let mut input2 = String::new();
    println!("Enter base 1: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base1:f32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter base 2: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let base2:f32 = input3.trim().parse().expect("Invalid input");
    
    let mut input4 = String::new();
    println!("Enter height: ");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let height:f32 = input4.trim().parse().expect("Invalid input");

    let area_of_trapezium = height/(2.0 * (base1 + base2));
    println!("Area of Trapezium: {}",area_of_trapezium);
    return area_of_trapezium;
}

fn area_of_rhombus()->f32{
    let mut input5 = String::new();
    println!("Enter diagonal 1: ");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let diagonal1:f32 = input5.trim().parse().expect("Invalid input");

    let mut input6 = String::new();
    println!("Enter diagonal 2: ");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let diagonal2:f32 = input6.trim().parse().expect("Invalid input");

    let area_of_rhombus = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus: {}",area_of_rhombus);
    return area_of_rhombus; 
}

fn area_of_parallelogram()->f32{
    let mut input7 = String::new();
    println!("Enter base: ");
    io::stdin().read_line(&mut input7).expect("Failed to read input");
    let base:f32 = input7.trim().parse().expect("Invalid input");

    let mut input8 = String::new();
    println!("Enter altitude: ");
    io::stdin().read_line(&mut input8).expect("Failed to read input");
    let altitude:f32 = input8.trim().parse().expect("Invalid input");

    let area_of_parallelogram = base * altitude;
    println!("Area of Parallelogram: {}",area_of_parallelogram);
    return area_of_parallelogram;
}

fn area_of_cube()->f32{
    let mut input9 = String::new();
    println!("Enter length: ");
    io::stdin().read_line(&mut input9).expect("Failed to read input");
    let length:f32 = input9.trim().parse().expect("Invalid input");

    let area_of_cube = 6.0 * (length * length);
    println!("Area of Cube: {}",area_of_cube);
    return area_of_cube;
}

fn volume_of_cylinder()->f32{
    let a:f32 = 22.0;
    let b:f32 = 7.0;
    let pi:f32 = a/b;

    let mut input10 = String::new();
    println!("Enter radius: ");
    io::stdin().read_line(&mut input10).expect("Failed to read input");
    let radius:f32 = input10.trim().parse().expect("Invalid input");

    let mut input11 = String::new();
    println!("Enter height: ");
    io::stdin().read_line(&mut input11).expect("Failed to read input");
    let height2:f32 = input10.trim().parse().expect("Invalid input");

    let volume_of_cylinder =  pi * (radius * radius) * height2;
    println!("Volume of Cylinder: {}",volume_of_cylinder);
    return volume_of_cylinder;
}