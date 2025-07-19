use tg_message_builder::*;

fn main() {
    let msg1 = msg! { "Hello, world!" };
    println!("msg1: {:?}", msg1);

    let bold_text = el!(bold { "Bold text" });
    println!("bold_text: {:?}", bold_text);

    let msg2 = vec![
        Element::text("Hello, "),
        Element::bold(vec![Element::text("world")]),
        Element::text("!"),
    ];

    let generator = Generator::new(ParseMode::MarkdownV2);
    for element in &msg2 {
        match generator.generate(element) {
            Ok(text) => print!("{}", text),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    println!();
}
