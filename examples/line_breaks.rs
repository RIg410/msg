#![recursion_limit = "256"]
use msg::*;

fn main() {
    let message = msg! {
        bold { "Title" }
        "\n"
        "First line\n"
        "Second line\n"
        "\n"
        italic { "Italic text\nwith line break" }
        "\n"
        "\n"
        "End of message"
    };

    let generator = Generator::new(ParseMode::MarkdownV2);

    println!("Message elements:");
    for (i, element) in message.iter().enumerate() {
        println!("  {}: {:?}", i, element);
    }

    println!("\nGenerated message:");
    let mut output = String::new();
    for element in &message {
        match generator.generate(&mut output, element) {
            Ok(()) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    println!("{}", output);
}
