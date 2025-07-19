use chrono::{Local, NaiveDate, NaiveTime};
use msg::{msg, Generator, ParseMode};

fn main() {
    let phone_number = "9991234567";
    let msg1 = msg! {
        "Contact us: "
        +7(phone_number)
        " or "
        +8("8001234567")
        " or "
        +("4951234567")
    };
    println!("Phone numbers: {:?}", msg1);

    let now = Local::now();
    let msg2 = msg! {
        "Today: "
        date(now.date_naive())
        "\n"
        "Current time: "
        time(now.time())
        "\n"
        "Full datetime: "
        datetime(now.naive_local())
    };
    println!("Date/Time: {:?}", msg2);

    let msg3 = msg! {
        "Check out https://example.com and also visit https://github.com/rust-lang/rust for more info"
    };
    println!("Auto-captured URLs: {:?}", msg3);

    let msg4 = msg! {
        bold { "Important Information" }
        "\n\n"
        "📞 Call us: "
        +7("9991234567")
        "\n"
        "🌐 Website: https://example.com"
        "\n"
        "📅 Meeting: "
        date(NaiveDate::from_ymd_opt(2025, 1, 20).unwrap())
        " at "
        time(NaiveTime::from_hms_opt(15, 30, 0).unwrap())
    };
    println!("Combined: {:?}", msg4);

    let generator = Generator::new(ParseMode::MarkdownV2);
    println!("\nGenerated MarkdownV2:");
    let mut output = String::new();
    for element in &msg4 {
        match generator.generate(&mut output, element) {
            Ok(()) => {},
            Err(e) => println!("Error: {}", e),
        }
    }
    println!("{}", output);
}
