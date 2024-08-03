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

    let rows_affected = sqlx::query(
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

    if rows_affected > 0 {
        println!("✅ Updated product_name of product_id: {}", last_insert_id);
    } else {
        println!(
            "🔴 Failed to product_name of product_id: {}",
            last_insert_id
        );
    }

    Ok(())
}
