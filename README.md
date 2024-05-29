# exper-surrealdb-take-multiple-fields-issue

A simple test where the rust app that shows attempting to use take
on multiple fields via Vec<FieldType> fails.

```
wink@3900x 24-05-29T05:13:34.101Z:~/prgs/SurrealDB/exper-surrealdb-take-multiple-fields-issue (simplest)
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/exper-surrealdb-take-multiple-fields-issue`
[src/main.rs:35:5] &names = [
    "Tobie Hitchcock",
]
[src/main.rs:39:5] &ages = []
thread 'main' panicked at src/main.rs:40:5:
Expected 1 age got 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
wink@3900x 24-05-29T05:15:54.043Z:~/prgs/SurrealDB/exper-surrealdb-take-multiple-fields-issue (simplest)
$ cargo fmt^C
wink@3900x 24-05-29T05:15:56.766Z:~/prgs/SurrealDB/exper-surrealdb-take-multiple-fields-issue (simplest)
$ cargo clippy && cargo fmt
    Checking exper-surrealdb-take-multiple-fields-issue v0.1.0 (/home/wink/prgs/SurrealDB/exper-surrealdb-take-multiple-fields-issue)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
wink@3900x 24-05-29T05:15:59.691Z:~/prgs/SurrealDB/exper-surrealdb-take-multiple-fields-issue (simplest)
$ cargo run
   Compiling exper-surrealdb-take-multiple-fields-issue v0.1.0 (/home/wink/prgs/SurrealDB/exper-surrealdb-take-multiple-fields-issue)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.60s
     Running `target/debug/exper-surrealdb-take-multiple-fields-issue`
[src/main.rs:35:5] &names = [
    "Tobie Hitchcock",
]
[src/main.rs:39:5] &ages = []
thread 'main' panicked at src/main.rs:40:5:
Expected 1 age got 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
wink@3900x 24-05-29T05:16:34.344Z:~/prgs/SurrealDB/exper-surrealdb-take-multiple-fields-issue (simplest)
$ 
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
