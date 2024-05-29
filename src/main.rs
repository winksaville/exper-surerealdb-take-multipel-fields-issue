use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: i64,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create database connection
    let db = Surreal::new::<Mem>(()).await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Create person Tobie Hitchcock and add it to persons table
    let _: Vec<Person> = db
        .create("persons")
        .content(Person {
            name: "Tobie Hitchcock".to_string(),
            age: 30,
        })
        .await?;

    // Select all persons from persons table
    let surql = "SELECT name, age FROM persons";
    let mut response = db.query(surql).await?;

    // Take all names from the response
    let names: Vec<String> = response.take((0, "name"))?;
    assert!(names.len() == 1, "Expected 1 name got {}", names.len()); // PASSES
    dbg!(&names);

    // Take all ages from the response
    let ages: Vec<i64> = response.take((0, "age"))?;
    dbg!(&ages);
    assert!(ages.len() == 1, "Expected 1 age got {}", ages.len()); // FAILS

    Ok(())
}
