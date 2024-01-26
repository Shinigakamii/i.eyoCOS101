use std::io;

fn main() {
    println!("Number of Siblings: ");
    let mut siblings = String::new();
    io::stdin().read_line(&mut siblings).expect("Failed to read input");
    let siblings:i32 = siblings.trim().parse().expect("Invalid input");

    let mut siblings_data: [String: 10] = Default::default();

    for s in 1.. = siblings {
        println!("Enter sibling details {}: ",s);

        let mut siblings_info = String::new();

        println!("Enter Name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Not a valid string");
        siblings_info.push_str(&format! ("Name: {}, ", name));

       println!("Enter Age: ");
       let mut age = String::new();
       io::stdin().read_line(&mut age).expect("Failed to read input");
       let age:i32 = age.trim().parse().expect("Invalid input");
       siblings_info.push_str(&format! ("Age: {}, ", age));
   }

       if age > 18 {
           println!("Is sibling married? yes/no: ");
           let mut married = String::new();
           io::stdin().read_line(&mut married).expect("Failed to read line");

            if married == "yes" {
            println!("How many children: ");
            let mut children = String::new();
            io::stdin().read_line(&mut children).expect("Failed to read input");
            siblings_info.push_str(&format! ("Marriage Status: Married, Children: {}, ", children.trim());
            }    

            println!("What city does the family live in: ");
            let mut city = String::new();
            io::stdin().read_line(&mut city).expect("Failed to read line");
            siblings_info.push_str(&format! ("City: {}, ", city));
            } 
            else {
            println!("Is sibling a student or worker: ");
            };
            let mut s_w = String::new();
            io::stdin().read_line(&mut s_w).expect("Failed to read line");

            if s_w.trim() == "student" {
                println!("Enter University: ");
                let mut university = String::new();
                io::stdin().read_line(&mut university).expect("Failed to read line");
                siblings_info.push_str(&format! ("Marriage Status: Single, Status: Student, University: {}, ",university));
                
                println!("Enter Course: ");
                let mut course = String::new();
                io::stdin().read_line(&mut course).expect("Failed to read line");
                siblings_info.push_str(&format! ("Course: {}, ", course));
            } else {
                siblings_info.push_str("Marital Status: Single, Status: Worker");
            }
        else {
        println!("Do you have WAEC: ");
        let mut waec = String::new();
        io::stdin().read_line(&mut waec).expect("Failed to read input");

        if waec.trim() == "yes" {
            println!("Enter Secondary School you went to: ");
            let mut sec = String::new();
            io::stdin().read_line(&mut sec).expect("Failed to read line");
            siblings_info.push_str(&format! ("WAEC status:yes , Secondary School {}, ", sec));
        } else{
            println!("Ente Class Level: ");
            let mut class = String::new();
            io::stdin().read_line(&mut class).expect("Failed to read line");
            siblings_info.push_str(&format! ("WAEC Status: No, Class Level: {}, ", class));
        }
    }         
}