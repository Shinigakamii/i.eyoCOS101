use std::io::Write;

fn main(){

    let name_of_commisioner = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs","Justice","Defence","Power & Steel","Petroleum"];
    let geopolitical_zone = vec!["South West","North East","South South","South West","South East"];

    let mut file = std::fs::File::create("EFCC.txt").expect("create failed");

    file.write_all(format!("{} , {} , {}\n","Name","Ministry","Geopolitical Zone").as_bytes()).expect("write failed");

    for n in 0..name_of_commisioner.len() {
        file.write_all(format!("{} , {} , {}\n", name_of_commisioner[n],ministry[n],geopolitical_zone[n]).as_bytes()).expect("write failed");
    }

    println!("EFCC details written in EFCC.txt")
}