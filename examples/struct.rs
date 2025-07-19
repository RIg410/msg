use msg::msg;
use msg::{Generator, ParseMode};

pub struct User {
    name: String,
    id: u64,
}

fn main() {
    let user = User {
        name: "Alice".to_string(),
        id: 12345,
    };

    let message = msg! {
        bold { "User Information" }
        "\n"
        "Name: " bold {user.name}
        "\n"
        "ID: " user.id
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
            Ok(()) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    println!("{}", output);
    println!("User message: {:?}", message);
}
