use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use sqlx::postgres::PgPool;
// use crate::db

#[derive(Deserialize)]
enum AccountType {
    Chequing,
    Credit,
    // Savings,
}

#[derive(Deserialize)]
struct UserData {
    username: String,
}

#[derive(Deserialize)]
struct AccountData {
    username: String,
    account: AccountType,
    amount: f64,
}

pub async fn run_server(db_pool: PgPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool))
            // API endpoints
            .route("/create_user", web::post().to(create_user))
            .route("/delete_user", web::delete().to(delete_user))
            .route("/create_account", web::post().to(create_account))
            .route("/delete_account", web::delete().to(delete_account))
            .route("/update_account", web::put().to(update_account))
            .route("/add_record", web::post().to(add_record))
            .route("/delete_record/{id}", web::delete().to(delete_record))
            .route("/records", web::get().to(get_records))
    })
    .bind("localhost:8080")?
    .run()
    .await
}

/*****************************************************************************/
/* API handlers */

// TBC for input types

async fn create_user(pool: web::Data<PgPool>, userData: web::Form<UserData>) -> impl Responder {
    let username = &userData.username; 
    match db::create_user(&pool, username).await {
        Ok(_) => HttpResponse::Ok().json("User created"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn delete_user(pool: web::Data<PgPool>, userData: web::Form<UserData>) -> impl Responder {
    let username = &userData.username; 
    match db::delete_user(&pool, id.into_inner(), username).await {
        Ok(_) => HttpResponse::Ok().json("User deleted"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn create_account(pool: web::Data<PgPool>, record: web::Json<RecordInput>) -> impl Responder {
    match db::create_account(&pool, record).await {
        Ok(_) => HttpResponse::Ok().json("Account added"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn delete_account(pool: web::Data<PgPool>, accountId: web::Path<i64>, userData: web::Form<UserData>) -> impl Responder {
    let username = &userData.username; 
    match db::delete_account(&pool, accountId.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Account deleted"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn add_record(pool: web::Data<PgPool>, record: web::Json<RecordInput>) -> impl Responder {
    match db::add_record(pool: web::Data<PgPool>, record).await {
        Ok(_) => HttpResponse::Ok().json("Record added"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn delete_record(pool: web::Data<PgPool>, record_id: web::Path<i64>, userData: web::Form<UserData>) -> impl Responder {
    let username = &userData.username; 
    match db::delete_record(&pool, record_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Record deleted"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn update_record(db_pool: web::Data<PgPool>, id: web::Path<i32>, userData: web::Form<UserData>) -> impl Responder {

    match db::update_record(&db_pool, id.into_inner(), record.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update record"),
    }
}

async fn get_records(pool: web::Data<PgPool>) -> impl Responder {
    match db::get_records(&pool).await {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}
