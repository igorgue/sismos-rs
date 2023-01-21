use std::fs::File;
use std::io::Read;

use sismos::ineter::parse_html;

#[test]
fn test_parse_html() {
    let content = _get_test_content("sismos.0.php.html");

    assert_eq!(content.len(), 43371);
    assert_eq!(parse_html(&content).len(), 10);
}

fn _get_test_content(filename: &str) -> String {
    let path = format!("tests/data/{}", filename);
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    contents
}
