// Define a struct to represent each record
struct Record {
    sn: usize,
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

fn main() {
    // Define the datasets
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Combine the datasets into a vector of structs
    let mut records = Vec::new();

    for i in 0..names.len() {
        records.push(Record {
            sn: i + 1,
            name: names[i].to_string(),
            ministry: ministries[i].to_string(),
            geopolitical_zone: geopolitical_zones[i].to_string(),
        });
    }

    // Print the table header
    println!("{:<5} {:<30} {:<20} {:<15}", "S/N", "Name of Commissioner", "Ministry", "Geopolitical Zone");
    println!("{}", "-".repeat(75));

    // Print the records
    for record in records {
        println!(
            "{:<5} {:<30} {:<20} {:<15}",
            record.sn, record.name, record.ministry, record.geopolitical_zone
        );
    }
}


