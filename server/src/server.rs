use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use sqlx::postgres::PgPool;
use crate::db;
use serde::Deserialize;

/* #[derive(Deserialize)]
struct UserData {
    username: String,
}

#[derive(Deserialize)]
struct AccountData {
    account_id: Option<i64>,
    username: String,
    account_name: String,
    account_type: db::AccountType,
    account_limit: f32,
}

#[derive(Deserialize)]
pub struct Transaction {
    transaction_id: Option<i64>,
    transaction_date: chrono::NaiveDate,
    transaction_type: db::TransactionType,
    category: String,
    amount: f32,
    transaction_memo: String,
    account_id: i64,
} */

#[derive(Deserialize)]
struct UserData {
    username: String,
}

pub async fn run_server(db_pool: PgPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            // API endpoints
            .route("/query_or_create_user", web::post().to(query_or_create_user))
            .route("/create_or_update_account", web::post().to(create_or_update_account))
            .route("/create_or_update_transaction", web::post().to(create_or_update_transaction))
            .route("/delete_user", web::delete().to(delete_user))
            .route("/delete_account", web::delete().to(delete_account))
            .route("/delete_transaction/{id}", web::delete().to(delete_transaction))
            .route("/query_account", web::get().to(query_account))
    })
    .bind("localhost:8080")?
    .run()
    .await
}

/*****************************************************************************/
/* API handlers */

async fn query_or_create_user(pool: web::Data<PgPool>, user_data: web::Form<UserData>) -> impl Responder {
    let username = &user_data.username;
    match db::query_or_create_user(&pool, username).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn delete_user(pool: web::Data<PgPool>, user_data: web::Form<UserData>) -> impl Responder {
    let username = &user_data.username;
    match db::delete_single_user(&pool, username).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn create_or_update_account(pool: web::Data<PgPool>, info: web::Json<db::Account>) -> impl Responder {
    let account_id = &info.account_id;
    // tbd: pass in either user id or username for account update
    let username = &info.user_id;
    let account_name = &info.account_name;
    let account_type = &info.account_type;
    let account_limit = &info.account_limit;
    match db::create_or_update_account(&pool, account_id, username, account_name, account_type, account_limit).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn delete_account(pool: web::Data<PgPool>, account_id: web::Path<i64>) -> impl Responder {
    
    match db::delete_single_account(&pool, account_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(), // return status code: 200 OK
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn create_or_update_transaction(pool: web::Data<PgPool>, info: web::Json<db::Transaction>) -> impl Responder {
    let transaction_date = &info.transaction_date;
    let transaction_type = &info.transaction_type;
    let category = &info.category;
    let amount = &info.amount;
    let transaction_memo = &info.transaction_memo;
    let account_id = &info.account_id;
    match db::create_or_update_transaction(&pool, transaction_date, transaction_type, category, amount, transaction_memo, account_id).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn delete_transaction(pool: web::Data<PgPool>, transaction_id: web::Path<i64>) -> impl Responder {
    match db::delete_single_transaction(&pool, transaction_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn query_account(pool: web::Data<PgPool>, account_id: web::Path<i64>) -> impl Responder {
    match db::query_account_transactions(&pool, account_id.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
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