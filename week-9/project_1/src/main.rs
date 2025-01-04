use std::fs::File;
use std::io::Write;
use serde::Serialize;

// Define a structure for the categories and their products
#[derive(Serialize)]
struct DrinkCategory {
    category: String,
    products: Vec<String>,
}

fn main() {
    // Define the data
    let categories = vec![
        DrinkCategory {
            category: String::from("Lager"),
            products: vec![
                String::from("33 Export"),
                String::from("Desperados"),
                String::from("Goldberg"),
                String::from("Gulder"),
                String::from("Heineken"),
                String::from("Star"),
            ],
        },
        DrinkCategory {
            category: String::from("Stout"),
            products: vec![
                String::from("Legend"),
                String::from("Turbo King"),
                String::from("Williams"),
            ],
        },
        DrinkCategory {
            category: String::from("Non-Alcoholic"),
            products: vec![
                String::from("Maltina"),
                String::from("Amstel Malta"),
                String::from("Malta Gold"),
                String::from("Fayrouz"),
            ],
        },
    ];

    // Serialize the data into JSON
    let json_data = serde_json::to_string_pretty(&categories).expect("Failed to serialize data");

    // Write the JSON data to a file
    let file_path = "drink_categories.json";
    let mut file = File::create(file_path).expect("Failed to create file");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to file");

    println!("Data has been saved to {}", file_path);
}
use std::fs::File;
use std::io::Write;
use serde::Serialize;

// Define a structure for the categories and their products
#[derive(Serialize)]
struct DrinkCategory {
    category: String,
    products: Vec<String>,
}

fn main() {
    // Define the data
    let categories = vec![
        DrinkCategory {
            category: String::from("Lager"),
            products: vec![
                String::from("33 Export"),
                String::from("Desperados"),
                String::from("Goldberg"),
                String::from("Gulder"),
                String::from("Heineken"),
                String::from("Star"),
            ],
        },
        DrinkCategory {
            category: String::from("Stout"),
            products: vec![
                String::from("Legend"),
                String::from("Turbo King"),
                String::from("Williams"),
            ],
        },
        DrinkCategory {
            category: String::from("Non-Alcoholic"),
            products: vec![
                String::from("Maltina"),
                String::from("Amstel Malta"),
                String::from("Malta Gold"),
                String::from("Fayrouz"),
            ],
        },
    ];

    // Serialize the data into JSON
    let json_data = serde_json::to_string_pretty(&categories).expect("Failed to serialize data");

    // Write the JSON data to a file
    let file_path = "drink_categories.json";
    let mut file = File::create(file_path).expect("Failed to create file");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to file");

    println!("Data has been saved to {}", file_path);
}

