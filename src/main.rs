// use std::str::FromStr;

pub mod db;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    let pool = sqlx::PgPool::connect(db::PG_CONNECTION_STR).await?;

    // Make a simple query to return the given parameter
    db::user_get_all(&pool).await?;
    db::user_get_one(&pool, "user1").await?;
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

    Ok(())
}
