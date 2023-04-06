use crate::docs_rs_builder::build_docs;
use actix_web::{get, rt::spawn, HttpRequest, HttpResponse};
use log::{error, info};

#[get("/{tail:.*}")]
pub async fn scrape(req: HttpRequest) -> HttpResponse {
    let tail = req.match_info().get("tail").unwrap_or("");
    if tail.is_empty() || tail.contains("releases") || tail == "/" {
        return HttpResponse::NotFound().finish();
    }

    let crate_name = tail.split('/').next().unwrap_or("").to_owned();
    if crate_name.is_empty() {
        return HttpResponse::NotFound().finish();
    }
    spawn(async move {
        let result = build_docs(&crate_name).await;
        match result {
            Ok(_) => info!("Successfully built docs for {}", crate_name),
            Err(e) => error!("Failed to build docs for {}: {}", crate_name, e),
        }
    });

    HttpResponse::NoContent().finish()
}
