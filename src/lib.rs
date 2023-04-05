pub mod docs_rs_scraper;
pub mod services;

use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use services::scrape_services::scrape;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(scrape)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
