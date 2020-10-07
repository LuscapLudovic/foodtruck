use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

mod handlers;
mod config;
mod db;
mod models;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Starting server at http://{}", config.server_addr);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/position", web::get().to(handlers::get_pos))
    })
    .bind(config.server_addr.clone())?
    .run()    
    .await
}