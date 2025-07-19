use crate::ast::*;
use crate::error::{Error, Result};
use crate::formatter::CustomFormatter;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseMode {
    MarkdownV2,
    Html,
}

pub trait Generate {
    fn generate(&self, mode: ParseMode) -> Result<String>;
}

pub struct Generator {
    mode: ParseMode,
    formatters: HashMap<String, Box<dyn CustomFormatter>>,
}

impl Generator {
    pub fn new(mode: ParseMode) -> Self {
        Self {
            mode,
            formatters: HashMap::new(),
        }
    }

    pub fn register_formatter(&mut self, formatter: Box<dyn CustomFormatter>) {
        self.formatters
            .insert(formatter.name().to_string(), formatter);
    }

    pub fn generate(&self, element: &Element) -> Result<String> {
        self.generate_element(element, self.mode)
    }

    fn generate_element(&self, element: &Element, mode: ParseMode) -> Result<String> {
        match element {
            Element::Text(text) => Ok(escape_text(text, mode)),

            Element::Bold(elements) => {
                let content = self.generate_elements(elements, mode)?;
                match mode {
                    ParseMode::MarkdownV2 => Ok(format!("*{}*", content)),
                    ParseMode::Html => Ok(format!("<b>{}</b>", content)),
                }
            }

            Element::Italic(elements) => {
                let content = self.generate_elements(elements, mode)?;
                match mode {
                    ParseMode::MarkdownV2 => Ok(format!("_{}_", content)),
                    ParseMode::Html => Ok(format!("<i>{}</i>", content)),
                }
            }

            Element::Code(code) => match mode {
                ParseMode::MarkdownV2 => Ok(format!("`{}`", escape_code(code))),
                ParseMode::Html => Ok(format!("<code>{}</code>", escape_html(code))),
            },

            Element::Pre(block) => match mode {
                ParseMode::MarkdownV2 => {
                    if let Some(lang) = &block.language {
                        Ok(format!(
                            "```{}\n{}\n```",
                            lang,
                            escape_pre(block.code.as_str())
                        ))
                    } else {
                        Ok(format!("```\n{}\n```", escape_pre(block.code.as_str())))
                    }
                }
                ParseMode::Html => {
                    if let Some(lang) = &block.language {
                        Ok(format!(
                            "<pre><code class=\"language-{}\">{}</code></pre>",
                            escape_html(lang),
                            escape_html(&block.code)
                        ))
                    } else {
                        Ok(format!("<pre>{}</pre>", escape_html(&block.code)))
                    }
                }
            },

            Element::Underline(elements) => {
                let content = self.generate_elements(elements, mode)?;
                match mode {
                    ParseMode::MarkdownV2 => Ok(format!("__{}__", content)),
                    ParseMode::Html => Ok(format!("<u>{}</u>", content)),
                }
            }

            Element::Strikethrough(elements) => {
                let content = self.generate_elements(elements, mode)?;
                match mode {
                    ParseMode::MarkdownV2 => Ok(format!("~~{}~~", content)),
                    ParseMode::Html => Ok(format!("<s>{}</s>", content)),
                }
            }

            Element::Spoiler(elements) => {
                let content = self.generate_elements(elements, mode)?;
                match mode {
                    ParseMode::MarkdownV2 => Ok(format!("||{}||", content)),
                    ParseMode::Html => Ok(format!("<tg-spoiler>{}</tg-spoiler>", content)),
                }
            }

            Element::Link { text, url } => {
                let text_content = self.generate_elements(text, mode)?;
                match mode {
                    ParseMode::MarkdownV2 => Ok(format!("[{}]({})", text_content, escape_url(url))),
                    ParseMode::Html => Ok(format!(
                        "<a href=\"{}\">{}</a>",
                        escape_html(url),
                        text_content
                    )),
                }
            }

            Element::TextLink { text, url } => match mode {
                ParseMode::MarkdownV2 => Ok(format!(
                    "[{}]({})",
                    escape_text(text, mode),
                    escape_url(url)
                )),
                ParseMode::Html => Ok(format!(
                    "<a href=\"{}\">{}</a>",
                    escape_html(url),
                    escape_html(text)
                )),
            },

            Element::Mention { username } => Ok(format!("@{}", username)),

            Element::MentionId { user_id, text } => match mode {
                ParseMode::MarkdownV2 => Ok(format!(
                    "[{}](tg://user?id={})",
                    escape_text(text, mode),
                    user_id
                )),
                ParseMode::Html => Ok(format!(
                    "<a href=\"tg://user?id={}\">{}</a>",
                    user_id,
                    escape_html(text)
                )),
            },

            Element::Hashtag(tag) => Ok(format!("#{}", tag)),

            Element::Command { name, args } => {
                if args.is_empty() {
                    Ok(format!("/{}", name))
                } else {
                    Ok(format!("/{} {}", name, args.join(" ")))
                }
            }

            Element::Emoji(emoji) => Ok(emoji.clone()),

            Element::CustomEmoji { emoji, id } => match mode {
                ParseMode::MarkdownV2 => Ok(format!("![{}](tg://emoji?id={})", emoji, id)),
                ParseMode::Html => Ok(format!(
                    "<tg-emoji emoji-id=\"{}\">{}</tg-emoji>",
                    id, emoji
                )),
            },

            Element::List(list) => self.generate_list(list, mode),

            Element::Table(table) => self.generate_table(table, mode),

            Element::Quote(elements) => {
                let content = self.generate_elements(elements, mode)?;
                match mode {
                    ParseMode::MarkdownV2 => Ok(format!(
                        ">{}",
                        content.lines().collect::<Vec<_>>().join("\n>")
                    )),
                    ParseMode::Html => Ok(format!("<blockquote>{}</blockquote>", content)),
                }
            }

            Element::Custom { formatter, value } => {
                if let Some(fmt) = self.formatters.get(formatter) {
                    fmt.format(value, mode)
                } else {
                    Err(Error::FormatterNotFound(formatter.clone()))
                }
            }

            Element::Group(elements) => self.generate_elements(elements, mode),
        }
    }

    fn generate_elements(&self, elements: &[Element], mode: ParseMode) -> Result<String> {
        elements
            .iter()
            .map(|e| self.generate_element(e, mode))
            .collect::<Result<Vec<_>>>()
            .map(|v| v.join(""))
    }

    fn generate_list(&self, list: &ListNode, mode: ParseMode) -> Result<String> {
        let mut result = String::new();

        for (i, item) in list.items.iter().enumerate() {
            let prefix = match &list.style {
                ListStyle::Bullet => "• ".to_string(),
                ListStyle::Numbered => format!("{}. ", i + 1),
                ListStyle::Custom(marker) => format!("{} ", marker),
            };

            result.push_str(&prefix);
            result.push_str(&self.generate_elements(&item.content, mode)?);

            if let Some(nested) = &item.nested {
                result.push('\n');
                let nested_content = self.generate_list(nested, mode)?;
                for line in nested_content.lines() {
                    result.push_str("  ");
                    result.push_str(line);
                    result.push('\n');
                }
                result.pop();
            }

            if i < list.items.len() - 1 {
                result.push('\n');
            }
        }

        Ok(result)
    }

    fn generate_table(&self, table: &TableNode, mode: ParseMode) -> Result<String> {
        let all_rows: Vec<&[TableCell]> = std::iter::once(table.headers.as_slice())
            .chain(table.rows.iter().map(|r| r.cells.as_slice()))
            .collect();

        let col_widths = calculate_column_widths(&all_rows, mode)?;

        match table.style {
            TableStyle::Unicode => self.generate_unicode_table(table, &col_widths, mode),
            TableStyle::Ascii => self.generate_ascii_table(table, &col_widths, mode),
            TableStyle::Minimal => self.generate_minimal_table(table, &col_widths, mode),
            TableStyle::Compact => self.generate_compact_table(table, &col_widths, mode),
        }
    }

    fn generate_unicode_table(
        &self,
        table: &TableNode,
        col_widths: &[usize],
        mode: ParseMode,
    ) -> Result<String> {
        let mut result = String::new();

        result.push_str(&format!(
            "```\n┌{}┐\n",
            col_widths
                .iter()
                .map(|&w| "─".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("┬")
        ));

        result.push_str(&self.format_table_row(&table.headers, col_widths, mode, "│")?);
        result.push_str(&format!(
            "\n├{}┤\n",
            col_widths
                .iter()
                .map(|&w| "─".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("┼")
        ));

        for row in &table.rows {
            result.push_str(&self.format_table_row(&row.cells, col_widths, mode, "│")?);
            result.push('\n');
        }

        result.push_str(&format!(
            "└{}┘\n```",
            col_widths
                .iter()
                .map(|&w| "─".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("┴")
        ));

        Ok(result)
    }

    fn generate_ascii_table(
        &self,
        table: &TableNode,
        col_widths: &[usize],
        mode: ParseMode,
    ) -> Result<String> {
        let mut result = String::new();

        result.push_str(&format!(
            "```\n+{}+\n",
            col_widths
                .iter()
                .map(|&w| "-".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("+")
        ));

        result.push_str(&self.format_table_row(&table.headers, col_widths, mode, "|")?);
        result.push_str(&format!(
            "\n+{}+\n",
            col_widths
                .iter()
                .map(|&w| "-".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("+")
        ));

        for row in &table.rows {
            result.push_str(&self.format_table_row(&row.cells, col_widths, mode, "|")?);
            result.push('\n');
        }

        result.push_str(&format!(
            "+{}+\n```",
            col_widths
                .iter()
                .map(|&w| "-".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("+")
        ));

        Ok(result)
    }

    fn generate_minimal_table(
        &self,
        table: &TableNode,
        col_widths: &[usize],
        mode: ParseMode,
    ) -> Result<String> {
        let mut result = String::new();
        result.push_str("```\n");

        result.push_str(&self.format_table_row(&table.headers, col_widths, mode, " ")?);
        result.push_str(&format!(
            "\n{}\n",
            col_widths
                .iter()
                .map(|&w| "─".repeat(w))
                .collect::<Vec<_>>()
                .join(" ")
        ));

        for row in &table.rows {
            result.push_str(&self.format_table_row(&row.cells, col_widths, mode, " ")?);
            result.push('\n');
        }

        result.push_str("```");
        Ok(result)
    }

    fn generate_compact_table(
        &self,
        table: &TableNode,
        col_widths: &[usize],
        mode: ParseMode,
    ) -> Result<String> {
        let mut result = String::new();
        result.push_str("```\n");

        result.push_str(&self.format_table_row(&table.headers, col_widths, mode, " ")?);
        result.push('\n');

        for row in &table.rows {
            result.push_str(&self.format_table_row(&row.cells, col_widths, mode, " ")?);
            result.push('\n');
        }

        result.push_str("```");
        Ok(result)
    }

    fn format_table_row(
        &self,
        cells: &[TableCell],
        col_widths: &[usize],
        _mode: ParseMode,
        separator: &str,
    ) -> Result<String> {
        let mut result = String::new();
        result.push_str(separator);

        for (i, cell) in cells.iter().enumerate() {
            if i < col_widths.len() {
                let content = cell
                    .content
                    .iter()
                    .map(|e| match e {
                        Element::Text(t) => t.clone(),
                        _ => String::new(),
                    })
                    .collect::<String>();

                let padded = match cell.align {
                    CellAlign::Left => format!(" {:<width$} ", content, width = col_widths[i]),
                    CellAlign::Center => format!(" {:^width$} ", content, width = col_widths[i]),
                    CellAlign::Right => format!(" {:>width$} ", content, width = col_widths[i]),
                };

                result.push_str(&padded);
                result.push_str(separator);
            }
        }

        Ok(result)
    }
}

fn calculate_column_widths(rows: &[&[TableCell]], _mode: ParseMode) -> Result<Vec<usize>> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let col_count = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut widths = vec![0; col_count];

    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() {
                let content_len = cell
                    .content
                    .iter()
                    .map(|e| match e {
                        Element::Text(t) => t.len(),
                        _ => 0,
                    })
                    .sum::<usize>();
                widths[i] = widths[i].max(content_len);
            }
        }
    }

    Ok(widths)
}

fn escape_text(text: &str, mode: ParseMode) -> String {
    match mode {
        ParseMode::MarkdownV2 => text
            .chars()
            .map(|c| match c {
                '_' | '*' | '[' | ']' | '(' | ')' | '~' | '`' | '>' | '#' | '+' | '-' | '='
                | '|' | '{' | '}' | '.' | '!' => {
                    format!("\\{}", c)
                }
                _ => c.to_string(),
            })
            .collect(),
        ParseMode::Html => escape_html(text),
    }
}

fn escape_code(code: &str) -> String {
    code.replace('\\', "\\\\").replace('`', "\\`")
}

fn escape_pre(code: &str) -> String {
    code.replace('\\', "\\\\").replace('`', "\\`")
}

fn escape_url(url: &str) -> String {
    url.replace(')', "\\)")
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
