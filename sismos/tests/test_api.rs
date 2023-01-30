use actix_web::{test, App};
use ctor;
use sismos::{api::root, models::SismoResponse};

mod utils;

#[ctor::ctor]
fn init() {
    utils::recreate_sqlite_db();
}

#[actix_web::test]
async fn test_root() {
    let app = test::init_service(App::new().service(root)).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp: Vec<SismoResponse> = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp.len(), 5);

    let first = &resp[0];
    assert_eq!(first.id, 400);
}
