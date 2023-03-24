use crate::config::AppConfig;
use ::config::{Config, Environment};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

mod config;
mod errors;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    dotenv().ok();

    let config_build = Config::builder()
        .add_source(Environment::default())
        .build()
        .unwrap();

    let config: AppConfig = config_build.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::records::get_servises())
    })
    .bind(config.server_address.clone())
    .unwrap()
    .run();

    server.await
}
