# exper-surrealdb-take-multiple-fields-issue

An rust app that shows attempting to use take on multiple fields via Vec<FieldType>
fails.

```
wink@3900x 24-05-28T21:25:10.956Z:~/prgs/SurrealDB/exper-surrealdb-take-multiple-fields-issue (main)
$ cargo run
   Compiling exper-surrealdb-take-multiple-fields-issue v0.1.0 (/home/wink/prgs/SurrealDB/exper-surrealdb-take-multiple-fields-issue)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.54s
     Running `target/debug/exper-surrealdb-take-multiple-fields-issue`
Create person Tobie Hitchcock and add it to persons table
[src/main.rs:36:5] &tobie = [
    Record {
        id: Thing {
            tb: "persons",
            id: String(
                "ri3mw8c32z1ljicnr260",
            ),
        },
    },
]
Add a second person Tony Tiger to persons table
[src/main.rs:47:5] &tony = [
    Record {
        id: Thing {
            tb: "persons",
            id: String(
                "wr42wzcz7jfqfgls2b3x",
            ),
        },
    },
]

"Select name, age FROM persons LIMIT 1", returns only 1 record
then: let name: Option<String> = response.take((0, "name"))?; succeeds
[src/main.rs:57:5] &name = Some(
    "Tobie Hitchcock",
)
and: let age: Option<i64> = response.take((0, "age"))?; succeeds
[src/main.rs:62:5] &age = Some(
    30,
)

"Select name, age FROM persons", which will return 2 records
then: let names: Vec<String> = response.take((0, "name"))?; succeeds, there are 2 records
[src/main.rs:74:5] &names = [
    "Tobie Hitchcock",
    "Tony Tiger",
]
but: let ages: Vec<i64> = response.take((0, "age"))?; FAILS, there are 0 records
[src/main.rs:78:5] &ages = []

Reversing the order and doing another "SELECT name, age FROM persons"
now: let ages: Vec<i64> = response.take((0, "age"))?; succeeds, there are 2 records
[src/main.rs:90:5] &ages = [
    30,
    50,
]
but: let names: Vec<i64> = response.take((0, "name"))?; FAILS, there are 0 records
[src/main.rs:97:5] &names = []
wink@3900x 24-05-28T21:25:20.780Z:~/prgs/SurrealDB/exper-surrealdb-take-multiple-fields-issue (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
