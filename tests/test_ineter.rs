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
    assert_eq!(first.created.month(), 12);
    assert_eq!(first.created.day(), 4);
    assert_eq!(first.created.hour(), 22);
    assert_eq!(first.created.minute(), 6);
    assert_eq!(first.created.second(), 29);
    assert_eq!(first.lat, "12.590");
    assert_eq!(first.long, "-90.132");
    assert_eq!(first.depth, "5");
    assert_eq!(first.richter, "4.1");
    assert_eq!(first.description, "C");
    assert_eq!(first.location, "116 Km al sur de Acajutla");
    assert_eq!(first.country, "El Salvador");
    assert_eq!(
        first.content_hash,
        "617216bf36050c6911b92a9d492b7625fa1c13e5e8f971e5b7e1a5bd1bac2778"
    );
    assert_eq!(
        first.partial_content_hash,
        "b3662eb31b134dfda89ea484a9724ba98652dcf68c02742b055a7d93eb879a18"
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
    let mut file = File::open(path).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    content
}
