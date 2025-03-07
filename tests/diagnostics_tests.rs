use systemd_language_server::parse_unit_file;

#[test]
fn test_syntax_error_detection() {
    // Test syntax error detection
    let content = "[Unit\nDescription=Service with syntax error\n";
    let result = parse_unit_file(content);
    assert!(
        result.is_err(),
        "Should detect missing closing bracket in section header"
    );
}

#[test]
fn test_valid_file_parsing() {
    // Test valid file parsing
    let content = "[Unit]\nDescription=Valid Service\n";
    let result = parse_unit_file(content);
    assert!(
        result.is_ok(),
        "Should parse valid unit file without errors"
    );

    let ini = result.unwrap();
    assert_eq!(ini.get("Unit", "Description").unwrap(), "Valid Service");
}

#[test]
fn test_empty_value_parsing() {
    // Test empty value parsing
    let content = "[Unit]\nDescription=\n";
    let result = parse_unit_file(content);
    assert!(result.is_ok(), "Should parse file with empty value");

    let ini = result.unwrap();
    assert_eq!(ini.get("Unit", "Description").unwrap(), "");
}

#[test]
fn test_invalid_key_value_format() {
    // Test invalid key-value format
    let content = "[Unit]\n[Description Test Service\n";
    let result = parse_unit_file(content);
    assert!(result.is_err(), "Should detect missing equals sign");
}

#[test]
fn test_comment_handling() {
    // Test comment handling
    let content =
        "# This is a comment\n[Unit]\n# Another comment\nDescription=Service with comments\n";
    let result = parse_unit_file(content);
    assert!(result.is_ok(), "Should parse file with comments");

    let ini = result.unwrap();
    assert_eq!(
        ini.get("Unit", "Description").unwrap(),
        "Service with comments"
    );
}
