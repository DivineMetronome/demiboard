mod config;
mod db;
mod handlers;
mod util;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web::route, App, HttpResponse, HttpServer};
use colored::Colorize;
use config::Config;
use handlers::{boards, catalog, new_post, new_thread, thread_subscribe};
use lazy_static::lazy_static;
use util::sse_thread::Broadcaster;

lazy_static! {
    static ref CONFIG: Config = Config::create();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let pool = match db::get_db_pool(&CONFIG.db_url).await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("{}: {}", "Couldn't connect to DB".red(), err);
            std::process::exit(1);
        }
    };

    let broadcaster = Broadcaster::create();

    CONFIG.print();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .app_data(broadcaster.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&CONFIG.private_key.clone().into_bytes())
                    .name("sid")
                    .path("/")
                    .secure(CONFIG.https),
            ))
            .service(boards)
            .service(catalog)
            .service(new_thread)
            .service(thread_subscribe)
            .service(new_post)
            .default_service(route().to(|| HttpResponse::MethodNotAllowed()))
    })
    .bind(&CONFIG.address)?
    .run()
    .await
}
