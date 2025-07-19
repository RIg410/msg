use tg_message_builder::*;

fn main() {
    let msg1 = vec![
        Element::text("Привет, "),
        Element::bold(vec![Element::text("мир")]),
        Element::text("!\n"),
    ];

    let msg2 = vec![
        Element::bold(vec![Element::text("Заголовок")]),
        Element::text("\n\n"),
        Element::text("Обычный текст с "),
        Element::italic(vec![Element::text("курсивом")]),
        Element::text(" и "),
        Element::code("inline кодом"),
        Element::text(".\n\n"),
        Element::link(
            vec![Element::text("Ссылка на Rust")],
            "https://rust-lang.org",
        ),
        Element::text("\n"),
        Element::mention("rustlang"),
        Element::text(" "),
        Element::hashtag("rust"),
        Element::text("\n\n"),
        Element::pre(
            "fn main() {\n    println!(\"Hello!\");\n}",
            Some("rust".to_string()),
        ),
    ];

    let list = Element::List(ListNode {
        style: ListStyle::Bullet,
        items: vec![
            ListItem {
                content: vec![Element::text("Первый пункт")],
                nested: None,
            },
            ListItem {
                content: vec![Element::text("Второй пункт")],
                nested: Some(Box::new(ListNode {
                    style: ListStyle::Numbered,
                    items: vec![
                        ListItem {
                            content: vec![Element::text("Подпункт 1")],
                            nested: None,
                        },
                        ListItem {
                            content: vec![Element::text("Подпункт 2")],
                            nested: None,
                        },
                    ],
                })),
            },
        ],
    });

    let generator = Generator::new(ParseMode::MarkdownV2);

    println!("=== Пример 1 ===");
    for element in &msg1 {
        print!("{}", generator.generate(element).unwrap());
    }

    println!("\n=== Пример 2 ===");
    for element in &msg2 {
        print!("{}", generator.generate(element).unwrap());
    }

    println!("\n=== Список ===");
    println!("{}", generator.generate(&list).unwrap());

    let html_generator = Generator::new(ParseMode::Html);

    println!("\n=== HTML версия примера 1 ===");
    for element in &msg1 {
        print!("{}", html_generator.generate(element).unwrap());
    }
}
