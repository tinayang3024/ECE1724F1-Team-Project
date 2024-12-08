use crate::db;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::postgres::PgPool;

#[derive(Deserialize)]
struct UserData {
    username: String,
}

#[derive(sqlx::FromRow, Deserialize)]
pub struct AccountInfo {
    pub account_id: Option<i64>,
    pub username: String,
    pub account_name: String,
    pub account_type: db::AccountType,
    pub account_limit: i32,
}

#[derive(sqlx::FromRow, Deserialize)]
pub struct TransactionInfo {
    pub transaction_id: Option<i64>,
    pub transaction_date: Option<NaiveDate>,
    pub transaction_type: Option<db::TransactionType>,
    pub category: Option<String>,
    pub amount: Option<f64>,
    pub transaction_memo: Option<String>,
    pub account_id: i64,
}

pub async fn run_server(db_pool: PgPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .route("/", web::get().to(greet))
            // API endpoints
            .route(
                "/query_or_create_user",
                web::post().to(query_or_create_user),
            )
            .route(
                "/create_or_update_account",
                web::post().to(create_or_update_account),
            )
            .route(
                "/create_or_update_transaction",
                web::post().to(create_or_update_transaction),
            )
            .route("/delete_user", web::post().to(delete_user))
            .route("/delete_account/{account_id}", web::get().to(delete_account))
            .route(
                "/delete_transaction/{transaction_id}",
                web::get().to(delete_transaction),
            )
            .route("/query_account", web::post().to(query_account))
    })
    .bind("localhost:8080")?
    .run()
    .await
}

/*****************************************************************************/
/* API handlers */

// check whether server is up
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Server is up!")
}

////////////////////////// tbd: change all input type to json
/// db api not returning any result.
async fn query_or_create_user(
    pool: web::Data<PgPool>,
    user_data: web::Form<UserData>,
) -> impl Responder {
    let username = &user_data.username;
    match db::query_or_create_user(&pool, username).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn delete_user(pool: web::Data<PgPool>, user_data: web::Form<UserData>) -> impl Responder {
    let username = &user_data.username;
    println!("deleting user {:?}", username.clone());
    match db::delete_single_user(&pool, username).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn create_or_update_account(
    pool: web::Data<PgPool>,
    info: web::Form<AccountInfo>,
) -> impl Responder {
    println!("create_or_update_account triggered");

    // tbd: pass username instead of user_id
    let username = &info.username;
    let account_name = &info.account_name;
    let account_type = &info.account_type;
    let account_limit = info.account_limit;
    match db::create_or_update_account(
        &pool,
        info.account_id,
        username,
        account_name,
        account_type,
        account_limit,
    )
    .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn delete_account(pool: web::Data<PgPool>, account_id: web::Path<i64>) -> impl Responder {
    println!("delete_account triggered");
    match db::delete_single_account(&pool, account_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(), // return status code: 200 OK
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn create_or_update_transaction(
    pool: web::Data<PgPool>,
    info: web::Form<TransactionInfo>,
) -> impl Responder {
    println!("create_or_update_transaction triggered");

    let transaction_date = &info.transaction_date.unwrap();
    let transaction_type = &info.transaction_type.as_ref().unwrap();
    let category = &info.category.as_ref().unwrap();
    let amount = info.amount.unwrap();
    let transaction_memo = &info.transaction_memo.as_ref().unwrap();
    let account_id = info.account_id;

    println!("create/update transaction triggered: transaction_date {:?} category {:?} transaction_memo {:?}", 
        transaction_date, category, transaction_memo);

    match db::create_or_update_transaction(
        &pool,
        info.transaction_id,
        transaction_date,
        transaction_type,
        category,
        amount,
        transaction_memo,
        account_id,
    )
    .await
    {
        Ok(result) => {
            println!("create/update transaction got: {:?}", result);
            HttpResponse::Ok().json(result)
        },
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn delete_transaction(
    pool: web::Data<PgPool>,
    transaction_id: web::Path<i64>,
) -> impl Responder {
    match db::delete_single_transaction(&pool, transaction_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn query_account(
    pool: web::Data<PgPool>,
    info: web::Form<TransactionInfo>,
) -> impl Responder {
    println!("query_account triggered");

    let account_id = info.account_id;
    match db::query_account_transactions(
        &pool,
        account_id,
        &info.transaction_type,
        &info.category,
    ).await {
        Ok(result) => {
            println!("query account got: {:?}", result);
            HttpResponse::Ok().json(result)
        }
        Err(e) => {
            println!("ERROR query account: {}", e);
            HttpResponse::InternalServerError().json(format!("Error: {}", e))
        }
    }
}

/*****************************************************************************/
/* Example frontend code for calling server APIs

const formData = new URLSearchParams();
formData.append("username", "renli");

fetch("http://localhost:8080/query_or_create_user", {
    method: "POST",
    headers: { "Content-Type": "application/x-www-form-urlencoded" },
    body: formData,
})
    .then((response) => response.json())
    .then((data) => console.log("User created:", data))
    .catch((error) => console.error("Error:", error));

///////////////////
for results, server returns in JSON format { id: 1, username: "renli" }
for success status code, frontend check for response.ok

 */
