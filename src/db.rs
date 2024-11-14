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

#[derive(sqlx::FromRow, Debug)]
pub struct Account {
    account_id: i64,
    user_id: i64,
    account_name: String,
    account_type: String,
    account_limit: i32,
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

pub async fn account_delete(pool: &PgPool, account_id: i64) -> Result<u64, sqlx::Error> {
    let rows = sqlx::query(
        r#"
DELETE FROM accounts
WHERE account_id=($1)
        "#
    )
    .bind(account_id)
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows)
}

pub async fn account_limit_update(pool: &PgPool,
                                  account_id: i64,
                                  account_limit: i32) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
UPDATE accounts
SET account_limit=($1)
WHERE account_id=($2)
        "#
    )
    .bind(account_limit)
    .bind(account_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn account_get_all(pool: &PgPool) -> Result<(), sqlx::Error> {
    let accounts: Vec<Account> = sqlx::query_as(
        r#"
SELECT *
FROM accounts
        "#
    )
    .fetch_all(pool)
    .await?;

    for account in accounts {
        println!(
            "::[DEBUG] Got account_id: {}, user_id: {}, account_name:{}, account_type: {}, account_limit: {}",
            account.account_id, account.user_id, account.account_name,
            account.account_type, account.account_limit
        );
    }

    Ok(())
}

pub async fn account_get_one(pool: &PgPool,
                             username: &str,
                             account_name: &str) -> Result<i64, sqlx::Error> {
    let user_id = user_get_one(pool, username).await?;
    let account: Account = sqlx::query_as(
        r#"
SELECT *
FROM accounts
WHERE user_id=($1) AND account_name=($2)
        "#
    )
    .bind(user_id)
    .bind(account_name)
    .fetch_one(pool)
    .await?;

    println!(
        "::[DEBUG] Found account_id: {} with user_id: {} and account_name: {}",
        account.account_id, account.user_id, account.account_name
    );

    Ok(account.account_id)
}
