#![recursion_limit = "256"]
use tg_message_builder::*;

fn main() {
    let message = msg! {
        "Hello, "
        bold { "world" }
        "! "
        "This is "
        italic { "italic" }
        " text.\n"
        "Here's a "
        link("https://rust-lang.org") { "link to Rust" }
        ".\n"
        "And a mention: "
        @rustlang
        "\n"
        "With hashtag: "
        #rust
    };

    let generator = Generator::new(ParseMode::MarkdownV2);
    for element in &message {
        match generator.generate(element) {
            Ok(text) => print!("{}", text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    println!();

    let html_generator = Generator::new(ParseMode::Html);
    println!("\nHTML version:");
    for element in &message {
        match html_generator.generate(element) {
            Ok(text) => print!("{}", text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    println!();
}
