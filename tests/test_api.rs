use std::env;

use actix_web::{test, App};
use ctor;
use sismos::{api::root, models::SismoResponse};

#[ctor::ctor]
fn init() {
    let _ = std::fs::copy("tests/data/sismos.test.db", "tests/data/sismos.db");
    env::set_var("DATABASE_URL", "sqlite://tests/data/sismos.db");
}

#[actix_web::test]
async fn test_root() {
    let app = test::init_service(App::new().service(root)).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp: Vec<SismoResponse> = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp.len(), 5);
}
