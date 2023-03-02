use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    address: String,
}

fn main() -> Result<()> {
    let json_string = r#"
        {
            "name": "John Doe",
            "age": 30,
            "address": "123 Main St."
        }
    "#;

    let person: Person = serde_json::from_str(json_string)?;

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Address: {}", person.address);

    Ok(())
}
