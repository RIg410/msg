use chrono::{Local, NaiveDate, NaiveTime};
use msg::{msg, Generator, ParseMode};

fn main() {
    let now = Local::now();
    let msg1 = msg! {
        "Today is "
        date(now.date_naive())
        " at "
        time(now.time())
    };
    println!("Current date/time: {:?}", msg1);

    let specific_date = NaiveDate::from_ymd_opt(2025, 1, 16).unwrap();
    let specific_time = NaiveTime::from_hms_opt(14, 30, 0).unwrap();

    let msg2 = msg! {
        "Meeting scheduled for "
        date(specific_date)
        " at "
        time(specific_time)
    };
    println!("Specific date/time: {:?}", msg2);

    let msg3 = msg! {
        bold { "Important reminder:" }
        "\n"
        "📅 Date: "
        date(Local::now())
        "\n"
        "⏰ Time: "
        time(Local::now())
        "\n"
        "📞 Phone: "
        +7("99900000340")
        "\n"
        "📧 Email: "
        link("mailto:contact@example.com") { "contact@example.com" }
    };
    println!("Combined message: {:?}", msg3);

    let generator = Generator::new(ParseMode::MarkdownV2);
    println!("\nGenerated MarkdownV2:");
    let mut output = String::new();
    for element in &msg3 {
        match generator.generate(&mut output, element) {
            Ok(()) => {}
            Err(e) => println!("Error: {}", e),
        }
    }
    println!("{}", output);
}
