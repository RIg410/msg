#[cfg(test)]
mod token_tests {
    use crate::token::{Lexer, Token};

    #[test]
    fn test_tokenize_simple_text() {
        let mut lexer = Lexer::new("Hello, world!");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::Text("Hello, world!".to_string()), Token::Eof,]
        );
    }

    #[test]
    fn test_tokenize_bold() {
        let mut lexer = Lexer::new("*bold text*");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Star,
                Token::Text("bold text".to_string()),
                Token::Star,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_mention() {
        let mut lexer = Lexer::new("Hello @username!");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Text("Hello ".to_string()),
                Token::Mention("username".to_string()),
                Token::Text("!".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_hashtag() {
        let mut lexer = Lexer::new("#rust #programming");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Hashtag("rust".to_string()),
                Token::Text(" ".to_string()),
                Token::Hashtag("programming".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_escaped() {
        let mut lexer = Lexer::new("\\*not bold\\*");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Escape('*'),
                Token::Text("not bold".to_string()),
                Token::Escape('*'),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_tokenize_mixed() {
        let mut lexer = Lexer::new("*bold* _italic_ `code`");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Star,
                Token::Text("bold".to_string()),
                Token::Star,
                Token::Text(" ".to_string()),
                Token::Underscore,
                Token::Text("italic".to_string()),
                Token::Underscore,
                Token::Text(" ".to_string()),
                Token::Backtick,
                Token::Text("code".to_string()),
                Token::Backtick,
                Token::Eof,
            ]
        );
    }
}

#[cfg(test)]
mod ast_tests {
    use crate::ast::*;

    #[test]
    fn test_create_text_element() {
        let element = Element::text("Hello, world!");
        assert_eq!(element, Element::Text("Hello, world!".to_string()));
    }

    #[test]
    fn test_create_bold_element() {
        let element = Element::bold(vec![Element::text("Bold text")]);
        assert_eq!(
            element,
            Element::Bold(vec![Element::Text("Bold text".to_string())])
        );
    }

    #[test]
    fn test_create_link_element() {
        let element = Element::link(vec![Element::text("Click here")], "https://example.com");
        assert_eq!(
            element,
            Element::Link {
                text: vec![Element::Text("Click here".to_string())],
                url: "https://example.com".to_string(),
            }
        );
    }

    #[test]
    fn test_create_list() {
        let list = ListNode {
            style: ListStyle::Bullet,
            items: vec![
                ListItem {
                    content: vec![Element::text("Item 1")],
                    nested: None,
                },
                ListItem {
                    content: vec![Element::text("Item 2")],
                    nested: None,
                },
            ],
        };
        let element = Element::List(list.clone());
        assert_eq!(element, Element::List(list));
    }

    #[test]
    fn test_table_cell_default() {
        let cell = TableCell::default();
        assert_eq!(cell.align, CellAlign::Left);
        assert_eq!(cell.colspan, 1);
        assert_eq!(cell.rowspan, 1);
        assert!(cell.content.is_empty());
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::ast::*;
    use crate::parser::parse;

    #[test]
    fn test_parse_simple_text() {
        let result = parse("Hello, world!").unwrap();
        assert_eq!(result, vec![Element::Text("Hello, world!".to_string())]);
    }

    #[test]
    fn test_parse_bold() {
        let result = parse("**bold text**").unwrap();
        assert_eq!(
            result,
            vec![Element::Bold(vec![Element::Text("bold text".to_string())])]
        );
    }

    #[test]
    fn test_parse_italic() {
        let result = parse("*italic text*").unwrap();
        assert_eq!(
            result,
            vec![Element::Italic(vec![Element::Text(
                "italic text".to_string()
            )])]
        );
    }

    #[test]
    fn test_parse_code() {
        let result = parse("`code block`").unwrap();
        assert_eq!(result, vec![Element::Code("code block".to_string())]);
    }

    #[test]
    fn test_parse_mention() {
        let result = parse("Hello @username!").unwrap();
        assert_eq!(
            result,
            vec![
                Element::Text("Hello ".to_string()),
                Element::Mention {
                    username: "username".to_string()
                },
                Element::Text("!".to_string()),
            ]
        );
    }

    #[test]
    fn test_parse_link() {
        let result = parse("[Google](https://google.com)").unwrap();
        assert_eq!(
            result,
            vec![Element::Link {
                text: vec![Element::Text("Google".to_string())],
                url: "https://google.com".to_string(),
            }]
        );
    }

    #[test]
    fn test_parse_nested() {
        let result = parse("**bold *and italic* text**").unwrap();
        assert_eq!(
            result,
            vec![Element::Bold(vec![
                Element::Text("bold ".to_string()),
                Element::Italic(vec![Element::Text("and italic".to_string())]),
                Element::Text(" text".to_string()),
            ])]
        );
    }

    #[test]
    fn test_parse_escaped() {
        let result = parse("\\*not bold\\*").unwrap();
        assert_eq!(
            result,
            vec![
                Element::Text("*".to_string()),
                Element::Text("not bold".to_string()),
                Element::Text("*".to_string()),
            ]
        );
    }
}

#[cfg(test)]
mod generator_tests {
    use crate::ast::*;
    use crate::generator::{Generator, ParseMode};

    #[test]
    fn test_generate_text_markdown() {
        let generator = Generator::new(ParseMode::MarkdownV2);
        let element = Element::Text("Hello, world!".to_string());
        let mut result = String::new();
        generator.generate(&mut result, &element).unwrap();
        assert_eq!(result, "Hello, world\\!");
    }

    #[test]
    fn test_generate_text_html() {
        let generator = Generator::new(ParseMode::Html);
        let element = Element::Text("Hello <world>".to_string());
        let mut result = String::new();
        generator.generate(&mut result, &element).unwrap();
        assert_eq!(result, "Hello &lt;world&gt;");
    }

    #[test]
    fn test_generate_bold_markdown() {
        let generator = Generator::new(ParseMode::MarkdownV2);
        let element = Element::Bold(vec![Element::Text("bold text".to_string())]);
        let mut result = String::new();
        generator.generate(&mut result, &element).unwrap();
        assert_eq!(result, "*bold text*");
    }

    #[test]
    fn test_generate_bold_html() {
        let generator = Generator::new(ParseMode::Html);
        let element = Element::Bold(vec![Element::Text("bold text".to_string())]);
        let mut result = String::new();
        generator.generate(&mut result, &element).unwrap();
        assert_eq!(result, "<b>bold text</b>");
    }

    #[test]
    fn test_generate_link_markdown() {
        let generator = Generator::new(ParseMode::MarkdownV2);
        let element = Element::Link {
            text: vec![Element::Text("Google".to_string())],
            url: "https://google.com".to_string(),
        };
        let mut result = String::new();
        generator.generate(&mut result, &element).unwrap();
        assert_eq!(result, "[Google](https://google.com)");
    }

    #[test]
    fn test_generate_link_html() {
        let generator = Generator::new(ParseMode::Html);
        let element = Element::Link {
            text: vec![Element::Text("Google".to_string())],
            url: "https://google.com".to_string(),
        };
        let mut result = String::new();
        generator.generate(&mut result, &element).unwrap();
        assert_eq!(result, "<a href=\"https://google.com\">Google</a>");
    }

    #[test]
    fn test_generate_pre_markdown() {
        let generator = Generator::new(ParseMode::MarkdownV2);
        let element = Element::Pre(PreBlock {
            code: "fn main() {\n    println!(\"Hello\");\n}".to_string(),
            language: Some("rust".to_string()),
        });
        let mut result = String::new();
        generator.generate(&mut result, &element).unwrap();
        assert_eq!(
            result,
            "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```"
        );
    }

    #[test]
    fn test_generate_list() {
        let generator = Generator::new(ParseMode::MarkdownV2);
        let element = Element::List(ListNode {
            style: ListStyle::Bullet,
            items: vec![
                ListItem {
                    content: vec![Element::Text("Item 1".to_string())],
                    nested: None,
                },
                ListItem {
                    content: vec![Element::Text("Item 2".to_string())],
                    nested: None,
                },
            ],
        });
        let mut result = String::new();
        generator.generate(&mut result, &element).unwrap();
        assert_eq!(result, "• Item 1\n• Item 2");
    }

    #[test]
    fn test_roundtrip() {
        use crate::parser::parse;

        let original = "**bold** *italic* `code`";
        let parsed = parse(original).unwrap();
        let generator = Generator::new(ParseMode::MarkdownV2);

        let mut generated = String::new();
        for element in &parsed {
            generator.generate(&mut generated, element).unwrap();
        }

        assert!(generated.contains("*bold*"));
        assert!(generated.contains("_italic_"));
        assert!(generated.contains("`code`"));
    }
}

#[cfg(test)]
mod formatter_tests {
    use crate::formatter::{CustomFormatter, PhoneFormatter};
    use crate::generator::ParseMode;

    #[test]
    fn test_phone_formatter_name() {
        let formatter = PhoneFormatter;
        assert_eq!(formatter.name(), "phone");
    }

    #[test]
    fn test_phone_formatter_parse_international() {
        let formatter = PhoneFormatter;
        
        let test_cases = vec![
            ("+1234567890", "+1234567890", 11),
            ("+1 234 567 890", "+1 234 567 890", 14),
            ("+1-234-567-890", "+1-234-567-890", 14),
            ("+1(234)567-890", "+1(234)567-890", 14),
            ("+7 (495) 123-45-67", "+7 (495) 123-45-67", 18),
            ("+380 44 123 4567", "+380 44 123 4567", 16),
        ];

        for (input, expected, expected_len) in test_cases {
            let result = formatter.parse(input);
            assert!(result.is_some(), "Failed to parse: {}", input);
            let (parsed, len) = result.unwrap();
            assert_eq!(parsed, expected, "Failed for input: {}", input);
            assert_eq!(len, expected_len, "Wrong length for: {}", input);
        }
    }

    #[test]
    fn test_phone_formatter_parse_local() {
        let formatter = PhoneFormatter;
        
        let test_cases = vec![
            ("1234567890", "1234567890", 10),
            ("123-456-7890", "123-456-7890", 12),
            ("(123) 456-7890", "(123) 456-7890", 14),
            ("123 456 7890", "123 456 7890", 12),
            ("8 800 555 35 35", "8 800 555 35 35", 15),
        ];

        for (input, expected, expected_len) in test_cases {
            let result = formatter.parse(input);
            assert!(result.is_some(), "Failed to parse: {}", input);
            let (parsed, len) = result.unwrap();
            assert_eq!(parsed, expected, "Failed for input: {}", input);
            assert_eq!(len, expected_len, "Wrong length for: {}", input);
        }
    }

    #[test]
    fn test_phone_formatter_parse_with_text() {
        let formatter = PhoneFormatter;
        
        let input = "+1234567890 call me";
        let result = formatter.parse(input);
        assert!(result.is_some());
        let (parsed, len) = result.unwrap();
        assert_eq!(parsed.trim(), "+1234567890");
        assert_eq!(len, 12); // includes the trailing space
    }

    #[test]
    fn test_phone_formatter_parse_empty() {
        let formatter = PhoneFormatter;
        
        let input = "";
        let result = formatter.parse(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_phone_formatter_parse_non_phone() {
        let formatter = PhoneFormatter;
        
        let input = "abc def";
        let result = formatter.parse(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_phone_formatter_format_markdown() {
        let formatter = PhoneFormatter;
        
        let test_cases = vec![
            ("+1234567890", "`\\+1234567890`"),
            ("+1-234-567-890", "`\\+1\\-234\\-567\\-890`"),
            ("+1 (234) 567-890", "`\\+1 \\(234\\) 567\\-890`"),
            ("123-456-7890", "`123\\-456\\-7890`"),
            ("(123) 456-7890", "`\\(123\\) 456\\-7890`"),
        ];

        for (input, expected) in test_cases {
            let result = formatter.format(input, ParseMode::MarkdownV2).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_phone_formatter_format_html() {
        let formatter = PhoneFormatter;
        
        let test_cases = vec![
            ("+1234567890", "<code>+1234567890</code>"),
            ("+1-234-567-890", "<code>+1-234-567-890</code>"),
            ("+1 (234) 567-890", "<code>+1 (234) 567-890</code>"),
            ("123-456-7890", "<code>123-456-7890</code>"),
            ("(123) 456-7890", "<code>(123) 456-7890</code>"),
            ("<123>", "<code>&lt;123&gt;</code>"),
        ];

        for (input, expected) in test_cases {
            let result = formatter.format(input, ParseMode::Html).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_phone_formatter_format_special_chars_markdown() {
        let formatter = PhoneFormatter;
        
        let phone = "+1_234*567#890";
        let result = formatter.format(phone, ParseMode::MarkdownV2).unwrap();
        assert_eq!(result, "`\\+1\\_234\\*567\\#890`");
    }

    #[test]
    fn test_phone_formatter_format_special_chars_html() {
        let formatter = PhoneFormatter;
        
        let phone = "+1<234>567&890";
        let result = formatter.format(phone, ParseMode::Html).unwrap();
        assert_eq!(result, "<code>+1&lt;234&gt;567&amp;890</code>");
    }

    #[test]
    fn test_phone_formatter_format_empty() {
        let formatter = PhoneFormatter;
        
        let result_md = formatter.format("", ParseMode::MarkdownV2).unwrap();
        assert_eq!(result_md, "``");
        
        let result_html = formatter.format("", ParseMode::Html).unwrap();
        assert_eq!(result_html, "<code></code>");
    }

    #[test]
    fn test_phone_formatter_various_formats() {
        let formatter = PhoneFormatter;
        
        let phone_numbers = vec![
            "+44 20 7946 0958",      // UK
            "+33 1 42 86 82 00",      // France
            "+49 30 2594 1000",       // Germany
            "+81 3-1234-5678",        // Japan
            "+86 10 1234 5678",       // China
            "+61 2 9374 4000",        // Australia
            "+55 11 98765-4321",      // Brazil
            "+91 22 2278 2278",       // India
            "+7 495 123-45-67",       // Russia
            "+1-800-FLOWERS",         // US with letters
        ];

        for phone in phone_numbers {
            let result = formatter.parse(phone);
            assert!(result.is_some(), "Failed to parse: {}", phone);
            
            let (parsed, _) = result.unwrap();
            assert!(!parsed.is_empty(), "Empty result for: {}", phone);
            
            let formatted_md = formatter.format(&parsed, ParseMode::MarkdownV2).unwrap();
            assert!(formatted_md.starts_with("`"), "Markdown format should start with backtick");
            assert!(formatted_md.ends_with("`"), "Markdown format should end with backtick");
            
            let formatted_html = formatter.format(&parsed, ParseMode::Html).unwrap();
            assert!(formatted_html.starts_with("<code>"), "HTML format should start with <code>");
            assert!(formatted_html.ends_with("</code>"), "HTML format should end with </code>");
        }
    }
}
