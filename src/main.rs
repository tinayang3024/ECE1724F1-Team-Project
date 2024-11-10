pub mod db;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    let pool = sqlx::PgPool::connect(db::PG_CONNECTION_STR).await?;

    // Make a simple query to return the given parameter
    db::user_get_all(&pool).await?;
    db::user_get_one(&pool, "user1").await?;
    db::user_get_one(&pool, "user3").await?;

    // Example usages
    // let my_user = String::from("user_uni");
    // let id = db::user_create(&pool, &my_user).await?;
    // println!("Created user \"{}\" with id {}", my_user, id);
    // let row = db::user_delete(&pool, &my_user).await?;
    // println!("Deleted user \"{}\", {} row(s) deleted", my_user, row);

    Ok(())
}
