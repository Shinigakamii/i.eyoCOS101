use std::fs::File;
use std::io::{self, Write};

struct Company {
    company_name: String,
    founding_date: i32,
    assets: f32,
    liabilities: f32,
    username: String,
    password: String,
}

impl Company {
    fn calculations(&self) -> (f32, f32, f32) {
        let over_leverage = (self.assets - self.liabilities) / self.assets;
        let percentage_leverages = over_leverage * 100.0;
        let multiple_of_percentage_leverages = percentage_leverages * over_leverage;
        (over_leverage, percentage_leverages, multiple_of_percentage_leverages)
    }

    fn five_percent_of_percentage_leverages(&self) -> f32 {
        if self.liabilities < 10_000_000.0 {
            self.calculations().1 * 5.0 / 100.0
        } else {
            println!("Liabilities is not less than 10,000,000");
            0.0
        }
    }
}

fn main() {
    let industry = vec![
        Company {
            company_name: String::from("Cadbury"),
            founding_date: 1965,
            assets: 15_000_000.0,
            liabilities: 5_500_000.0,
            username: String::from("Cadb"),
            password: String::from("pass"),
        },
        
        Company {
            company_name: String::from("Champion"),
            founding_date: 1974,
            assets: 25_000_000.0,
            liabilities: 8_000_000.0,
            username: String::from("Cham"),
            password: String::from("pass"),
        },

        Company {
            company_name: String::from("Dangote"),
            founding_date: 1970,
            assets: 18_000_000.0,
            liabilities: 10_000_000.0,
            username: String::from("Dang"),
            password: String::from("pass"),
        },

        Company {
            company_name: String::from("Flour Mills"),
            founding_date: 1960,
            assets: 32_000_000.0,
            liabilities: 4_000_000.0,
            username: String::from("Flou"),
            password: String::from("pass"),
        },

        Company {
            company_name: String::from("Nestle"),
            founding_date: 1961,
            assets: 8_000_000.0,
            liabilities: 1_500_000.0,
            username: String::from("Nest"),
            password: String::from("pass"),
        },

        Company {
            company_name: String::from("Unilever"),
            founding_date: 1923,
            assets: 37_000_000.0,
            liabilities: 11_000_000.0,
            username: String::from("Unil"),
            password: String::from("pass"),
        },

        Company {
            company_name: String::from("Honeywell"),
            founding_date: 1906,
            assets: 34_000_000.0,
            liabilities: 9_000_000.0,
            username: String::from("Hone"),
            password: String::from("pass"),
        },

        Company {
            company_name: String::from("Nigerian Breweries"),
            founding_date: 1946,
            assets: 30_000_000.0,
            liabilities: 12_000_000.0,
            username: String::from("Nige"),
            password: String::from("pass"),
        },
    ];

    println!("Enter your username: ");
    let mut input_username = String::new();
    io::stdin().read_line(&mut input_username).expect("Failed to read");

    println!("Enter your password: ");
    let mut input_password = String::new();
    io::stdin().read_line(&mut input_password).expect("Failed to read");

    if let Some(company) = industry.iter().find(|c| c.username == input_username.trim() && (4..=8).contains(&input_password.trim().len())) {
        println!("Login successful for company: {}", company.company_name);

        let mut file = File::create("Nigerian_Economy.txt").expect("create failed");

        file.write_all(format!("{} , {} , {} , {} , {} , {}\n", "Company", "Founding Date", "Assets", "Liabilities", "Percentage Leverages", "5% of percentage leverages").as_bytes()).expect("write failed");

        for company in &industry {
            let (over_leverage, percentage_leverages, multiple_of_percentage_leverages) = company.calculations();
            let five_percent = company.five_percent_of_percentage_leverages();
            file.write_all(format!("{} , {} , {} , {} , {} , {}\n", company.company_name, company.founding_date, company.assets, company.liabilities, percentage_leverages, five_percent).as_bytes()).expect("write failed");
        }

        println!("Data has been saved to Nigerian_Economy.txt");

        if industry.iter().any(|company| company.liabilities < 10_000_000.0 && company.assets > 20_000_000.0) {
            let mut file = File::create("Data1.txt").expect("create failed");

            file.write_all("For Companies with Liabilities Less than 10,000,000\n".as_bytes()).expect("write failed");

            file.write_all(format!("{} , {} , {}\n", "Company", "Percentage Leverages", "Multiple of Percentage Leverages").as_bytes()).expect("write failed");

            for company in &industry {
                if company.liabilities < 10_000_000.0 && company.assets > 20_000_000.0 {
                    let (_, percentage_leverages, multiple_of_percentage_leverages) = company.calculations();
                    file.write_all(format!("{} , {} , {}\n", company.company_name, percentage_leverages, multiple_of_percentage_leverages).as_bytes()).expect("write failed");
                }
            }

            println!("Data has been saved to Data1.txt");
        }
    } else {
        println!("Invalid credentials. Exiting...");
    }
}