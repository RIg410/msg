use msg::{msg, Element, Generator, ParseMode};

#[test]
fn test_phone_formatter_with_variable() {
    let phone = "79997353574";
    let message = msg! { +(phone) };
    
    assert_eq!(message.len(), 1);
    match &message[0] {
        Element::TextLink { text, url } => {
            assert_eq!(text, "+7(999) 735-35-74");
            assert_eq!(url, "tel:+79997353574");
        }
        _ => panic!("Expected TextLink element for phone, got: {:?}", message[0]),
    }
}

#[test]
fn test_phone_formatter_with_string_variable() {
    let phone = "79997353574";
    let message = msg! { +(phone) };
    
    assert_eq!(message.len(), 1);
    match &message[0] {
        Element::TextLink { text, url } => {
            assert_eq!(text, "+7(999) 735-35-74");
            assert_eq!(url, "tel:+79997353574");
        }
        _ => panic!("Expected TextLink element for phone"),
    }
}

#[test]
fn test_phone_formatter_with_prefix() {
    let phone = "79997353574";
    let message = msg! { "Call us: " +(phone) };
    
    assert_eq!(message.len(), 2);
    match &message[0] {
        Element::Text(text) => assert_eq!(text, "Call us: "),
        _ => panic!("Expected Text element first"),
    }
    match &message[1] {
        Element::TextLink { text, url } => {
            assert_eq!(text, "+7(999) 735-35-74");
            assert_eq!(url, "tel:+79997353574");
        }
        _ => panic!("Expected TextLink element for phone"),
    }
}

#[test]
fn test_phone_formatter_international_prefix() {
    let phone = "9997353574";
    let message = msg! { +7(phone) };
    
    assert_eq!(message.len(), 1);
    match &message[0] {
        Element::TextLink { text, url } => {
            assert_eq!(text, "+7 (999) 735-35-74");
            assert_eq!(url, "tel:+79997353574");
        }
        _ => panic!("Expected TextLink element for phone"),
    }
}

#[test]
fn test_phone_formatter_rendering_markdown() {
    let phone = "79997353574";
    let message = msg! { "Contact: " +(phone) };
    
    let generator = Generator::new(ParseMode::MarkdownV2);
    let mut output = String::new();
    for element in &message {
        generator.generate(&mut output, element).unwrap();
    }
    
    assert_eq!(output, "Contact: [\\+7\\(999\\) 735\\-35\\-74](tel:+79997353574)");
}

#[test]
fn test_phone_formatter_rendering_html() {
    let phone = "79997353574";
    let message = msg! { "Contact: " +(phone) };
    
    let generator = Generator::new(ParseMode::Html);
    let mut output = String::new();
    for element in &message {
        generator.generate(&mut output, element).unwrap();
    }
    
    assert_eq!(output, "Contact: <a href=\"tel:+79997353574\">+7(999) 735-35-74</a>");
}

#[test]
fn test_phone_formatter_complex_message() {
    let phone = "79997353574";
    let email = "user@example.com";
    let message = msg! {
        bold { "Contact Information" }
        "\n"
        "Phone: " +(phone)
        "\n"
        "Email: " (email)
    };
    
    // Phone creates TextLink, so we have 7 elements total:
    // 1. Bold with "Contact Information"
    // 2. Text with "\n"
    // 3. Text with "Phone: "
    // 4. TextLink with phone
    // 5. Text with "\n"
    // 6. Text with "Email: "
    // 7. Text with email
    assert_eq!(message.len(), 7);
    
    let generator = Generator::new(ParseMode::MarkdownV2);
    let mut output = String::new();
    for element in &message {
        generator.generate(&mut output, element).unwrap();
    }
    
    assert!(output.contains("*Contact Information*"));
    assert!(output.contains("tel:+79997353574"));
    assert!(output.contains("user@example\\.com"));
}

#[test]
fn test_phone_formatter_empty_string() {
    let phone = "";
    let message = msg! { +(phone) };
    
    assert_eq!(message.len(), 1);
    match &message[0] {
        Element::Text(text) => {
            assert_eq!(text, "-");
        }
        _ => panic!("Expected Text element with '-' for empty phone, got: {:?}", message[0]),
    }
}

#[test]
fn test_phone_formatter_as_requested() {
    // Test for the specific case requested by user: let phone = 79997353574; msg!{+(phone)}
    let phone = "79997353574";
    let message = msg! { +(phone) };
    
    // Check that phone number is formatted as a link
    assert_eq!(message.len(), 1);
    match &message[0] {
        Element::TextLink { text, url } => {
            // Phone is formatted with spaces and dashes
            assert_eq!(text, "+7(999) 735-35-74");
            // URL contains the clean phone number
            assert_eq!(url, "tel:+79997353574");
        }
        _ => panic!("Expected TextLink element for phone"),
    }
    
    // Check Markdown rendering
    let generator = Generator::new(ParseMode::MarkdownV2);
    let mut output = String::new();
    for element in &message {
        generator.generate(&mut output, element).unwrap();
    }
    // In Markdown, it renders as a clickable link with escaped special characters
    assert_eq!(output, "[\\+7\\(999\\) 735\\-35\\-74](tel:+79997353574)");
    
    // Check HTML rendering
    let generator = Generator::new(ParseMode::Html);
    let mut output_html = String::new();
    for element in &message {
        generator.generate(&mut output_html, element).unwrap();
    }
    // In HTML, it renders as an anchor tag
    assert_eq!(output_html, "<a href=\"tel:+79997353574\">+7(999) 735-35-74</a>");
}