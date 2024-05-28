use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: i64,
    is_active: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create database connection
    let db = Surreal::new::<Mem>(()).await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    println!("Create person Tobie Hitchcock and add it to persons table");
    let tobie: Vec<Record> = db
        .create("persons")
        .content(Person {
            name: "Tobie Hitchcock".to_string(),
            age: 30,
            is_active: true,
        })
        .await?;
    dbg!(&tobie);

    println!("Add a second person Tony Tiger to persons table");
    let tony: Vec<Record> = db
        .create("persons")
        .content(Person {
            name: "Tony Tiger".to_string(),
            age: 50,
            is_active: false,
        })
        .await?;
    dbg!(&tony);

    println!();
    println!(r#""Select name, age FROM persons LIMIT 1", returns only 1 record"#);
    let surql = "SELECT name, age FROM persons LIMIT 1";
    let mut response = db.query(surql).await?;

    println!(r#"then: let name: Option<String> = response.take((0, "name"))?; succeeds"#);
    let name: Option<String> = response.take((0, "name"))?;
    assert!(name.is_some(), "Expected Some(name)");
    dbg!(&name);

    println!(r#"and: let age: Option<i64> = response.take((0, "age"))?; succeeds"#);
    let age: Option<i64> = response.take((0, "age"))?;
    assert!(age.is_some(), "Expected Some(age)");
    dbg!(&age);

    println!();
    println!(r#""Select name, age FROM persons", which will return 2 records"#);
    let surql = "SELECT name, age FROM persons";
    let mut response = db.query(surql).await?;

    println!(
        r#"then: let names: Vec<String> = response.take((0, "name"))?; succeeds, there are 2 records"#
    );
    let names: Vec<String> = response.take((0, "name"))?;
    assert!(names.len() == 2, "Expected 2 names");
    dbg!(&names);

    println!(r#"but: let ages: Vec<i64> = response.take((0, "age"))?; FAILS, there are 0 records"#);
    let ages: Vec<i64> = response.take((0, "age"))?;
    dbg!(&ages);
    //assert!(ages.len() == 2, "Expected 2 ages"); // FAILS nothing is returned if take is used twice

    println!();
    println!(r#"Reversing the order and doing another "SELECT name, age FROM persons""#);
    let surql = "SELECT name, age FROM persons";
    let mut response = db.query(surql).await?;

    println!(
        r#"now: let ages: Vec<i64> = response.take((0, "age"))?; succeeds, there are 2 records"#
    );
    let ages: Vec<i64> = response.take((0, "age"))?;
    dbg!(&ages);
    assert!(ages.len() == 2, "Expected 2 ages");

    println!(
        r#"but: let names: Vec<i64> = response.take((0, "name"))?; FAILS, there are 0 records"#
    );
    let names: Vec<String> = response.take((0, "name"))?;
    dbg!(&names);
    //assert!(names.len() == 2, "Expected 2 names");

    Ok(())
}
