use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, QueryBuilder};
use strum_macros::{Display, EnumString};

pub const PG_CONNECTION_STR: &str =
    "postgres://postgres:1724_password@database-1.chmwu04uiq6g.us-east-2.rds.amazonaws.com:5432/financedb";

#[derive(Debug, Display, EnumString, Deserialize)]
pub enum AccountType {
    Chequing,
    Credit,
    // Savings,
}

#[derive(Display, PartialEq, EnumString, Deserialize)]
pub enum TransactionType {
    Expenses,
    Income,
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub user_id: i64,
    pub username: String,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Account {
    pub account_id: i64,
    pub user_id: i64,
    pub account_name: String,
    pub account_type: String,
    pub account_limit: f64,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_id: i64,
    pub transaction_date: NaiveDate,
    pub transaction_type: String,
    pub category: String,
    pub amount: f64,
    pub transaction_memo: String,
    pub account_id: i64,
}

/*****************************************************************************/
/*                               Public APIs                                 */
/*****************************************************************************/

pub async fn query_or_create_user(
    pool: &PgPool,
    username: &str,
) -> Result<Vec<Account>, sqlx::Error> {
    if let Ok(user_id) = user_get_one(pool, username).await {
        account_get_all_for_user(pool, user_id).await
    } else {
        let user_id = user_create(pool, username).await?;
        account_get_all_for_user(pool, user_id).await
    }
}

pub async fn create_or_update_account(
    pool: &PgPool,
    account_id: Option<i64>,
    username: &str,
    account_name: &str,
    account_type: &AccountType,
    account_limit: i32,
) -> Result<i64, sqlx::Error> {
    if let Some(aid) = account_id {
        account_update(pool, aid, account_name, account_limit).await
    } else {
        account_create(pool, username, account_name, account_type, account_limit).await
    }
}

pub async fn create_or_update_transaction(
    pool: &PgPool,
    transaction_id: Option<i64>,
    transaction_date: &NaiveDate,
    transaction_type: &TransactionType,
    category: &str,
    amount: f64,
    transaction_memo: &str,
    account_id: i64,
) -> Result<i64, sqlx::Error> {
    if let Some(tid) = transaction_id {
        transaction_update(
            pool,
            tid,
            transaction_date,
            transaction_type,
            category,
            amount,
            transaction_memo,
            account_id,
        )
        .await
    } else {
        transaction_create(
            pool,
            transaction_date,
            transaction_type,
            category,
            amount,
            transaction_memo,
            account_id,
        )
        .await
    }
}

pub async fn query_account_transactions(
    pool: &PgPool,
    account_id: i64,
    transaction_type: &Option<TransactionType>,
    category: &Option<String>,
) -> Result<(Vec<Transaction>, f64), sqlx::Error> {
    let transactions =
        transaction_get_all_for_account(pool, account_id, transaction_type, category).await?;
    let transaction_sum = 
        transaction_get_sum_for_account(pool, account_id, transaction_type, category).await?;
    Ok((transactions, transaction_sum))
}

pub async fn delete_single_user(pool: &PgPool, username: &str) -> Result<(), sqlx::Error> {
    let num_deleted = user_delete(pool, username).await?;
    if num_deleted == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    if num_deleted != 1 {
        panic!("More than one user deleted, username is not unique! database is in a bad state, please contact admin :(");
    }
    Ok(())
}

pub async fn delete_single_account(pool: &PgPool, account_id: i64) -> Result<(), sqlx::Error> {
    let num_deleted = account_delete(pool, account_id).await?;
    if num_deleted == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    if num_deleted != 1 {
        panic!("More than one account deleted, account_id is not unique! database is in a bad state, please contact admin :(");
    }
    Ok(())
}

pub async fn delete_single_transaction(
    pool: &PgPool,
    transaction_id: i64,
) -> Result<(), sqlx::Error> {
    let num_deleted = transaction_delete(pool, transaction_id).await?;
    if num_deleted == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    if num_deleted != 1 {
        panic!("More than one transaction deleted, transaction_id is not unique! database is in a bad state, please contact admin :(");
    }
    Ok(())
}

/*****************************************************************************/
/*                                User APIs                                  */
/*****************************************************************************/

async fn user_create(pool: &PgPool, username: &str) -> Result<i64, sqlx::Error> {
    let rec: (i64,) = sqlx::query_as(
        r#"
INSERT INTO users (username)
VALUES ($1)
RETURNING user_id
        "#,
    )
    .bind(username)
    .fetch_one(pool)
    .await?;

    Ok(rec.0)
}

async fn user_delete(pool: &PgPool, username: &str) -> Result<u64, sqlx::Error> {
    let rows = sqlx::query(
        r#"
DELETE FROM users
WHERE username=($1)
        "#,
    )
    .bind(username)
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows)
}

async fn user_get_one(pool: &PgPool, username: &str) -> Result<i64, sqlx::Error> {
    let user: User = sqlx::query_as(
        r#"
SELECT *
FROM users
WHERE username=($1)
        "#,
    )
    .bind(username)
    .fetch_one(pool)
    .await?;

    let _username = user.username.clone();

    Ok(user.user_id)
}

/*****************************************************************************/
/*                               Account APIs                                */
/*****************************************************************************/

async fn account_create(
    pool: &PgPool,
    username: &str,
    account_name: &str,
    account_type: &AccountType,
    account_limit: i32,
) -> Result<i64, sqlx::Error> {
    let user_id = user_get_one(pool, username).await?;
    let rec: (i64,) = sqlx::query_as(
        r#"
INSERT INTO accounts (user_id, account_name, account_type, account_limit)
VALUES ($1, $2, $3, $4)
RETURNING account_id
        "#,
    )
    .bind(user_id)
    .bind(account_name)
    .bind(account_type.to_string())
    .bind(account_limit)
    .fetch_one(pool)
    .await?;

    Ok(rec.0)
}

async fn account_delete(pool: &PgPool, account_id: i64) -> Result<u64, sqlx::Error> {
    let rows = sqlx::query(
        r#"
DELETE FROM accounts
WHERE account_id=($1)
        "#,
    )
    .bind(account_id)
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows)
}

// We only update account_name and account_limit
async fn account_update(
    pool: &PgPool,
    account_id: i64,
    account_name: &str,
    account_limit: i32,
) -> Result<i64, sqlx::Error> {
    sqlx::query(
        r#"
UPDATE accounts
SET account_name=($1), account_limit=($2)
WHERE account_id=($3)
        "#,
    )
    .bind(account_name)
    .bind(account_limit)
    .bind(account_id)
    .execute(pool)
    .await?;

    Ok(account_id)
}

async fn account_get_all_for_user(
    pool: &PgPool,
    user_id: i64,
) -> Result<Vec<Account>, sqlx::Error> {
    let accounts: Vec<Account> = sqlx::query_as(
        r#"
SELECT *
FROM accounts
WHERE user_id=($1)
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(accounts)
}

/*****************************************************************************/
/*                             Transaction APIs                              */
/*****************************************************************************/

async fn transaction_create(
    pool: &PgPool,
    transaction_date: &NaiveDate,
    transaction_type: &TransactionType,
    category: &str,
    amount: f64,
    transaction_memo: &str,
    account_id: i64,
) -> Result<i64, sqlx::Error> {
    let mut adjusted_amount = amount;
    if transaction_type == &TransactionType::Expenses && amount > 0.0 {
        // Assume user means negative
        adjusted_amount = 0.0 - amount;
    }
    let rec: (i64,) = sqlx::query_as(
        r#"
INSERT INTO transactions
(transaction_date, transaction_type, category, amount, transaction_memo, account_id)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING transaction_id
        "#,
    )
    .bind(transaction_date)
    .bind(transaction_type.to_string())
    .bind(category)
    .bind(adjusted_amount)
    .bind(transaction_memo)
    .bind(account_id)
    .fetch_one(pool)
    .await?;

    Ok(rec.0)
}

async fn transaction_delete(pool: &PgPool, transaction_id: i64) -> Result<u64, sqlx::Error> {
    let rows = sqlx::query(
        r#"
DELETE FROM transactions
WHERE transaction_id=($1)
        "#,
    )
    .bind(transaction_id)
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows)
}

async fn transaction_update(
    pool: &PgPool,
    transaction_id: i64,
    transaction_date: &NaiveDate,
    transaction_type: &TransactionType,
    category: &str,
    amount: f64,
    transaction_memo: &str,
    account_id: i64,
) -> Result<i64, sqlx::Error> {
    let mut adjusted_amount = amount;
    if transaction_type == &TransactionType::Expenses && amount > 0.0 {
        // Assume user means negative
        adjusted_amount = 0.0 - amount;
    }
    sqlx::query(
        r#"
UPDATE transactions
SET transaction_date=($1), transaction_type=($2), category=($3), amount=($4),
    transaction_memo=($5), account_id=($6)
WHERE transaction_id=($7)
        "#,
    )
    .bind(transaction_date)
    .bind(transaction_type.to_string())
    .bind(category)
    .bind(adjusted_amount)
    .bind(transaction_memo)
    .bind(account_id)
    .bind(transaction_id)
    .execute(pool)
    .await?;

    Ok(transaction_id)
}

async fn transaction_get_all_for_account(
    pool: &PgPool,
    account_id: i64,
    transaction_type: &Option<TransactionType>,
    category: &Option<String>,
) -> Result<Vec<Transaction>, sqlx::Error> {
    let mut query: QueryBuilder<'_, sqlx::Postgres> =
        QueryBuilder::new("SELECT * FROM transactions WHERE account_id=");
    query.push_bind(account_id);

    if let Some(trans_type) = transaction_type {
        query.push(" AND transaction_type=");
        query.push_bind(trans_type.to_string());
    }

    if let Some(trans_category) = category {
        query.push(" AND category=");
        query.push_bind(trans_category);
    }

    let transactions: Vec<Transaction> = query.build_query_as()
        .fetch_all(pool)
        .await?;

    Ok(transactions)
}

async fn transaction_get_sum_for_account(
    pool: &PgPool,
    account_id: i64,
    transaction_type: &Option<TransactionType>,
    category: &Option<String>,
) -> Result<f64, sqlx::Error> {
    let mut query: QueryBuilder<'_, sqlx::Postgres> =
        QueryBuilder::new(
            "SELECT SUM(amount) FROM transactions WHERE account_id="
        );
    query.push_bind(account_id);

    if let Some(trans_type) = transaction_type {
        query.push(" AND transaction_type=");
        query.push_bind(trans_type.to_string());
    }

    if let Some(trans_category) = category {
        query.push(" AND category=");
        query.push_bind(trans_category);
    }

    let sum: (Option<f64>,) = query.build_query_as()
        .fetch_one(pool)
        .await?;
    
    if let Some(s) = sum.0 {
        Ok(s)
    } else {
        Ok(0.0)
    }
}
