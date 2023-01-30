use std::fs::File;
use std::io::Read;

use chrono::{Datelike, Timelike};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use sismos::ineter::{get_data_from_api, parse_html};

#[test]
fn test_parse_html() {
    let content = get_test_content("sismos.0.php.html");
    let items = parse_html(content.as_str());

    assert_eq!(content.len(), 43371);
    assert_eq!(items.len(), 147);

    let first = &items[0];
    assert_eq!(first.created.year(), 2022);
    assert_eq!(first.created.month(), 11);
    assert_eq!(first.created.day(), 24);
    assert_eq!(first.created.hour(), 22);
    assert_eq!(first.created.minute(), 30);
    assert_eq!(first.created.second(), 01);
    assert_eq!(first.lat, "12.914");
    assert_eq!(first.long, "-88.963");
    assert_eq!(first.depth, "31");
    assert_eq!(first.richter, "2.9");
    assert_eq!(first.description, "C");
    assert_eq!(first.location, "41 Km al sur de Delta del Rio Lempa");
    assert_eq!(first.country, "El Salvador");
    assert_eq!(
        first.content_hash,
        "e59cff63fea49ed0876cec8bd8e6adb57e0e8bf8691e5b1a9c2024fbe1ba6e40"
    );
    assert_eq!(
        first.partial_content_hash,
        "a44bf084d01d30a297000ddebd56d5aee39ea89394d4561f67d634d05379a594"
    )
}

#[actix_web::test]
async fn test_get_data_from_api() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/geofisica/sis/events/sismos.php"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            get_test_content("sismos.0.php.html").as_bytes(),
            "text/html",
        ))
        .mount(&mock_server)
        .await;

    let url = format!(
        "{}{}",
        mock_server.uri().as_str(),
        "/geofisica/sis/events/sismos.php"
    );
    let items = get_data_from_api(Some(url.as_str())).await.unwrap();

    assert_eq!(items.len(), 147);
}

fn get_test_content(filename: &str) -> String {
    let path = format!("tests/data/{}", filename);
    let mut file = File::open(path).expect(format!("File not found: {}", filename).as_str());
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    content
}
