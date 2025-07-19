use msg::*;

fn main() {
    let mut generator = Generator::new(ParseMode::MarkdownV2);

    generator.register_formatter(Box::new(formatter::PhoneFormatter));
    generator.register_formatter(Box::new(formatter::DateFormatter));
    generator.register_formatter(Box::new(formatter::TimeFormatter));
    generator.register_formatter(Box::new(formatter::EmailFormatter));
    generator.register_formatter(Box::new(formatter::ProgressFormatter));
    generator.register_formatter(Box::new(formatter::PercentFormatter));
    generator.register_formatter(Box::new(formatter::CurrencyFormatter::new(
        "₽".to_string(),
        "RUB".to_string(),
    )));

    let message = vec![
        Element::bold(vec![Element::text("Информация о заказе")]),
        Element::text("\n\n"),
        Element::text("Клиент: "),
        Element::text("Иван Иванов"),
        Element::text("\n"),
        Element::text("Телефон: "),
        Element::Custom {
            formatter: "phone".to_string(),
            value: "+7 (999) 123-45-67".to_string(),
        },
        Element::text("\n"),
        Element::text("Email: "),
        Element::Custom {
            formatter: "email".to_string(),
            value: "ivan@example.com".to_string(),
        },
        Element::text("\n"),
        Element::text("Дата заказа: "),
        Element::Custom {
            formatter: "date".to_string(),
            value: "2024-01-15".to_string(),
        },
        Element::text("\n"),
        Element::text("Время: "),
        Element::Custom {
            formatter: "time".to_string(),
            value: "14:30".to_string(),
        },
        Element::text("\n\n"),
        Element::text("Прогресс выполнения: "),
        Element::Custom {
            formatter: "progress".to_string(),
            value: "75".to_string(),
        },
        Element::text("\n"),
        Element::text("Скидка: "),
        Element::Custom {
            formatter: "percent".to_string(),
            value: "15".to_string(),
        },
        Element::text("\n"),
        Element::text("Сумма: "),
        Element::Custom {
            formatter: "RUB".to_string(),
            value: "1500".to_string(),
        },
    ];

    println!("=== MarkdownV2 ===");
    for element in &message {
        print!(
            "{}",
            generator
                .generate(element)
                .unwrap_or_else(|e| format!("[ERROR: {}]", e))
        );
    }

    println!("\n\n=== HTML ===");
    let html_generator = Generator::new(ParseMode::Html);
    for element in &message {
        print!(
            "{}",
            html_generator
                .generate(element)
                .unwrap_or_else(|e| format!("[ERROR: {}]", e))
        );
    }
}
