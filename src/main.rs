use postgres::{Client, Error, NoTls};
use std::collections::HashMap;

struct Author {
    _id: i32,
    name: String,
    country: String,
}

fn main() -> Result<(), Error> {
    // Connect to the PostgreSQL database
    let mut client = Client::connect(
        "postgres://postgres:postgres@localhost:5432/library",
        NoTls,
    )?;

    // Create the table if it doesn't already exist
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS author (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            country VARCHAR NOT NULL
        );
        ",
    )?;

    // Store authors in a HashMap
    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    // Insert authors into the database
    for (name, country) in &authors {
        let author = Author {
            _id: 0,
            name: name.to_string(),
            country: country.to_string(),
        };

        client.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
        )?;
    }

    // Retrieve and display all authors
    println!("Authors in the database:");
    println!("-------------------------");

    for row in client.query(
        "SELECT id, name, country FROM author ORDER BY id",
        &[],
    )? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };

        println!(
            "ID: {}, Name: {}, Country: {}",
            author._id, author.name, author.country
        );
    }

    Ok(())
}
