use sqlx::postgres::PgPool;
use strum_macros::{Display, EnumString};

pub const PG_CONNECTION_STR: &str =
    "postgres://postgres:1724_password@database-1.chmwu04uiq6g.us-east-2.rds.amazonaws.com:5432/financedb";

#[derive(Display, EnumString)]
pub enum AccountType {
    Chequing,
    Credit,
    // Savings,
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    user_id: i64,
    username: String,
}

/*****************************************************************************/
/*                                User APIs                                  */
/*****************************************************************************/

pub async fn user_create(pool: &PgPool, username: &str) -> Result<i64, sqlx::Error> {
    let rec: (i64, ) = sqlx::query_as(
        r#"
INSERT INTO users (username)
VALUES ($1)
RETURNING user_id
        "#
    )
    .bind(username)
    .fetch_one(pool)
    .await?;

    Ok(rec.0)
}

pub async fn user_delete(pool: &PgPool, username: &str) -> Result<u64, sqlx::Error> {
    let rows = sqlx::query(
        r#"
DELETE FROM users
WHERE username=($1)
        "#
    )
    .bind(username)
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows)
}

pub async fn user_get_all(pool: &PgPool) -> Result<(), sqlx::Error> {
    let users: Vec<User> = sqlx::query_as(
        r#"
SELECT *
FROM users
        "#
    )
    .fetch_all(pool)
    .await?;

    for user in users {
        println!("::[DEBUG] Got username: {} with id: {}", user.username, user.user_id);
    }

    Ok(())
}

pub async fn user_get_one(pool: &PgPool, username: &str) -> Result<i64, sqlx::Error> {
    let user: User = sqlx::query_as(
        r#"
SELECT *
FROM users
WHERE username=($1)
        "#
    )
    .bind(username)
    .fetch_one(pool)
    .await?;

    println!("::[DEBUG] Found user_id: {} from username: {}", user.user_id, user.username);
    Ok(user.user_id)
}

/*****************************************************************************/
/*                               Account APIs                                */
/*****************************************************************************/

pub async fn account_create(pool: &PgPool,
                            username: &str,
                            account_name: &str,
                            account_type: &AccountType,
                            account_limit: i32) -> Result<i64, sqlx::Error> {
    let user_id = user_get_one(pool, username).await?;
    let rec: (i64, ) = sqlx::query_as(
        r#"
INSERT INTO accounts (user_id, account_name, account_type, account_limit)
VALUES ($1, $2, $3, $4)
RETURNING account_id
        "#
    )
    .bind(user_id)
    .bind(account_name)
    .bind(account_type.to_string())
    .bind(account_limit)
    .fetch_one(pool)
    .await?;

    Ok(rec.0)
}
