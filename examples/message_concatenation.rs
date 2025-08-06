use msg::*;

fn main() {
    println!("=== Message Concatenation Tests ===\n");

    // Test 1: Simple concatenation
    let simple = msg! { "Hello" };
    let concatenated = msg! { #simple.clone() " World!" };
    println!("Test 1 - Simple concatenation:");
    println!("  simple: {:?}", simple);
    println!("  concatenated: {:?}", concatenated);
    assert_eq!(concatenated.len(), 2);
    println!("  ✓ Passed\n");

    // Test 2: Multiple references
    let part1 = msg! { "Part 1" };
    let part2 = msg! { " and Part 2" };
    let combined = msg! { #part1.clone() #part2.clone() " - Done!" };
    println!("Test 2 - Multiple references:");
    println!("  combined: {:?}", combined);
    assert_eq!(combined.len(), 3);
    println!("  ✓ Passed\n");

    // Test 3: Nested formatting
    let bold_text = msg! { bold { "Bold" } };
    let italic_text = msg! { italic { "Italic" } };
    let mixed = msg! { "Normal " #bold_text.clone() " and " #italic_text.clone() " text" };
    println!("Test 3 - Mixed formatting:");
    println!("  mixed: {:?}", mixed);
    assert_eq!(mixed.len(), 5);
    println!("  ✓ Passed\n");

    // Test 4: Complex nested structure
    let header = msg! { bold { "Header:" } };
    let list_item1 = msg! { "• Item 1" };
    let list_item2 = msg! { "• Item 2" };
    let document = msg! {
        #header.clone() "\n"
        #list_item1.clone() "\n"
        #list_item2.clone()
    };
    println!("Test 4 - Complex structure:");
    println!("  document: {:?}", document);
    assert_eq!(document.len(), 5);
    println!("  ✓ Passed\n");

    // Test 5: Generate Markdown output
    let styled_msg = msg! {
        bold { "Title" } "\n"
        italic { "Subtitle" } "\n"
        "Regular text"
    };
    let final_msg = msg! {
        "Document:\n"
        #styled_msg.clone()
        "\n---\nEnd"
    };

    let generator = Generator::new(ParseMode::MarkdownV2);
    let mut output = String::new();
    for element in &final_msg {
        generator.generate(&mut output, element).unwrap();
    }
    println!("Test 5 - Generated Markdown:");
    println!("  final_msg: {:?}", final_msg);
    println!("  Markdown output:\n{}", output);
    println!("  ✓ Passed\n");

    // Test 6: Function returning messages
    fn create_greeting(name: &str) -> Vec<Element> {
        msg! { "Hello, " bold { (name) } "!" }
    }

    let greeting = create_greeting("Alice");
    let full_greeting = msg! { #greeting.clone() " Welcome to Rust!" };
    println!("Test 6 - Function composition:");
    println!("  full_greeting: {:?}", full_greeting);
    assert_eq!(full_greeting.len(), 4);
    println!("  ✓ Passed\n");

    println!("=== All tests passed! ===");
}
