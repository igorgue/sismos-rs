use std::env;
use actix_web::{test, App};
use sismos::api::root;

#[actix_web::test]
async fn test_root() {
    env::set_var("DATABASE_URL", "sqlite://tests/data/sismos.test.db");
    let app = test::init_service(
        App::new().service(root)
    ).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}
