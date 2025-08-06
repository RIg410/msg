use msg::*;

fn main() {
    // Test basic message
    let msg1 = msg! { "Hello, world!" };
    println!("msg1: {:?}", msg1);

    // Test message concatenation with #msg syntax
    let msg = msg! { "foo" };
    let msg2 = msg! { "Start: " #msg.clone() " :End" };
    println!("msg2 (concatenated): {:?}", msg2);
    println!("Original msg still available: {:?}", msg);

    // Test with more complex message
    let greeting = msg! { bold { "Hello" } " from " italic { "Rust" } };
    let full_message = msg! { "Message: " #greeting.clone() " - Have a nice day!" };
    println!("full_message: {:?}", full_message);

    // Generate output for the concatenated message
    let generator = Generator::new(ParseMode::MarkdownV2);
    let mut output = String::new();
    for element in &full_message {
        match generator.generate(&mut output, element) {
            Ok(()) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    println!("Generated output: {}", output);
}
