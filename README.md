# 🦀 How to run 🦀 

```sh
docker-compose up -d # for start mysql
cargo run
```

- i follow from example 
https://github.com/launchbadge/sqlx/blob/main/examples/mysql/todos/src/main.rs

- ? in case bulk insert on mysql will concat string myself
-ref : https://users.rust-lang.org/t/how-to-use-sqlx-for-batch-insert-or-dynamic-query-of-mysql/43064

-ref2: on sqlx rust repo
https://github.com/launchbadge/sqlx/blob/main/FAQ.md#how-can-i-bind-an-array-to-a-values-clause-how-can-i-do-bulk-inserts