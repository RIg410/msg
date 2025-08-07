use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token, Expr, Ident, Lit, Result, Token,
};

#[derive(Debug)]
enum TgMessageItem {
    Text(Lit),
    Bold(Vec<TgMessageItem>),
    Italic(Vec<TgMessageItem>),
    Underline(Vec<TgMessageItem>),
    Strikethrough(Vec<TgMessageItem>),
    Spoiler(Vec<TgMessageItem>),
    Code(Lit),
    Pre {
        code: Lit,
        lang: Option<Lit>,
    },
    Link {
        text: Vec<TgMessageItem>,
        url: Expr,
    },
    Mention(Expr),
    MentionAt(Ident),
    Hashtag(Expr),
    HashtagHash(Ident),
    MessageReference(Expr), // Added for #msg syntax
    List {
        style: ListStyle,
        items: Vec<Vec<TgMessageItem>>,
    },
    Table {
        headers: Vec<Expr>,
        rows: Vec<Vec<Expr>>,
    },
    Phone {
        prefix: Option<String>,
        number: Expr,
    },
    Date(Expr),
    DateTime(Expr),
    Time(Expr),
    Expression(Expr),
}

#[derive(Debug)]
enum ListStyle {
    Bullet,
    Numbered,
    Custom(Ident),
}

impl Parse for TgMessageItem {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident) {
            // First, check if this is a known keyword by peeking ahead
            let fork = input.fork();
            let ident: Ident = fork.parse()?;
            let name = ident.to_string();

            // Check if this is a known keyword that might have parentheses
            let has_parens = fork.peek(token::Paren);

            // Check if next token is something that makes this an expression (like '.')
            let is_expression = fork.peek(Token![.]) || fork.peek(Token![::]);

            if !is_expression
                && (matches!(
                    name.as_str(),
                    "bold"
                        | "italic"
                        | "underline"
                        | "strikethrough"
                        | "spoiler"
                        | "code"
                        | "pre"
                        | "link"
                        | "mention"
                        | "hashtag"
                        | "list"
                        | "table"
                        | "date"
                        | "datetime"
                        | "time"
                ) || has_parens)
            {
                match name.as_str() {
                    "bold" => {
                        // Now consume the actual identifier from input
                        let _: Ident = input.parse()?;
                        let content;
                        syn::braced!(content in input);
                        let items = parse_message_items(&content)?;
                        Ok(TgMessageItem::Bold(items))
                    }
                    "italic" => {
                        let _: Ident = input.parse()?;
                        let content;
                        syn::braced!(content in input);
                        let items = parse_message_items(&content)?;
                        Ok(TgMessageItem::Italic(items))
                    }
                    "underline" => {
                        let _: Ident = input.parse()?;
                        let content;
                        syn::braced!(content in input);
                        let items = parse_message_items(&content)?;
                        Ok(TgMessageItem::Underline(items))
                    }
                    "strikethrough" => {
                        let _: Ident = input.parse()?;
                        let content;
                        syn::braced!(content in input);
                        let items = parse_message_items(&content)?;
                        Ok(TgMessageItem::Strikethrough(items))
                    }
                    "spoiler" => {
                        let _: Ident = input.parse()?;
                        let content;
                        syn::braced!(content in input);
                        let items = parse_message_items(&content)?;
                        Ok(TgMessageItem::Spoiler(items))
                    }
                    "code" => {
                        let _: Ident = input.parse()?;
                        let content;
                        syn::braced!(content in input);
                        let text: Lit = content.parse()?;
                        Ok(TgMessageItem::Code(text))
                    }
                    "pre" => {
                        let _: Ident = input.parse()?;
                        let lang = if input.peek(token::Paren) {
                            let content;
                            syn::parenthesized!(content in input);
                            Some(content.parse::<Lit>()?)
                        } else {
                            None
                        };
                        let content;
                        syn::braced!(content in input);
                        let code: Lit = content.parse()?;
                        Ok(TgMessageItem::Pre { code, lang })
                    }
                    "link" => {
                        let _: Ident = input.parse()?;
                        let content;
                        syn::parenthesized!(content in input);
                        let url: Expr = content.parse()?;
                        let content;
                        syn::braced!(content in input);
                        let text = parse_message_items(&content)?;
                        Ok(TgMessageItem::Link { text, url })
                    }
                    "mention" => {
                        let _: Ident = input.parse()?;
                        let content;
                        syn::braced!(content in input);
                        let username: Expr = content.parse()?;
                        Ok(TgMessageItem::Mention(username))
                    }
                    "hashtag" => {
                        let _: Ident = input.parse()?;
                        let content;
                        syn::braced!(content in input);
                        let tag: Expr = content.parse()?;
                        Ok(TgMessageItem::Hashtag(tag))
                    }
                    "list" => {
                        let _: Ident = input.parse()?;
                        let style = if input.peek(token::Paren) {
                            let content;
                            syn::parenthesized!(content in input);
                            let style_ident: Ident = content.parse()?;
                            match style_ident.to_string().as_str() {
                                "bullet" => ListStyle::Bullet,
                                "numbered" => ListStyle::Numbered,
                                _ => ListStyle::Custom(style_ident),
                            }
                        } else {
                            ListStyle::Bullet
                        };

                        let content;
                        syn::braced!(content in input);
                        let items = parse_list_items(&content)?;
                        Ok(TgMessageItem::List { style, items })
                    }
                    "table" => {
                        let _: Ident = input.parse()?;
                        let content;
                        syn::braced!(content in input);

                        let _: Ident = content.parse()?;
                        let _: Token![:] = content.parse()?;
                        let headers_content;
                        syn::bracketed!(headers_content in content);
                        let headers =
                            Punctuated::<Expr, Token![,]>::parse_terminated(&headers_content)?
                                .into_iter()
                                .collect();

                        let _: Ident = content.parse()?;
                        let _: Token![:] = content.parse()?;
                        let rows_content;
                        syn::bracketed!(rows_content in content);
                        let mut rows = Vec::new();
                        while !rows_content.is_empty() {
                            let row_content;
                            syn::bracketed!(row_content in rows_content);
                            let row =
                                Punctuated::<Expr, Token![,]>::parse_terminated(&row_content)?
                                    .into_iter()
                                    .collect();
                            rows.push(row);
                        }

                        Ok(TgMessageItem::Table { headers, rows })
                    }
                    "date" => {
                        let _: Ident = input.parse()?;
                        let content;
                        syn::parenthesized!(content in input);
                        let value: Expr = content.parse()?;
                        Ok(TgMessageItem::Date(value))
                    }
                    "datetime" => {
                        let _: Ident = input.parse()?;
                        let content;
                        syn::parenthesized!(content in input);
                        let value: Expr = content.parse()?;
                        Ok(TgMessageItem::DateTime(value))
                    }
                    "time" => {
                        let _: Ident = input.parse()?;
                        let content;
                        syn::parenthesized!(content in input);
                        let value: Expr = content.parse()?;
                        Ok(TgMessageItem::Time(value))
                    }
                    _ => {
                        // This is not a known keyword, parse as expression
                        let expr: Expr = input.parse()?;
                        Ok(TgMessageItem::Expression(expr))
                    }
                }
            } else {
                // This is an expression (has . or :: after identifier)
                let expr: Expr = input.parse()?;
                Ok(TgMessageItem::Expression(expr))
            }
        } else if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            let username: Ident = input.parse()?;
            Ok(TgMessageItem::MentionAt(username))
        } else if input.peek(Token![#]) {
            input.parse::<Token![#]>()?;
            // Check if this is a message reference (expression) or a hashtag (identifier)
            if input.peek(Ident) {
                // Check if the identifier might be a variable (message reference)
                let fork = input.fork();
                let _ident: Ident = fork.parse()?;
                // If next token suggests this is an expression (e.g., field access), parse as expression
                if fork.peek(Token![.]) || fork.peek(Token![::]) || fork.peek(token::Paren) {
                    let expr: Expr = input.parse()?;
                    Ok(TgMessageItem::MessageReference(expr))
                } else {
                    // Simple identifier - could be either hashtag or message reference
                    // We'll treat simple identifiers as message references if they don't start with uppercase
                    let ident: Ident = input.parse()?;
                    let name = ident.to_string();
                    if name.chars().next().map_or(false, |c| c.is_uppercase()) {
                        // Likely a hashtag
                        Ok(TgMessageItem::HashtagHash(ident))
                    } else {
                        // Likely a variable reference
                        Ok(TgMessageItem::MessageReference(syn::parse_quote!(#ident)))
                    }
                }
            } else {
                // Not an identifier after #, parse as expression
                let expr: Expr = input.parse()?;
                Ok(TgMessageItem::MessageReference(expr))
            }
        } else if input.peek(Token![+]) {
            // Parse phone number format: +7(phone), +8(phone), +(phone)
            input.parse::<Token![+]>()?;
            let prefix = if input.peek(syn::LitInt) {
                let lit: syn::LitInt = input.parse()?;
                Some(format!("+{}", lit.base10_parse::<u32>()?))
            } else {
                None
            };
            let content;
            syn::parenthesized!(content in input);
            let number: Expr = content.parse()?;
            Ok(TgMessageItem::Phone { prefix, number })
        } else if input.peek(Lit) {
            let lit: Lit = input.parse()?;
            Ok(TgMessageItem::Text(lit))
        } else {
            let expr: Expr = input.parse()?;
            Ok(TgMessageItem::Expression(expr))
        }
    }
}

fn parse_message_items(input: ParseStream) -> Result<Vec<TgMessageItem>> {
    let mut items = Vec::new();
    while !input.is_empty() {
        items.push(input.parse()?);
    }
    Ok(items)
}

fn parse_list_items(input: ParseStream) -> Result<Vec<Vec<TgMessageItem>>> {
    let mut items = Vec::new();
    while !input.is_empty() {
        input.parse::<Token![-]>()?;
        let mut item_content = Vec::new();
        while !input.peek(Token![;]) && !input.is_empty() && !input.peek(Token![-]) {
            item_content.push(input.parse()?);
        }
        items.push(item_content);
        if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
        }
    }
    Ok(items)
}

impl ToTokens for TgMessageItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let result = match self {
            TgMessageItem::Text(lit) => {
                let text = match lit {
                    Lit::Str(s) => {
                        let _value = s.value();
                        quote! {
                            {
                                let text = #s;
                                // URL regex pattern
                                let url_regex = ::regex::Regex::new(r"https?://[^\s]+").unwrap();

                                if url_regex.is_match(&text) {
                                    let mut elements = Vec::new();
                                    let mut last_end = 0;

                                    for mat in url_regex.find_iter(&text) {
                                        // Add text before URL if any
                                        if mat.start() > last_end {
                                            let before = &text[last_end..mat.start()];
                                            if !before.is_empty() {
                                                elements.push(::msg::Element::text(before));
                                            }
                                        }

                                        // Add URL as link
                                        let url = mat.as_str();
                                        elements.push(::msg::Element::TextLink {
                                            text: url.to_string(),
                                            url: url.to_string(),
                                        });

                                        last_end = mat.end();
                                    }

                                    // Add remaining text after last URL
                                    if last_end < text.len() {
                                        let after = &text[last_end..];
                                        if !after.is_empty() {
                                            elements.push(::msg::Element::text(after));
                                        }
                                    }

                                    if elements.len() == 1 {
                                        elements.into_iter().next().unwrap()
                                    } else {
                                        ::msg::Element::Group(elements)
                                    }
                                } else if text.contains('\n') {
                                    let parts: Vec<&str> = text.split('\n').collect();
                                    let mut elements = Vec::new();
                                    for (i, part) in parts.iter().enumerate() {
                                        if !part.is_empty() {
                                            elements.push(::msg::Element::text(*part));
                                        }
                                        if i < parts.len() - 1 {
                                            elements.push(::msg::Element::text("\n"));
                                        }
                                    }
                                    ::msg::Element::Group(elements)
                                } else {
                                    ::msg::Element::text(text)
                                }
                            }
                        }
                    }
                    _ => quote! { ::msg::Element::text(#lit.to_string()) },
                };
                text
            }
            TgMessageItem::Bold(items) => {
                let elements = generate_elements(items);
                quote! { ::msg::Element::bold(vec![#(#elements),*]) }
            }
            TgMessageItem::Italic(items) => {
                let elements = generate_elements(items);
                quote! { ::msg::Element::italic(vec![#(#elements),*]) }
            }
            TgMessageItem::Underline(items) => {
                let elements = generate_elements(items);
                quote! { ::msg::Element::underline(vec![#(#elements),*]) }
            }
            TgMessageItem::Strikethrough(items) => {
                let elements = generate_elements(items);
                quote! { ::msg_macro::Element::strikethrough(vec![#(#elements),*]) }
            }
            TgMessageItem::Spoiler(items) => {
                let elements = generate_elements(items);
                quote! { ::msg::Element::spoiler(vec![#(#elements),*]) }
            }
            TgMessageItem::Code(lit) => {
                quote! { ::msg::Element::code(#lit) }
            }
            TgMessageItem::Pre { code, lang } => {
                if let Some(lang) = lang {
                    quote! { ::msg::Element::pre(#code, Some(#lang.to_string())) }
                } else {
                    quote! { ::msg::Element::pre(#code, None) }
                }
            }
            TgMessageItem::Link { text, url } => {
                let elements = generate_elements(text);
                quote! { ::msg::Element::link(vec![#(#elements),*], #url) }
            }
            TgMessageItem::Mention(username) => {
                quote! { ::msg::Element::mention(#username) }
            }
            TgMessageItem::MentionAt(username) => {
                let username_str = username.to_string();
                quote! { ::msg::Element::mention(#username_str) }
            }
            TgMessageItem::Hashtag(tag) => {
                quote! { ::msg::Element::hashtag(#tag) }
            }
            TgMessageItem::HashtagHash(tag) => {
                let tag_str = tag.to_string();
                quote! { ::msg::Element::hashtag(#tag_str) }
            }
            TgMessageItem::List { style, items } => {
                let style_expr = match style {
                    ListStyle::Bullet => quote! { ::msg::ListStyle::Bullet },
                    ListStyle::Numbered => quote! { ::msg::ListStyle::Numbered },
                    ListStyle::Custom(ident) => {
                        quote! { ::msg::ListStyle::Custom(#ident.to_string()) }
                    }
                };

                let list_items = items.iter().map(|item| {
                    let elements = generate_elements(item);
                    quote! {
                        ::msg::ListItem {
                            content: vec![#(#elements),*],
                            nested: None,
                        }
                    }
                });

                quote! {
                    ::msg::Element::List(::msg::ListNode {
                        style: #style_expr,
                        items: vec![#(#list_items),*],
                    })
                }
            }
            TgMessageItem::Table { headers, rows } => {
                let header_cells = headers.iter().map(|h| {
                    quote! {
                        ::msg::TableCell {
                            content: vec![::msg::Element::text(#h.to_string())],
                            align: ::msg::CellAlign::Left,
                            colspan: 1,
                            rowspan: 1,
                        }
                    }
                });

                let table_rows = rows.iter().map(|row| {
                    let cells = row.iter().map(|cell| {
                        quote! {
                            ::msg::TableCell {
                                content: vec![::msg::Element::text(#cell.to_string())],
                                align: ::msg::CellAlign::Left,
                                colspan: 1,
                                rowspan: 1,
                            }
                        }
                    });
                    quote! {
                        ::msg::TableRow {
                            cells: vec![#(#cells),*],
                        }
                    }
                });

                quote! {
                    ::msg::Element::Table(::msg::TableNode {
                        headers: vec![#(#header_cells),*],
                        rows: vec![#(#table_rows),*],
                        style: ::msg::TableStyle::Unicode,
                        rules: Vec::new(),
                    })
                }
            }
            TgMessageItem::Phone { prefix, number } => {
                let prefix_expr = match prefix {
                    Some(p) => quote! { Some(#p.to_string()) },
                    None => quote! { None::<String> },
                };
                quote! {
                    {
                        let phone_str = #number.to_string();
                        
                        // Handle empty string
                        if phone_str.is_empty() {
                            ::msg::Element::Text("-".to_string())
                        } else {
                            // Remove non-digit characters
                            let digits: String = phone_str.chars().filter(|c| c.is_digit(10)).collect();
                            
                            // Return "-" if no digits
                            if digits.is_empty() {
                                ::msg::Element::Text("-".to_string())
                            } else {
                                // Determine the actual prefix and format accordingly
                                let (final_prefix, phone_digits, tel_prefix) = match #prefix_expr {
                                    Some(prefix) => {
                                        // If prefix is provided explicitly (e.g., +7), use it
                                        (prefix.clone(), digits.clone(), format!("{}{}", prefix.replace("+", ""), digits))
                                    }
                                    None => {
                                        // If +(phone) format, check if number starts with 7 or 8
                                        if digits.len() == 11 && digits.starts_with("7") {
                                            // Russian number format with 7: extract country code
                                            ("+7".to_string(), digits[1..].to_string(), format!("7{}", &digits[1..]))
                                        } else if digits.len() == 11 && digits.starts_with("8") {
                                            // Russian number format with 8: convert to +7
                                            ("+7".to_string(), digits[1..].to_string(), format!("7{}", &digits[1..]))
                                        } else if digits.len() == 10 {
                                            // Assume it's a local number without country code, default to +7
                                            ("+7".to_string(), digits.clone(), format!("7{}", digits))
                                        } else {
                                            // Other format, use as is with +
                                            ("+".to_string(), digits.clone(), digits.clone())
                                        }
                                    }
                                };

                                // Format the phone number if we have enough digits
                                let formatted = if phone_digits.len() == 10 {
                                    // Format as (XXX) XXX-XX-XX for 10-digit numbers
                                    let area = &phone_digits[0..3];
                                    let prefix_part = &phone_digits[3..6];
                                    let part1 = &phone_digits[6..8];
                                    let part2 = &phone_digits[8..10];
                                    // Check if prefix was explicitly provided (e.g., +7(phone))
                                    let space_after_prefix = if #prefix_expr.is_some() { " " } else { "" };
                                    format!("{}{}({}) {}-{}-{}", final_prefix, space_after_prefix, area, prefix_part, part1, part2)
                                } else if phone_digits.len() >= 7 {
                                    // Format with dashes for other lengths >= 7
                                    let area_len = 3.min(phone_digits.len());
                                    let area = &phone_digits[0..area_len];
                                    let rest = &phone_digits[area_len..];
                                    
                                    // Split rest into chunks with dashes
                                    let mut formatted_rest = String::new();
                                    let mut chars = rest.chars();
                                    
                                    // First chunk of 3 digits if available
                                    if rest.len() >= 3 {
                                        for _ in 0..3 {
                                            if let Some(c) = chars.next() {
                                                formatted_rest.push(c);
                                            }
                                        }
                                        // Add remaining digits with dashes every 2 digits
                                        let remaining: String = chars.collect();
                                        if !remaining.is_empty() {
                                            formatted_rest.push('-');
                                            for (i, c) in remaining.chars().enumerate() {
                                                if i > 0 && i % 2 == 0 {
                                                    formatted_rest.push('-');
                                                }
                                                formatted_rest.push(c);
                                            }
                                        }
                                    } else {
                                        formatted_rest = rest.to_string();
                                    }
                                    
                                    format!("{}({}) {}", final_prefix, area, formatted_rest)
                                } else {
                                    // Short number, return without formatting
                                    format!("{}{}", final_prefix, phone_digits)
                                };

                                // Create tel: URL with proper prefix
                                let tel_url = format!("tel:+{}", tel_prefix);
                                
                                ::msg::Element::TextLink {
                                    text: formatted,
                                    url: tel_url,
                                }
                            }
                        }
                    }
                }
            }
            TgMessageItem::Date(value) => {
                quote! {
                    {
                        use ::chrono::Datelike;
                        let date_value = #value;
                        ::msg::Element::text(
                            format!("{:04}-{:02}-{:02}", date_value.year(), date_value.month(), date_value.day())
                        )
                    }
                }
            }
            TgMessageItem::DateTime(value) => {
                quote! {
                    {
                        use ::chrono::{Datelike, Timelike};
                        let dt_value = #value;
                        ::msg::Element::text(
                            format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                                dt_value.year(), dt_value.month(), dt_value.day(),
                                dt_value.hour(), dt_value.minute(), dt_value.second()
                            )
                        )
                    }
                }
            }
            TgMessageItem::Time(value) => {
                quote! {
                    {
                        use ::chrono::Timelike;
                        let time_value = #value;
                        ::msg::Element::text(
                            format!("{:02}:{:02}:{:02}", time_value.hour(), time_value.minute(), time_value.second())
                        )
                    }
                }
            }
            TgMessageItem::Expression(expr) => {
                quote! { ::msg::Element::text(#expr.to_string()) }
            }
            TgMessageItem::MessageReference(expr) => {
                // Don't wrap in Element here, just return the expression
                // The msg! macro will handle flattening
                quote! { #expr }
            }
        };

        tokens.extend(result);
    }
}

fn generate_elements(items: &[TgMessageItem]) -> Vec<proc_macro2::TokenStream> {
    items
        .iter()
        .map(|item| {
            match item {
                TgMessageItem::MessageReference(_) => {
                    // For message references, we want to expand the vector inline
                    quote! { #item }
                }
                _ => quote! { #item },
            }
        })
        .collect()
}

struct TgMessage {
    items: Vec<TgMessageItem>,
}

impl Parse for TgMessage {
    fn parse(input: ParseStream) -> Result<Self> {
        let items = parse_message_items(input)?;
        Ok(TgMessage { items })
    }
}

#[proc_macro]
pub fn msg(input: TokenStream) -> TokenStream {
    let message = parse_macro_input!(input as TgMessage);

    let elements = message.items.iter().map(|item| {
        match item {
            TgMessageItem::MessageReference(expr) => {
                // For message references, we directly extend from the vector
                quote! {
                    {
                        let referenced_items = #expr;
                        for item in referenced_items {
                            result.push(item);
                        }
                    }
                }
            }
            _ => {
                // For normal items, convert to Element and push
                quote! {
                    {
                        let element = #item;
                        match element {
                            ::msg::Element::Group(mut elements) => result.append(&mut elements),
                            other => result.push(other),
                        }
                    }
                }
            }
        }
    });

    let output = quote! {
        {
            let mut result = Vec::new();
            #(#elements)*
            result
        }
    };

    output.into()
}

#[proc_macro]
pub fn el(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as TgMessageItem);
    let output = quote! { #item };
    output.into()
}
