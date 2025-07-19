#![recursion_limit = "256"]
use msg::*;

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
    let mut output = String::new();
    for element in &message {
        match generator.generate(&mut output, element) {
            Ok(_) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    println!("{}", output);

    let html_generator = Generator::new(ParseMode::Html);
    let mut html_output = String::new();
    println!("\nHTML version:");
    for element in &message {
        match html_generator.generate(&mut html_output, element) {
            Ok(_) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    println!("{}", html_output);
}
