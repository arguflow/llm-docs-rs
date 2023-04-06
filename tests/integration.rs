use actix_web::{test, App};
use llm_docs_rs::services::scrape_services::scrape;

#[actix_web::test]
async fn build_docs() {
    let mut app = test::init_service(App::new().service(scrape)).await;

    // Test 404 for empty tail
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);

    // Test 404 for releases
    let req = test::TestRequest::get().uri("/releases").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);

    // Test that docs are built
    let path = std::path::Path::new("./embedding-docs/ratchet_core");
    if path.is_dir() {
        let remove_result = std::fs::remove_dir_all(path);
        assert!(remove_result.is_ok());
    }

    let req = test::TestRequest::get().uri("/ratchet_core/0.3.0/ratchet_core").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 204);
}
