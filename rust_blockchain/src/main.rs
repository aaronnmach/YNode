use actix_web::{web, App, HttpServer, Responder};

async fn greet() -> impl Responder {
    "Welcome to My DeFi Wallet API!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn create_wallet() -> impl Responder {
    let wallet = LocalWallet::new(&mut rand::thread_rng());
    format!("Wallet Address: {:?}", wallet.address())
}

use sqlx::PgPool;
use dotenv::dotenv;

async fn create_db_pool() -> PgPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.unwrap()
}
