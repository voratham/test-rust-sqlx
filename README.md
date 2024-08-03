# 🦀 How to run 🦀 

```sh
docker-compose up -d # for start mysql
cargo run
```

## Preview result
```sh
🚀 Start program...
🦀 Product { product_id: 1, product_name: "Product A", price: BigDecimal(sign=Plus, scale=2, digits=[1999]), quantity: 100, created_at: 2024-08-03 14:15:03.0 +00:00:00, updated_at: 2024-08-03 14:15:03.0 +00:00:00 }
-------------
🦀 Product { product_id: 2, product_name: "Product B", price: BigDecimal(sign=Plus, scale=2, digits=[2999]), quantity: 200, created_at: 2024-08-03 14:15:03.0 +00:00:00, updated_at: 2024-08-03 14:15:03.0 +00:00:00 }
-------------
🦀 Product { product_id: 3, product_name: "Product C", price: BigDecimal(sign=Plus, scale=2, digits=[3999]), quantity: 150, created_at: 2024-08-03 14:15:03.0 +00:00:00, updated_at: 2024-08-03 14:15:03.0 +00:00:00 }
-------------
🦀 Product { product_id: 4, product_name: "Product D", price: BigDecimal(sign=Plus, scale=2, digits=[4999]), quantity: 50, created_at: 2024-08-03 14:15:03.0 +00:00:00, updated_at: 2024-08-03 14:15:03.0 +00:00:00 }
-------------
🦀 Product { product_id: 5, product_name: "Product E", price: BigDecimal(sign=Plus, scale=2, digits=[5999]), quantity: 75, created_at: 2024-08-03 14:15:03.0 +00:00:00, updated_at: 2024-08-03 14:15:03.0 +00:00:00 }
-------------
🦀 Product { product_id: 6, product_name: "Apple Updated", price: BigDecimal(sign=Plus, scale=2, digits=[1212]), quantity: 10, created_at: 2024-08-03 14:15:11.0 +00:00:00, updated_at: 2024-08-03 14:15:11.0 +00:00:00 }
-------------
🟢 total product 6
✅ Added product completed last_insert_id: 7
✅ Updated product_name of product_id: 7
```

- i follow from example 
https://github.com/launchbadge/sqlx/blob/main/examples/mysql/todos/src/main.rs

- ? in case bulk insert on mysql will concat string myself
-ref : https://users.rust-lang.org/t/how-to-use-sqlx-for-batch-insert-or-dynamic-query-of-mysql/43064

-ref2: on sqlx rust repo
https://github.com/launchbadge/sqlx/blob/main/FAQ.md#how-can-i-bind-an-array-to-a-values-clause-how-can-i-do-bulk-inserts