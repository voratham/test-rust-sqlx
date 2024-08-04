# 🦀 How to run 🦀 

```sh
docker-compose up -d # for start mysql
cargo run
```

## Preview result
```sh
🚀 Start program...
🦀 Product { product_id: 2, product_name: "Product B", price: BigDecimal(sign=Plus, scale=2, digits=[2999]), quantity: 200, created_at: 2024-08-04 3:41:28.0 +00:00:00, updated_at: 2024-08-04 3:41:28.0 +00:00:00 }
-------------
🦀 Product { product_id: 3, product_name: "Product C", price: BigDecimal(sign=Plus, scale=2, digits=[3999]), quantity: 150, created_at: 2024-08-04 3:41:28.0 +00:00:00, updated_at: 2024-08-04 3:41:28.0 +00:00:00 }
-------------
🦀 Product { product_id: 4, product_name: "Product D", price: BigDecimal(sign=Plus, scale=2, digits=[4999]), quantity: 50, created_at: 2024-08-04 3:41:28.0 +00:00:00, updated_at: 2024-08-04 3:41:28.0 +00:00:00 }
-------------
🦀 Product { product_id: 5, product_name: "Product E", price: BigDecimal(sign=Plus, scale=2, digits=[5999]), quantity: 75, created_at: 2024-08-04 3:41:28.0 +00:00:00, updated_at: 2024-08-04 3:41:28.0 +00:00:00 }
-------------
🦀 Product { product_id: 6, product_name: "Apple Updated", price: BigDecimal(sign=Plus, scale=2, digits=[1212]), quantity: 10, created_at: 2024-08-04 3:41:42.0 +00:00:00, updated_at: 2024-08-04 3:41:42.0 +00:00:00 }
-------------
🦀 Product { product_id: 7, product_name: "Apple Updated", price: BigDecimal(sign=Plus, scale=2, digits=[1212]), quantity: 10, created_at: 2024-08-04 3:43:07.0 +00:00:00, updated_at: 2024-08-04 3:43:07.0 +00:00:00 }
-------------
🦀 Product { product_id: 8, product_name: "Apple Updated", price: BigDecimal(sign=Plus, scale=2, digits=[1212]), quantity: 10, created_at: 2024-08-04 3:44:10.0 +00:00:00, updated_at: 2024-08-04 3:44:10.0 +00:00:00 }
-------------
🦀 Product { product_id: 9, product_name: "Apple Updated", price: BigDecimal(sign=Plus, scale=2, digits=[1212]), quantity: 10, created_at: 2024-08-04 3:44:23.0 +00:00:00, updated_at: 2024-08-04 3:44:23.0 +00:00:00 }
-------------
🦀 Product { product_id: 10, product_name: "Apple Updated", price: BigDecimal(sign=Plus, scale=2, digits=[1212]), quantity: 10, created_at: 2024-08-04 3:45:08.0 +00:00:00, updated_at: 2024-08-04 3:45:08.0 +00:00:00 }
-------------
🦀 Product { product_id: 11, product_name: "Apple Updated", price: BigDecimal(sign=Plus, scale=2, digits=[1212]), quantity: 10, created_at: 2024-08-04 3:46:40.0 +00:00:00, updated_at: 2024-08-04 3:46:40.0 +00:00:00 }
-------------
🦀 Product { product_id: 12, product_name: "Bulk product 1", price: BigDecimal(sign=Plus, scale=2, digits=[10022]), quantity: 10, created_at: 2024-08-04 3:46:40.0 +00:00:00, updated_at: 2024-08-04 3:46:40.0 +00:00:00 }
-------------
🦀 Product { product_id: 13, product_name: "Bulk product 2", price: BigDecimal(sign=Plus, scale=2, digits=[8888]), quantity: 5, created_at: 2024-08-04 3:46:40.0 +00:00:00, updated_at: 2024-08-04 3:46:40.0 +00:00:00 }
-------------
🦀 Product { product_id: 14, product_name: "Bulk product 3", price: BigDecimal(sign=Plus, scale=2, digits=[8888]), quantity: 5, created_at: 2024-08-04 3:46:40.0 +00:00:00, updated_at: 2024-08-04 3:46:40.0 +00:00:00 }
-------------
🟢 total product 13
✅ Added product completed last_insert_id: 15
✅ Updated product_name of product_id: 15
🔴 Failed to product_name of product_id: 1
placeholders :: ["(?, ?, ?)", "(?, ?, ?)", "(?, ?, ?)"]
query debug "INSERT INTO product (product_name, price, quantity) VALUES(?, ?, ?), (?, ?, ?), (?, ?, ?)"
✅ Bulked completed
🚀 Ended program...
```


- i follow from example 
https://github.com/launchbadge/sqlx/blob/main/examples/mysql/todos/src/main.rs

- ref1 in case bulk insert on mysql will concat string myself : https://users.rust-lang.org/t/how-to-use-sqlx-for-batch-insert-or-dynamic-query-of-mysql/43064

- ref2: on sqlx rust repo
https://github.com/launchbadge/sqlx/blob/main/FAQ.md#how-can-i-bind-an-array-to-a-values-clause-how-can-i-do-bulk-inserts

- transaction ref: https://docs.rs/sqlx/latest/sqlx/struct.Transaction.html

- blog : https://dev.to/behainguyen/rust-mysql-delete-insert-data-using-crate-sqlx-9ii