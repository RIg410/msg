use crate::ast::*;
use crate::error::{Error, Result};
use crate::formatter::CustomFormatter;
use std::collections::HashMap;
use std::fmt::Write;

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

macro_rules! write_fmt {
    ($writer:expr, $($args:tt)*) => {
        write!($writer, $($args)*).map_err(|e| Error::Generation(e.to_string()))
    };
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

    pub fn generate<W: Write>(&self, writer: &mut W, element: &Element) -> Result<()> {
        self.generate_element(writer, element, self.mode)
    }

    fn generate_element<W: Write>(
        &self,
        writer: &mut W,
        element: &Element,
        mode: ParseMode,
    ) -> Result<()> {
        match element {
            Element::Text(text) => write_fmt!(writer, "{}", escape_text(text, mode)),

            Element::Bold(elements) => {
                match mode {
                    ParseMode::MarkdownV2 => write_fmt!(writer, "*")?,
                    ParseMode::Html => write_fmt!(writer, "<b>")?,
                }
                self.generate_elements(writer, elements, mode)?;
                match mode {
                    ParseMode::MarkdownV2 => write_fmt!(writer, "*")?,
                    ParseMode::Html => write_fmt!(writer, "</b>")?,
                }
                Ok(())
            }

            Element::Italic(elements) => {
                match mode {
                    ParseMode::MarkdownV2 => write_fmt!(writer, "_")?,
                    ParseMode::Html => write_fmt!(writer, "<i>")?,
                }
                self.generate_elements(writer, elements, mode)?;
                match mode {
                    ParseMode::MarkdownV2 => write_fmt!(writer, "_")?,
                    ParseMode::Html => write_fmt!(writer, "</i>")?,
                }
                Ok(())
            }

            Element::Code(code) => match mode {
                ParseMode::MarkdownV2 => write_fmt!(writer, "`{}`", escape_code(code)),
                ParseMode::Html => write_fmt!(writer, "<code>{}</code>", escape_html(code)),
            },

            Element::Pre(block) => match mode {
                ParseMode::MarkdownV2 => {
                    if let Some(lang) = &block.language {
                        write_fmt!(
                            writer,
                            "```{}\n{}\n```",
                            lang,
                            escape_pre(block.code.as_str())
                        )
                    } else {
                        write_fmt!(writer, "```\n{}\n```", escape_pre(block.code.as_str()))
                    }
                }
                ParseMode::Html => {
                    if let Some(lang) = &block.language {
                        write_fmt!(
                            writer,
                            "<pre><code class=\"language-{}\">{}</code></pre>",
                            escape_html(lang),
                            escape_html(&block.code)
                        )
                    } else {
                        write_fmt!(writer, "<pre>{}</pre>", escape_html(&block.code))
                    }
                }
            },

            Element::Underline(elements) => {
                match mode {
                    ParseMode::MarkdownV2 => write_fmt!(writer, "__")?,
                    ParseMode::Html => write_fmt!(writer, "<u>")?,
                }
                self.generate_elements(writer, elements, mode)?;
                match mode {
                    ParseMode::MarkdownV2 => write_fmt!(writer, "__")?,
                    ParseMode::Html => write_fmt!(writer, "</u>")?,
                }
                Ok(())
            }

            Element::Strikethrough(elements) => {
                match mode {
                    ParseMode::MarkdownV2 => write_fmt!(writer, "~~")?,
                    ParseMode::Html => write_fmt!(writer, "<s>")?,
                }
                self.generate_elements(writer, elements, mode)?;
                match mode {
                    ParseMode::MarkdownV2 => write_fmt!(writer, "~~")?,
                    ParseMode::Html => write_fmt!(writer, "</s>")?,
                }
                Ok(())
            }

            Element::Spoiler(elements) => {
                match mode {
                    ParseMode::MarkdownV2 => write_fmt!(writer, "||")?,
                    ParseMode::Html => write_fmt!(writer, "<tg-spoiler>")?,
                }
                self.generate_elements(writer, elements, mode)?;
                match mode {
                    ParseMode::MarkdownV2 => write_fmt!(writer, "||")?,
                    ParseMode::Html => write_fmt!(writer, "</tg-spoiler>")?,
                }
                Ok(())
            }

            Element::Link { text, url } => match mode {
                ParseMode::MarkdownV2 => {
                    write_fmt!(writer, "[")?;
                    self.generate_elements(writer, text, mode)?;
                    write_fmt!(writer, "]({})", escape_url(url))
                }
                ParseMode::Html => {
                    write_fmt!(writer, "<a href=\"{}\">", escape_html(url))?;
                    self.generate_elements(writer, text, mode)?;
                    write_fmt!(writer, "</a>")
                }
            },

            Element::TextLink { text, url } => match mode {
                ParseMode::MarkdownV2 => {
                    write_fmt!(writer, "[{}]({})", escape_text(text, mode), escape_url(url))
                }
                ParseMode::Html => write_fmt!(
                    writer,
                    "<a href=\"{}\">{}</a>",
                    escape_html(url),
                    escape_html(text)
                ),
            },

            Element::Mention { username } => write_fmt!(writer, "@{}", username),

            Element::MentionId { user_id, text } => match mode {
                ParseMode::MarkdownV2 => write_fmt!(
                    writer,
                    "[{}](tg://user?id={})",
                    escape_text(text, mode),
                    user_id
                ),
                ParseMode::Html => write_fmt!(
                    writer,
                    "<a href=\"tg://user?id={}\">{}</a>",
                    user_id,
                    escape_html(text)
                ),
            },

            Element::Hashtag(tag) => write_fmt!(writer, "#{}", tag),

            Element::Command { name, args } => {
                if args.is_empty() {
                    write_fmt!(writer, "/{}", name)
                } else {
                    write_fmt!(writer, "/{} {}", name, args.join(" "))
                }
            }

            Element::Emoji(emoji) => write_fmt!(writer, "{}", emoji),

            Element::CustomEmoji { emoji, id } => match mode {
                ParseMode::MarkdownV2 => write_fmt!(writer, "![{}](tg://emoji?id={})", emoji, id),
                ParseMode::Html => {
                    write_fmt!(writer, "<tg-emoji emoji-id=\"{}\">{}</tg-emoji>", id, emoji)
                }
            },

            Element::List(list) => self.generate_list(writer, list, mode),

            Element::Table(table) => self.generate_table(writer, table, mode),

            Element::Quote(elements) => match mode {
                ParseMode::MarkdownV2 => {
                    let mut temp = String::new();
                    self.generate_elements(&mut temp, elements, mode)?;
                    let quoted = temp.lines().collect::<Vec<_>>().join("\n>");
                    write_fmt!(writer, ">{}", quoted)
                }
                ParseMode::Html => {
                    write_fmt!(writer, "<blockquote>")?;
                    self.generate_elements(writer, elements, mode)?;
                    write_fmt!(writer, "</blockquote>")
                }
            },

            Element::Custom { formatter, value } => {
                if let Some(fmt) = self.formatters.get(formatter) {
                    let result = fmt.format(value, mode)?;
                    write_fmt!(writer, "{}", result)
                } else {
                    Err(Error::FormatterNotFound(formatter.clone()))
                }
            }

            Element::Group(elements) => self.generate_elements(writer, elements, mode),
        }
    }

    fn generate_elements<W: Write>(
        &self,
        writer: &mut W,
        elements: &[Element],
        mode: ParseMode,
    ) -> Result<()> {
        for element in elements {
            self.generate_element(writer, element, mode)?;
        }
        Ok(())
    }

    fn generate_list<W: Write>(
        &self,
        writer: &mut W,
        list: &ListNode,
        mode: ParseMode,
    ) -> Result<()> {
        for (i, item) in list.items.iter().enumerate() {
            let prefix = match &list.style {
                ListStyle::Bullet => "• ".to_string(),
                ListStyle::Numbered => format!("{}. ", i + 1),
                ListStyle::Custom(marker) => format!("{} ", marker),
            };

            write_fmt!(writer, "{}", prefix)?;
            self.generate_elements(writer, &item.content, mode)?;

            if let Some(nested) = &item.nested {
                write_fmt!(writer, "\n")?;
                let mut nested_content = String::new();
                self.generate_list(&mut nested_content, nested, mode)?;
                for line in nested_content.lines() {
                    write_fmt!(writer, "  {}\n", line)?;
                }
                // Remove last newline
                // Note: This logic might need adjustment based on actual requirements
            }

            if i < list.items.len() - 1 {
                write_fmt!(writer, "\n")?;
            }
        }

        Ok(())
    }

    fn generate_table<W: Write>(
        &self,
        writer: &mut W,
        table: &TableNode,
        mode: ParseMode,
    ) -> Result<()> {
        let all_rows: Vec<&[TableCell]> = std::iter::once(table.headers.as_slice())
            .chain(table.rows.iter().map(|r| r.cells.as_slice()))
            .collect();

        let col_widths = calculate_column_widths(&all_rows, mode)?;

        match table.style {
            TableStyle::Unicode => self.generate_unicode_table(writer, table, &col_widths, mode),
            TableStyle::Ascii => self.generate_ascii_table(writer, table, &col_widths, mode),
            TableStyle::Minimal => self.generate_minimal_table(writer, table, &col_widths, mode),
            TableStyle::Compact => self.generate_compact_table(writer, table, &col_widths, mode),
        }
    }

    fn generate_unicode_table<W: Write>(
        &self,
        writer: &mut W,
        table: &TableNode,
        col_widths: &[usize],
        mode: ParseMode,
    ) -> Result<()> {
        write_fmt!(
            writer,
            "```\n┌{}┐\n",
            col_widths
                .iter()
                .map(|&w| "─".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("┬")
        )?;

        self.format_table_row(writer, &table.headers, col_widths, mode, "│")?;
        write_fmt!(
            writer,
            "\n├{}┤\n",
            col_widths
                .iter()
                .map(|&w| "─".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("┼")
        )?;

        for row in &table.rows {
            self.format_table_row(writer, &row.cells, col_widths, mode, "│")?;
            write_fmt!(writer, "\n")?;
        }

        write_fmt!(
            writer,
            "└{}┘\n```",
            col_widths
                .iter()
                .map(|&w| "─".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("┴")
        )?;

        Ok(())
    }

    fn generate_ascii_table<W: Write>(
        &self,
        writer: &mut W,
        table: &TableNode,
        col_widths: &[usize],
        mode: ParseMode,
    ) -> Result<()> {
        write_fmt!(
            writer,
            "```\n+{}+\n",
            col_widths
                .iter()
                .map(|&w| "-".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("+")
        )?;

        self.format_table_row(writer, &table.headers, col_widths, mode, "|")?;
        write_fmt!(
            writer,
            "\n+{}+\n",
            col_widths
                .iter()
                .map(|&w| "-".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("+")
        )?;

        for row in &table.rows {
            self.format_table_row(writer, &row.cells, col_widths, mode, "|")?;
            write_fmt!(writer, "\n")?;
        }

        write_fmt!(
            writer,
            "+{}+\n```",
            col_widths
                .iter()
                .map(|&w| "-".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("+")
        )?;

        Ok(())
    }

    fn generate_minimal_table<W: Write>(
        &self,
        writer: &mut W,
        table: &TableNode,
        col_widths: &[usize],
        mode: ParseMode,
    ) -> Result<()> {
        write_fmt!(writer, "```\n")?;

        self.format_table_row(writer, &table.headers, col_widths, mode, " ")?;
        write_fmt!(
            writer,
            "\n{}\n",
            col_widths
                .iter()
                .map(|&w| "─".repeat(w))
                .collect::<Vec<_>>()
                .join(" ")
        )?;

        for row in &table.rows {
            self.format_table_row(writer, &row.cells, col_widths, mode, " ")?;
            write_fmt!(writer, "\n")?;
        }

        write_fmt!(writer, "```")?;
        Ok(())
    }

    fn generate_compact_table<W: Write>(
        &self,
        writer: &mut W,
        table: &TableNode,
        col_widths: &[usize],
        mode: ParseMode,
    ) -> Result<()> {
        write_fmt!(writer, "```\n")?;

        self.format_table_row(writer, &table.headers, col_widths, mode, " ")?;
        write_fmt!(writer, "\n")?;

        for row in &table.rows {
            self.format_table_row(writer, &row.cells, col_widths, mode, " ")?;
            write_fmt!(writer, "\n")?;
        }

        write_fmt!(writer, "```")?;
        Ok(())
    }

    fn format_table_row<W: Write>(
        &self,
        writer: &mut W,
        cells: &[TableCell],
        col_widths: &[usize],
        _mode: ParseMode,
        separator: &str,
    ) -> Result<()> {
        write_fmt!(writer, "{}", separator)?;

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

                write_fmt!(writer, "{}", padded)?;
                write_fmt!(writer, "{}", separator)?;
            }
        }

        Ok(())
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
