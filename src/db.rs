use sqlx::postgres::PgPool;

pub const PG_CONNECTION_STR: &str =
    "postgres://postgres:1724_password@database-1.chmwu04uiq6g.us-east-2.rds.amazonaws.com:5432/financedb";

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
        println!("Got username: {} with id: {}", user.username, user.user_id);
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
