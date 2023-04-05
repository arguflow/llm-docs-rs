use actix_web::{test, App};
use llm_docs_rs::services::scrape_services::scrape;

#[actix_web::test]
async fn scrape_request() {
    let mut app = test::init_service(App::new().service(scrape)).await;

    // Test 404 for empty tail
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);

    // Test 404 for releases
    let req = test::TestRequest::get().uri("/releases").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}
