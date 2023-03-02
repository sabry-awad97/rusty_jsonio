use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

// Define a struct to represent your data
#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

fn main() {
    let data = vec![
        Person {
            name: String::from("Alice"),
            age: 25,
            email: String::from("alice@example.com"),
        },
        Person {
            name: String::from("Bob"),
            age: 30,
            email: String::from("bob@example.com"),
        },
    ];

    let path = Path::new("data.json");

    // Write data to JSON file
    let file = File::create(&path).expect("Could not create file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &data).expect("Could not write to file");

    // Read data from JSON file
    let file = File::open(&path).expect("Could not open file");
    let reader = BufReader::new(file);
    let loaded_data: Vec<Person> =
        serde_json::from_reader(reader).expect("Could not read from file");

    println!("{:#?}", loaded_data);
}
