use hextorgb::*;

#[test]
fn test_full_conversion_workflow() {
    let test_cases = vec![
        ("#FF0000", "RGB(255, 0, 0)"),
        ("0x00FF00", "RGB(0, 255, 0)"),
        ("0000FF", "RGB(0, 0, 255)"),
        ("0000FF00", "RGB(0, 0, 255, 0.00)"),
    ];

    for (input, expected) in test_cases {
        assert_eq!(hextorgb(input), expected);
    }
}

#[test]
fn test_edge_cases() {
    // Test alpha channel
    assert_eq!(hextorgb("#FF0000AA"), "RGB(255, 0, 0, 0.67)");

    // Test lowercase
    assert_eq!(hextorgb("#ff0000"), "RGB(255, 0, 0)");

    // Test error cases
    assert_eq!(hextorgb("#ZZZZZZ"), "Invalid hex");
    assert_eq!(hextorgb("#FFF"), "Invalid hex length");
}
