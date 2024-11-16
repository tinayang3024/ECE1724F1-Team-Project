// use std::str::FromStr;
// use sqlx::types::chrono;

pub mod db;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    let pool = sqlx::PgPool::connect(db::PG_CONNECTION_STR).await?;

    // Make a simple query to return the given parameter
    db::user_get_all(&pool).await?;
    // db::user_get_one(&pool, "user1").await?;
    // db::account_get_all(&pool).await?;
    // db::account_get_one(&pool, "user1", "account1").await?;
    // db::transaction_get_all(&pool).await?;
    // db::transaction_get_one(&pool, 2).await?;
    // db::user_get_one(&pool, "user3").await?; // Will error out
    
    // Example usages
    // ---User---
    // let my_user = String::from("user_uni");
    // let id = db::user_create(&pool, &my_user).await?;
    // println!("Created user \"{}\" with id {}", my_user, id);
    // let row = db::user_delete(&pool, &my_user).await?;
    // println!("Deleted user \"{}\", {} row(s) deleted", my_user, row);
    // ---Account---
    // let at: db::AccountType = db::AccountType::from_str("Chequing").unwrap();
    // let id = db::account_create(&pool, "user1", "account2", &at, 5000).await?;
    // println!("Created account with id {}", id);
    // let row = db::account_delete(&pool, 3).await?;
    // println!("Deleted account 3, {} row(s) deleted", row);
    // db::account_limit_update(&pool, 1, 5000).await?;
    // ---Transaction---
    // let date = chrono::NaiveDate::from_ymd_opt(2024, 11, 10).unwrap();
    // let tt: db::TransactionType = db::TransactionType::from_str("Expenses").unwrap();
    // let id = db::transaction_create(&pool, &date, &tt, "Meal", 12.34, "Sushi Burrito", 1).await?;
    // println!("Created transaction with id {}", id);
    // let row = db::transaction_delete(&pool, 1).await?;
    // println!("Deleted transaction 1, {} row(s) deleted", row);

    Ok(())
}
