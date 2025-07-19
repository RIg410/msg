use msg::{msg, Generator, ParseMode};

fn main() {
    let msg1 = msg!("Hello, World!");
    println!("Simple text: {:?}", msg1);

    let msg2 = msg! {
        bold { "Bold text" }
        " "
        italic { "Italic text" }
    };
    println!("Formatted: {:?}", msg2);

    let msg3 = msg! {
        bold {
            "This is "
            italic { "bold and italic" }
            " text"
        }
    };
    println!("Nested: {:?}", msg3);

    let url = "https://example.com";
    let msg4 = msg! {
        "Check out "
        link(url) { "this link" }
        " and "
        @username
        " or "
        #hashtag
    };
    println!("Links: {:?}", msg4);

    let msg5 = msg! {
        "Inline "
        code { "code block" }
        " and pre:"
        "\n"
        pre("rust") { "fn main() { println!(\"Hello\"); }" }
    };
    println!("Code: {:?}", msg5);

    let msg6 = msg! {
        "My list:"
        "\n"
        list {
            - "First item" ;
            - "Second item" ;
            - "Third item"
        }
    };
    println!("List: {:?}", msg6);

    let msg7 = msg! {
        table {
            headers: ["Name", "Age", "City"]
            rows: [
                ["Alice", "25", "NYC"]
                ["Bob", "30", "LA"]
            ]
        }
    };
    println!("Table: {:?}", msg7);

    let phone_number = "234567890";
    let msg8 = msg! {
        "Phone: "
        +1(phone_number)
        "\n"
        "Email: "
        link("mailto:user@example.com") { "user@example.com" }
    };
    println!("Contact info: {:?}", msg8);

    let generator = Generator::new(ParseMode::MarkdownV2);
    let mut output = String::new();
    for element in &msg2 {
        match generator.generate(&mut output, element) {
            Ok(()) => {},
            Err(e) => println!("Error: {}", e),
        }
    }
    println!("Generated: {}", output);

    let element = msg!(bold { "Single bold element" });
    println!("Single element: {:?}", element);
}
