use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_json_file(file_path: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut json_vec = Vec::new();

    for line in reader.lines() {
        let json_string = line?;
        let json_value = serde_json::from_str(&json_string)?;
        json_vec.push(json_value);
    }

    Ok(json_vec)
}

fn main() -> Result<(), Box<dyn Error>> {
    let json_values = read_json_file("example.json")?;
    for value in json_values {
        println!("{}", value);
    }
    Ok(())
}
