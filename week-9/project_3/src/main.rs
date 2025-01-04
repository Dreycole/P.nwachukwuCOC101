use std::fs::File;
use std::io::{self, Write};

// Define a struct to represent each minister's details
struct Minister {
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

fn main() -> io::Result<()> {
    // Separate datasets as vectors
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
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

    // Merge the datasets into a single vector of structs
    let mut ministers: Vec<Minister> = Vec::new();
    for i in 0..names.len() {
        ministers.push(Minister {
            name: names[i].to_string(),
            ministry: ministries[i].to_string(),
            geopolitical_zone: geopolitical_zones[i].to_string(),
        });
    }

    // Display the merged data in a tabular format
    println!("{:<5} {:<30} {:<20} {:<15}", "S/N", "Name", "Ministry", "Geo. Zone");
    println!("{}", "-".repeat(75));
    for (index, minister) in ministers.iter().enumerate() {
        println!(
            "{:<5} {:<30} {:<20} {:<15}",
            index + 1,
            minister.name,
            minister.ministry,
            minister.geopolitical_zone
        );
    }

    // Save the merged data into a CSV file
    let mut file = File::create("ministers.csv")?;
    writeln!(file, "S/N,Name,Ministry,Geopolitical Zone")?;
    for (index, minister) in ministers.iter().enumerate() {
        writeln!(
            file,
            "{},{},{},{}",
            index + 1,
            minister.name,
            minister.ministry,
            minister.geopolitical_zone
        )?;
    }

    println!("Data has been saved to 'ministers.csv'.");

    Ok(())
}
