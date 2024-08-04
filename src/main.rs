#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)] // using for disabled warning
struct Product {
    product_id: i64,
    product_name: String,
    price: sqlx::types::BigDecimal,
    quantity: i64,
    created_at: sqlx::types::time::OffsetDateTime,
    updated_at: sqlx::types::time::OffsetDateTime,
}

struct ProductCreate {
    product_name: String,
    price: f64,
    quantity: i64,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("🚀 Start program...");

    let conn_str: String = String::from("mysql://admin:1234@127.0.0.1:3306/product");

    let pool = sqlx::mysql::MySqlPool::connect(&conn_str).await?;

    let result = sqlx::query_as::<_, Product>("SELECT * from product ORDER BY product_id")
        .fetch_all(&pool)
        .await?;

    for r in result {
        println!("🦀 {:?}", r);
        println!("-------------")
    }

    let total_product: i64 = sqlx::query_scalar("SELECT COUNT(*) from product")
        .fetch_one(&pool)
        .await?;

    println!("🟢 total product {:?}", total_product);

    let last_insert_id =
        sqlx::query("INSERT INTO product (product_name, price, quantity) VALUES (?, ?, ?)")
            .bind("Apple".to_string())
            .bind(12.12)
            .bind(10)
            .execute(&pool)
            .await?
            .last_insert_id();

    println!(
        "✅ Added product completed last_insert_id: {}",
        last_insert_id
    );

    let updated_rows_affected = sqlx::query(
        r#"
         UPDATE product SET product_name = ?
         WHERE product_id = ?
        "#,
    )
    .bind("Apple Updated".to_string())
    .bind(last_insert_id)
    .execute(&pool)
    .await?
    .rows_affected();

    if updated_rows_affected > 0 {
        println!("✅ Updated product_name of product_id: {}", last_insert_id);
    } else {
        println!(
            "🔴 Failed to product_name of product_id: {}",
            last_insert_id
        );
    }

    let target_product_id = 1;
    let deleted_rows_affected = sqlx::query(r#" DELETE FROM product WHERE product_id = ? "#)
        .bind(target_product_id)
        .execute(&pool)
        .await?
        .rows_affected();

    if deleted_rows_affected > 0 {
        println!("✅ Deleted product_id: {}", target_product_id);
    } else {
        println!(
            "🔴 Failed to product_name of product_id: {}",
            target_product_id
        );
    }

    let mut products = Vec::new();

    products.push(ProductCreate {
        product_name: "Bulk product 1".to_string(),
        price: 100.22,
        quantity: 10,
    });

    products.push(ProductCreate {
        product_name: "Bulk product 2".to_string(),
        price: 88.88,
        quantity: 5,
    });

    products.push(ProductCreate {
        product_name: "Bulk product 3".to_string(),
        price: 88.88,
        quantity: 5,
    });

    let mut bulk_query_insert =
        String::from("INSERT INTO product (product_name, price, quantity) VALUES");

    let placeholders: Vec<String> = products.iter().map(|_| "(?, ?, ?)".to_string()).collect();
    // placeholders :: ["(?, ?, ?)", "(?, ?, ?)", "(?, ?, ?)"]

    println!("placeholders :: {:?}", placeholders);

    bulk_query_insert.push_str(&placeholders.join(", "));

    println!("query debug {:?}", bulk_query_insert);
    // query debug "INSERT INTO product (product_name, price, quantity) VALUES(?, ?, ?), (?, ?, ?), (?, ?, ?)"

    let mut stmt = sqlx::query(&bulk_query_insert);

    // Bind all values
    for p in products {
        stmt = stmt
            .bind(p.product_name.to_string())
            .bind(p.price)
            .bind(p.quantity);
    }

    stmt.execute(&pool).await?;
    println!("✅ Bulked completed");

    println!("🚀 Ended program...");
    Ok(())
}
