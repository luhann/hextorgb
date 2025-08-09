use hextorgb::*;

// Tests for hextorgb function
#[test]
fn test_hextorgb_basic_colors() {
    assert_eq!(hextorgb("#FF0000"), "RGB(255, 0, 0)");
    assert_eq!(hextorgb("#00FF00"), "RGB(0, 255, 0)");
    assert_eq!(hextorgb("#0000FF"), "RGB(0, 0, 255)");
    assert_eq!(hextorgb("#000000"), "RGB(0, 0, 0)");
    assert_eq!(hextorgb("#FFFFFF"), "RGB(255, 255, 255)");
}

#[test]
fn test_hextorgb_input_formats() {
    let expected = "RGB(255, 0, 0)";
    
    assert_eq!(hextorgb("#FF0000"), expected);
    assert_eq!(hextorgb("0xFF0000"), expected);
    assert_eq!(hextorgb("0XFF0000"), expected);
    assert_eq!(hextorgb("FF0000"), expected);
}

#[test]
fn test_hextorgb_case_insensitive() {
    assert_eq!(hextorgb("#ff0000"), "RGB(255, 0, 0)");
    assert_eq!(hextorgb("#FfAaBb"), "RGB(255, 170, 187)");
    assert_eq!(hextorgb("aabbcc"), "RGB(170, 187, 204)");
}

#[test]
fn test_hextorgb_alpha_channel() {
    assert_eq!(hextorgb("#FF0000AA"), "RGB(255, 0, 0, 0.67)");
    assert_eq!(hextorgb("#FF000080"), "RGB(255, 0, 0, 0.50)");
    assert_eq!(hextorgb("#0000FF00"), "RGB(0, 0, 255, 0.00)");
    assert_eq!(hextorgb("#00FF00FF"), "RGB(0, 255, 0, 1.00)");
}

#[test]
fn test_hextorgb_errors() {
    assert_eq!(hextorgb("#ZZZZZZ"), "Invalid hex");
    assert_eq!(hextorgb("#FFF"), "Invalid hex length");
    assert_eq!(hextorgb(""), "Invalid hex length");
    assert_eq!(hextorgb("#12345"), "Invalid hex length");
    assert_eq!(hextorgb("#123456789"), "Invalid hex length");
}

// Tests for parse_hex function
#[test]
fn test_parse_hex_6_digit() {
    let result = parse_hex("#FF0000").unwrap();
    assert_eq!(result.0, [255, 0, 0]);
    assert_eq!(result.1, None);
    
    let result = parse_hex("#00FF00").unwrap();
    assert_eq!(result.0, [0, 255, 0]);
    assert_eq!(result.1, None);
}

#[test]
fn test_parse_hex_8_digit() {
    let result = parse_hex("#FF0000AA").unwrap();
    assert_eq!(result.0, [255, 0, 0]);
    assert_eq!(result.1, Some(170));
    
    let result = parse_hex("#00FF0080").unwrap();
    assert_eq!(result.0, [0, 255, 0]);
    assert_eq!(result.1, Some(128));
}

#[test]
fn test_parse_hex_prefixes() {
    // Hash prefix
    let result = parse_hex("#FF0000").unwrap();
    assert_eq!(result.0, [255, 0, 0]);
    
    // 0x prefix
    let result = parse_hex("0xFF0000").unwrap();
    assert_eq!(result.0, [255, 0, 0]);
    
    // 0X prefix
    let result = parse_hex("0XFF0000").unwrap();
    assert_eq!(result.0, [255, 0, 0]);
    
    // No prefix
    let result = parse_hex("FF0000").unwrap();
    assert_eq!(result.0, [255, 0, 0]);
}

#[test]
fn test_parse_hex_case_insensitive() {
    let result = parse_hex("#ff0000").unwrap();
    assert_eq!(result.0, [255, 0, 0]);
    
    let result = parse_hex("0xAABBCC").unwrap();
    assert_eq!(result.0, [170, 187, 204]);
    
    let result = parse_hex("#FfAaBb").unwrap();
    assert_eq!(result.0, [255, 170, 187]);
}

#[test]
fn test_parse_hex_errors() {
    assert_eq!(parse_hex("#ZZZZZZ"), Err("Invalid hex"));
    assert_eq!(parse_hex("#FFF"), Err("Invalid hex length"));
    assert_eq!(parse_hex(""), Err("Invalid hex length"));
    assert_eq!(parse_hex("#12345"), Err("Invalid hex length"));
    assert_eq!(parse_hex("#123456789"), Err("Invalid hex length"));
    assert_eq!(parse_hex("#GGGGGG"), Err("Invalid hex"));
}

// Tests for convert_hex_to_format function
#[test]
fn test_convert_hex_to_format_standard() {
    // Test standard format without alpha
    assert_eq!(
        convert_hex_to_format("#FF0000", "standard").unwrap(),
        "RGB(255, 0, 0)"
    );
    
    // Test standard format with alpha
    assert_eq!(
        convert_hex_to_format("#FF0000AA", "standard").unwrap(),
        "RGBA(255, 0, 0, 0.67)"
    );
    
    // Test different colors
    assert_eq!(
        convert_hex_to_format("#00FF00", "standard").unwrap(),
        "RGB(0, 255, 0)"
    );
    
    assert_eq!(
        convert_hex_to_format("#0000FF80", "standard").unwrap(),
        "RGBA(0, 0, 255, 0.50)"
    );
}

#[test]
fn test_convert_hex_to_format_css() {
    // Test CSS format without alpha
    assert_eq!(
        convert_hex_to_format("#FF0000", "css").unwrap(),
        "rgb(255, 0, 0)"
    );
    
    // Test CSS format with alpha
    assert_eq!(
        convert_hex_to_format("#FF0000AA", "css").unwrap(),
        "rgba(255, 0, 0, 0.67)"
    );
    
    // Test edge cases
    assert_eq!(
        convert_hex_to_format("#000000", "css").unwrap(),
        "rgb(0, 0, 0)"
    );
    
    assert_eq!(
        convert_hex_to_format("#FFFFFF00", "css").unwrap(),
        "rgba(255, 255, 255, 0.00)"
    );
}

#[test]
fn test_convert_hex_to_format_json() {
    // Test JSON format without alpha
    assert_eq!(
        convert_hex_to_format("#FF0000", "json").unwrap(),
        r#"{"r": 255, "g": 0, "b": 0}"#
    );
    
    // Test JSON format with alpha
    assert_eq!(
        convert_hex_to_format("#FF0000AA", "json").unwrap(),
        r#"{"r": 255, "g": 0, "b": 0, "a": 0.67}"#
    );
    
    // Test different values
    assert_eq!(
        convert_hex_to_format("#80C0E0", "json").unwrap(),
        r#"{"r": 128, "g": 192, "b": 224}"#
    );
    
    assert_eq!(
        convert_hex_to_format("#80C0E040", "json").unwrap(),
        r#"{"r": 128, "g": 192, "b": 224, "a": 0.25}"#
    );
}

#[test]
fn test_convert_hex_to_format_hex() {
    // Test hex format without alpha (normalization)
    assert_eq!(
        convert_hex_to_format("#ff0000", "hex").unwrap(),
        "#FF0000"
    );
    
    // Test hex format with alpha
    assert_eq!(
        convert_hex_to_format("#FF0000AA", "hex").unwrap(),
        "#FF0000AA"
    );
    
    // Test different input formats
    assert_eq!(
        convert_hex_to_format("0xFF0000", "hex").unwrap(),
        "#FF0000"
    );
    
    assert_eq!(
        convert_hex_to_format("00ff00", "hex").unwrap(),
        "#00FF00"
    );
    
    // Test alpha normalization
    assert_eq!(
        convert_hex_to_format("#FF000080", "hex").unwrap(),
        "#FF000080"
    );
}

#[test]
fn test_convert_hex_to_format_compact() {
    // Test compact format without alpha
    assert_eq!(
        convert_hex_to_format("#FF0000", "compact").unwrap(),
        "255,0,0"
    );
    
    // Test compact format with alpha
    assert_eq!(
        convert_hex_to_format("#FF0000AA", "compact").unwrap(),
        "255,0,0,0.67"
    );
    
    // Test different values
    assert_eq!(
        convert_hex_to_format("#123456", "compact").unwrap(),
        "18,52,86"
    );
    
    assert_eq!(
        convert_hex_to_format("#12345678", "compact").unwrap(),
        "18,52,86,0.47"
    );
}

#[test]
fn test_convert_hex_to_format_all_input_formats() {
    let expected_standard = "RGB(255, 0, 0)";
    
    // Test all input format variations produce the same output
    assert_eq!(convert_hex_to_format("#FF0000", "standard").unwrap(), expected_standard);
    assert_eq!(convert_hex_to_format("0xFF0000", "standard").unwrap(), expected_standard);
    assert_eq!(convert_hex_to_format("0XFF0000", "standard").unwrap(), expected_standard);
    assert_eq!(convert_hex_to_format("FF0000", "standard").unwrap(), expected_standard);
    assert_eq!(convert_hex_to_format("#ff0000", "standard").unwrap(), expected_standard);
    assert_eq!(convert_hex_to_format("ff0000", "standard").unwrap(), expected_standard);
}

#[test]
fn test_convert_hex_to_format_alpha_precision() {
    // Test specific alpha values for precision
    assert_eq!(
        convert_hex_to_format("#FF000000", "standard").unwrap(),
        "RGBA(255, 0, 0, 0.00)"
    );
    
    assert_eq!(
        convert_hex_to_format("#FF000001", "standard").unwrap(),
        "RGBA(255, 0, 0, 0.00)"  // Rounds to 0.00
    );
    
    assert_eq!(
        convert_hex_to_format("#FF000080", "standard").unwrap(),
        "RGBA(255, 0, 0, 0.50)"
    );
    
    assert_eq!(
        convert_hex_to_format("#FF0000FF", "standard").unwrap(),
        "RGBA(255, 0, 0, 1.00)"
    );
}

#[test]
fn test_convert_hex_to_format_errors() {
    // Test invalid format
    assert_eq!(
        convert_hex_to_format("#FF0000", "invalid").unwrap_err(),
        "Unknown format: invalid"
    );
    
    // Test invalid hex input
    assert_eq!(
        convert_hex_to_format("#ZZZZZZ", "standard").unwrap_err(),
        "Invalid hex"
    );
    
    // Test invalid length
    assert_eq!(
        convert_hex_to_format("#FFF", "standard").unwrap_err(),
        "Invalid hex length"
    );
    
    // Test empty input
    assert_eq!(
        convert_hex_to_format("", "standard").unwrap_err(),
        "Invalid hex length"
    );
}

#[test]
fn test_convert_hex_to_format_comprehensive() {
    // Test a comprehensive example across all formats
    let hex = "#8A2BE2CC";  // BlueViolet with alpha
    
    assert_eq!(
        convert_hex_to_format(hex, "standard").unwrap(),
        "RGBA(138, 43, 226, 0.80)"
    );
    
    assert_eq!(
        convert_hex_to_format(hex, "css").unwrap(),
        "rgba(138, 43, 226, 0.80)"
    );
    
    assert_eq!(
        convert_hex_to_format(hex, "json").unwrap(),
        r#"{"r": 138, "g": 43, "b": 226, "a": 0.80}"#
    );
    
    assert_eq!(
        convert_hex_to_format(hex, "hex").unwrap(),
        "#8A2BE2CC"
    );
    
    assert_eq!(
        convert_hex_to_format(hex, "compact").unwrap(),
        "138,43,226,0.80"
    );
}