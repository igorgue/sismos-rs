use std::fs::File;
use std::io::Read;

use sismos::ineter::parse_html;

#[test]
fn test_parse_html() {
    let content = _get_test_content("sismos.0.php.html");
    let items = parse_html(content.as_str());
    let first = &items[0];

    assert_eq!(content.len(), 43371);
    assert_eq!(parse_html(&content).len(), 147);
    assert_eq!(first.created.to_string(), "2022-12-04 22:06:29 UTC");
}

fn _get_test_content(filename: &str) -> String {
    let path = format!("tests/data/{}", filename);
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    contents
}
