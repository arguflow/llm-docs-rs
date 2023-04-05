use crate::docs_rs_scraper::scrape_site;
use actix_web::{get, HttpRequest, HttpResponse};

#[get("/{tail:.*}")]
pub async fn scrape(req: HttpRequest) -> HttpResponse {
    let tail = req.match_info().get("tail").unwrap_or("");
    if tail.is_empty() || tail.contains("releases") || tail == "/" {
        return HttpResponse::NotFound().finish();
    }

    let docs_rs_url = format!("https://docs.rs/{}", tail);
    scrape_site(&docs_rs_url).await.unwrap();

    HttpResponse::NoContent().finish()
}
