use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let lagers = vec![
        "33 Export",
        "Desperado",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stouts = vec!["Legend", "Turbo King", "Williams"];

    let non_alcoholic = vec![
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
    ];

    let drink_categories = vec![("Lager", lagers), ("Stout", stouts), ("Non_Alcoholic", non_alcoholic)];

    let file_name = "drink.txt";

    let mut file = File::create(file_name)?;

    for (category_name, drinks) in drink_categories {
        writeln!(file, "{}:", category_name)?;

        for drink_variety in drinks {
            writeln!(file, "= {}", drink_variety)?;
        }
    }

    println!("Drink categories saved to drink.txt");

    Ok(())
}