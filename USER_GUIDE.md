# MSG Library - User Guide

A powerful Rust library for building formatted messages with support for Telegram formatting and more.

## Table of Contents
- [Installation](#installation)
- [Basic Usage](#basic-usage)
- [Text Formatting](#text-formatting)
- [Links and Mentions](#links-and-mentions)
- [Lists](#lists)
- [Tables](#tables)
- [Code Blocks](#code-blocks)
- [Date and Time](#date-and-time)
- [Phone Numbers](#phone-numbers)
- [Message Substitution](#message-substitution)
- [Expressions and Variables](#expressions-and-variables)
- [Output Generation](#output-generation)

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
msg = "0.1.0"
chrono = "0.4"  # Required for date/time features
regex = "1.11"  # Required for URL auto-detection
```

## Basic Usage

```rust
use msg::*;

fn main() {
    // Simple text message
    let message = msg! { "Hello, world!" };
    
    // Multiple elements
    let message = msg! { "Hello, " "world" "!" };
}
```

## Text Formatting

### Bold Text
```rust
let message = msg! { bold { "Bold text" } };
```

### Italic Text
```rust
let message = msg! { italic { "Italic text" } };
```

### Underline Text
```rust
let message = msg! { underline { "Underlined text" } };
```

### Strikethrough Text
```rust
let message = msg! { strikethrough { "Strikethrough text" } };
```

### Spoiler Text
```rust
let message = msg! { spoiler { "This is a spoiler" } };
```

### Combined Formatting
```rust
let message = msg! {
    bold { "Bold " italic { "and italic" } " text" }
};
```

## Links and Mentions

### Text Links
```rust
// Basic link
let message = msg! { link("https://example.com") { "Click here" } };

// URLs in text are auto-detected
let message = msg! { "Check out https://example.com for more info" };
```

### User Mentions
```rust
// Using @ syntax
let message = msg! { @username };

// Using mention function
let message = msg! { mention { "username" } };

// With variables
let user = "alice";
let message = msg! { mention { (user) } };
```

### Hashtags
```rust
// Using # syntax for simple hashtags (starting with uppercase)
let message = msg! { #RustLang };

// Using hashtag function
let message = msg! { hashtag { "programming" } };

// With variables
let tag = "opensource";
let message = msg! { hashtag { (tag) } };
```

## Lists

### Bullet Lists
```rust
let message = msg! {
    list {
        - "First item";
        - "Second item";
        - "Third item";
    }
};
```

### Numbered Lists
```rust
let message = msg! {
    list(numbered) {
        - "Step one";
        - "Step two";
        - "Step three";
    }
};
```

### Custom List Markers
```rust
let message = msg! {
    list(arrow) {
        - "Item A";
        - "Item B";
    }
};
```

## Tables

```rust
let message = msg! {
    table {
        headers: ["Name", "Age", "City"]
        rows: [
            ["Alice", "30", "New York"],
            ["Bob", "25", "London"],
            ["Charlie", "35", "Paris"]
        ]
    }
};
```

## Code Blocks

### Inline Code
```rust
let message = msg! {
    "Use " code { "println!()" } " to print"
};
```

### Code Blocks with Language
```rust
let message = msg! {
    pre("rust") {
        "fn main() {\n    println!(\"Hello!\");\n}"
    }
};
```

### Code Blocks without Language
```rust
let message = msg! {
    pre {
        "This is a\npreformatted\ncode block"
    }
};
```

## Date and Time

Requires `chrono` crate:

```rust
use chrono::{Local, NaiveDate};

let message = msg! {
    "Today is " date(Local::now())
};

let message = msg! {
    "Current time: " time(Local::now())
};

let message = msg! {
    "Timestamp: " datetime(Local::now())
};
```

## Phone Numbers

```rust
// Format: +X(XXX) XXX-XX-XX
let message = msg! {
    +7("9991234567")  // +7 (999) 123-45-67
};

let message = msg! {
    +("9991234567")   // + (999) 123-45-67
};

// With variable
let number = "9991234567";
let message = msg! {
    +7((number))
};
```

## Message Substitution

The `#variable` syntax is used to substitute (insert) one message into another. This allows combining pre-built messages:

### Basic Substitution
```rust
let greeting = msg! { "Hello" };
let message = msg! { #greeting.clone() ", world!" };
// Result: ["Hello", ", world!"]
```

### Multiple Substitutions
```rust
let part1 = msg! { "Part 1" };
let part2 = msg! { "Part 2" };
let combined = msg! { 
    #part1.clone() " and " #part2.clone() 
};
```

### Complex Composition
```rust
let header = msg! { bold { "Header:" } };
let content = msg! { "Some content" };
let footer = msg! { italic { "Footer" } };

let document = msg! {
    #header.clone() "\n"
    #content.clone() "\n"
    "---\n"
    #footer.clone()
};
```

### Function Composition
```rust
fn create_greeting(name: &str) -> Vec<Element> {
    msg! { "Hello, " bold { (name) } "!" }
}

let greeting = create_greeting("Alice");
let full_message = msg! { 
    #greeting.clone() " Welcome!" 
};
```

**Important**: 
- The `#` symbol is used specifically for substituting other messages into the current one
- Use `.clone()` when substituting to avoid moving the original value
- The substituted message must be of type `Vec<Element>`

## Expressions and Variables

### Using Variables
```rust
let name = "Alice";
let age = 30;

let message = msg! {
    "Name: " (name) ", Age: " (age)
};
```

### String Interpolation
```rust
let user = "Bob";
let count = 5;

let message = msg! {
    (format!("{} has {} messages", user, count))
};
```

### Conditional Content
```rust
let is_admin = true;
let role = if is_admin { "Admin" } else { "User" };

let message = msg! {
    "Role: " bold { (role) }
};
```

## Output Generation

### Generating Formatted Output
```rust
use msg::{Generator, ParseMode};

let message = msg! {
    bold { "Title" } "\n"
    italic { "Subtitle" } "\n"
    "Regular text"
};

// Generate Markdown
let generator = Generator::new(ParseMode::Markdown);
let mut output = String::new();
for element in &message {
    generator.generate(&mut output, element).unwrap();
}
// Output: **Title**\n*Subtitle*\nRegular text

// Generate Telegram MarkdownV2
let generator = Generator::new(ParseMode::MarkdownV2);
let mut output = String::new();
for element in &message {
    generator.generate(&mut output, element).unwrap();
}
// Output: *Title*\n_Subtitle_\nRegular text
```

### Available Parse Modes
- `ParseMode::Plain` - No formatting
- `ParseMode::Markdown` - Standard Markdown
- `ParseMode::MarkdownV2` - Telegram MarkdownV2
- `ParseMode::HTML` - HTML formatting

## Advanced Examples

### Newsletter Template
```rust
fn create_newsletter(title: &str, articles: Vec<(&str, &str)>) -> Vec<Element> {
    let header = msg! { 
        bold { (title) } "\n"
        "=" "\n\n"
    };
    
    let mut content = header;
    
    for (headline, summary) in articles {
        let article = msg! {
            "• " bold { (headline) } "\n"
            "  " (summary) "\n\n"
        };
        // Substitute article into overall content
        content = msg! { #content.clone() #article };
    }
    
    let footer = msg! {
        "---\n"
        italic { "© 2024 Newsletter Inc." }
    };
    
    // Substitute footer to content
    msg! { #content #footer }
}
```

### Error Messages with Formatting
```rust
fn format_error(code: i32, message: &str, details: Option<&str>) -> Vec<Element> {
    let base = msg! {
        bold { "Error " (code) ": " } (message)
    };
    
    if let Some(details) = details {
        // Substitute base message and add details
        msg! {
            #base "\n"
            italic { "Details: " (details) }
        }
    } else {
        base
    }
}
```

### Dynamic List Generation
```rust
fn create_task_list(tasks: Vec<(String, bool)>) -> Vec<Element> {
    let mut result = msg! { "Tasks:\n" };
    
    for (task, completed) in tasks {
        let status = if completed { "✓" } else { "○" };
        let item = msg! {
            (status) " " 
            if completed {
                strikethrough { (task) }
            } else {
                (task)
            }
            "\n"
        };
        // Substitute each item into result
        result = msg! { #result.clone() #item };
    }
    result
}
```

## Best Practices

1. **Use `.clone()` for Substitutions**: When using `#variable` syntax, always use `.clone()` to avoid moving values.

2. **Combine Formatting**: Nest formatting functions for complex styles:
   ```rust
   msg! { bold { italic { "Bold and Italic" } } }
   ```

3. **Modular Message Building**: Create functions that return `Vec<Element>` for reusable components.

4. **Handle Special Characters**: The generator automatically escapes special characters based on the output format.

5. **Performance**: Pre-build commonly used message parts and substitute them with `#variable` syntax.

## Element Helper Functions

The library provides helper functions for programmatic element creation:

```rust
use msg::Element;

// Direct element creation
let text = Element::text("Hello");
let bold = Element::bold(vec![Element::text("Bold")]);
let link = Element::link(
    vec![Element::text("Click")], 
    "https://example.com"
);
```

## Error Handling

```rust
use msg::{Generator, ParseMode, Error};

let generator = Generator::new(ParseMode::MarkdownV2);
let mut output = String::new();

match generator.generate(&mut output, &element) {
    Ok(()) => println!("Generated: {}", output),
    Err(Error::UnsupportedElement(el)) => {
        eprintln!("Unsupported element: {:?}", el);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Core Macros

### `msg!`
Main macro for creating messages. Returns `Vec<Element>`.

### `el!`
Macro for creating a single element. Returns `Element`.

```rust
let single = el!(bold { "Single element" });
let message = msg! { #vec![single] " and more text" };
```