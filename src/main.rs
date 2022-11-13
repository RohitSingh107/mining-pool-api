#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;

use {
    actix_web::web::Data,
    actix_web::{middleware, App, HttpResponse, HttpServer},
    diesel::r2d2::ConnectionManager,
    diesel::PgConnection,
    r2d2::{Pool, PooledConnection},
    std::{env, io},
};

mod miner;
mod miner_controller;
mod util;
mod wallet;
mod wallet_controller;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(wallet_controller::list_wallets)
            .service(wallet_controller::get_wallet)
            .service(wallet_controller::create_wallet)
            .service(miner_controller::list_miners)
            .service(miner_controller::get_miner)
            .service(miner_controller::create_miner)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
