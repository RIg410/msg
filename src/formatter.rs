use crate::error::Result;
use crate::generator::ParseMode;
use chrono::NaiveDate;

pub trait CustomFormatter: Send + Sync {
    fn name(&self) -> &str;
    fn format(&self, value: &str, mode: ParseMode) -> Result<String>;
    fn parse(&self, input: &str) -> Option<(String, usize)>;
}

pub struct PhoneFormatter;

impl CustomFormatter for PhoneFormatter {
    fn name(&self) -> &str {
        "phone"
    }

    fn format(&self, value: &str, mode: ParseMode) -> Result<String> {
        let formatted = match mode {
            ParseMode::MarkdownV2 => format!("`{}`", escape_markdown(value)),
            ParseMode::Html => format!("<code>{}</code>", escape_html(value)),
        };
        Ok(formatted)
    }

    fn parse(&self, input: &str) -> Option<(String, usize)> {
        let phone_regex = regex::Regex::new(r"^\+?[\d\s\-\(\)]+").ok()?;
        let mat = phone_regex.find(input)?;
        Some((mat.as_str().to_string(), mat.len()))
    }
}

pub struct DateFormatter;

impl CustomFormatter for DateFormatter {
    fn name(&self) -> &str {
        "date"
    }

    fn format(&self, value: &str, mode: ParseMode) -> Result<String> {
        let date = NaiveDate::parse_from_str(value, "%Y-%m-%d")
            .map(|d| d.format("%d.%m.%Y").to_string())
            .unwrap_or_else(|_| value.to_string());

        let formatted = match mode {
            ParseMode::MarkdownV2 => format!("`{}`", escape_markdown(&date)),
            ParseMode::Html => format!("<code>{}</code>", escape_html(&date)),
        };
        Ok(formatted)
    }

    fn parse(&self, input: &str) -> Option<(String, usize)> {
        let date_regex = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}").ok()?;
        let mat = date_regex.find(input)?;
        Some((mat.as_str().to_string(), mat.len()))
    }
}

pub struct TimeFormatter;

impl CustomFormatter for TimeFormatter {
    fn name(&self) -> &str {
        "time"
    }

    fn format(&self, value: &str, mode: ParseMode) -> Result<String> {
        let formatted = match mode {
            ParseMode::MarkdownV2 => format!("`{}`", escape_markdown(value)),
            ParseMode::Html => format!("<code>{}</code>", escape_html(value)),
        };
        Ok(formatted)
    }

    fn parse(&self, input: &str) -> Option<(String, usize)> {
        let time_regex = regex::Regex::new(r"^\d{1,2}:\d{2}(:\d{2})?").ok()?;
        let mat = time_regex.find(input)?;
        Some((mat.as_str().to_string(), mat.len()))
    }
}

pub struct EmailFormatter;

impl CustomFormatter for EmailFormatter {
    fn name(&self) -> &str {
        "email"
    }

    fn format(&self, value: &str, mode: ParseMode) -> Result<String> {
        let formatted = match mode {
            ParseMode::MarkdownV2 => format!("[✉️ {}](mailto:{})", escape_markdown(value), value),
            ParseMode::Html => format!("<a href=\"mailto:{}\">{}</a>", value, escape_html(value)),
        };
        Ok(formatted)
    }

    fn parse(&self, input: &str) -> Option<(String, usize)> {
        let email_regex =
            regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").ok()?;
        let mat = email_regex.find(input)?;
        Some((mat.as_str().to_string(), mat.len()))
    }
}

pub struct CurrencyFormatter {
    symbol: String,
    code: String,
}

impl CurrencyFormatter {
    pub fn new(symbol: String, code: String) -> Self {
        Self { symbol, code }
    }
}

impl CustomFormatter for CurrencyFormatter {
    fn name(&self) -> &str {
        &self.code
    }

    fn format(&self, value: &str, mode: ParseMode) -> Result<String> {
        let amount = value.parse::<f64>().unwrap_or(0.0);
        let formatted_amount = format!("{:.2} {}", amount, self.symbol);

        let formatted = match mode {
            ParseMode::MarkdownV2 => format!("`{}`", escape_markdown(&formatted_amount)),
            ParseMode::Html => format!("<code>{}</code>", escape_html(&formatted_amount)),
        };
        Ok(formatted)
    }

    fn parse(&self, input: &str) -> Option<(String, usize)> {
        let num_regex = regex::Regex::new(r"^[\d,\.]+").ok()?;
        let mat = num_regex.find(input)?;
        Some((mat.as_str().to_string(), mat.len()))
    }
}

pub struct PercentFormatter;

impl CustomFormatter for PercentFormatter {
    fn name(&self) -> &str {
        "percent"
    }

    fn format(&self, value: &str, mode: ParseMode) -> Result<String> {
        let percent = value.parse::<f64>().unwrap_or(0.0);
        let formatted_percent = format!("{:.1}%", percent);

        let formatted = match mode {
            ParseMode::MarkdownV2 => format!("`{}`", escape_markdown(&formatted_percent)),
            ParseMode::Html => format!("<code>{}</code>", escape_html(&formatted_percent)),
        };
        Ok(formatted)
    }

    fn parse(&self, input: &str) -> Option<(String, usize)> {
        let num_regex = regex::Regex::new(r"^[\d\.]+").ok()?;
        let mat = num_regex.find(input)?;
        Some((mat.as_str().to_string(), mat.len()))
    }
}

pub struct ProgressFormatter;

impl CustomFormatter for ProgressFormatter {
    fn name(&self) -> &str {
        "progress"
    }

    fn format(&self, value: &str, mode: ParseMode) -> Result<String> {
        let progress = value.parse::<u8>().unwrap_or(0).min(100);
        let filled = (progress as f32 / 10.0).round() as usize;
        let empty = 10 - filled;
        let bar = format!("{}{}", "▓".repeat(filled), "░".repeat(empty));
        let formatted_progress = format!("{} {}%", bar, progress);

        let formatted = match mode {
            ParseMode::MarkdownV2 => format!("`{}`", escape_markdown(&formatted_progress)),
            ParseMode::Html => format!("<code>{}</code>", escape_html(&formatted_progress)),
        };
        Ok(formatted)
    }

    fn parse(&self, input: &str) -> Option<(String, usize)> {
        let num_regex = regex::Regex::new(r"^\d{1,3}").ok()?;
        let mat = num_regex.find(input)?;
        Some((mat.as_str().to_string(), mat.len()))
    }
}

fn escape_markdown(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '_' | '*' | '[' | ']' | '(' | ')' | '~' | '`' | '>' | '#' | '+' | '-' | '=' | '|'
            | '{' | '}' | '.' | '!' => {
                format!("\\{}", c)
            }
            _ => c.to_string(),
        })
        .collect()
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
