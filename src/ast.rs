#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Text(String),
    Bold(Vec<Element>),
    Italic(Vec<Element>),
    Code(String),
    Pre(PreBlock),
    Underline(Vec<Element>),
    Strikethrough(Vec<Element>),
    Spoiler(Vec<Element>),

    Link { text: Vec<Element>, url: String },
    TextLink { text: String, url: String },

    Mention { username: String },
    MentionId { user_id: u64, text: String },

    Hashtag(String),
    Command { name: String, args: Vec<String> },
    Emoji(String),
    CustomEmoji { emoji: String, id: u64 },

    List(ListNode),
    Table(TableNode),
    Quote(Vec<Element>),

    Custom { formatter: String, value: String },

    Group(Vec<Element>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PreBlock {
    pub code: String,
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListNode {
    pub style: ListStyle,
    pub items: Vec<ListItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListStyle {
    Bullet,
    Numbered,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    pub content: Vec<Element>,
    pub nested: Option<Box<ListNode>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableNode {
    pub headers: Vec<TableCell>,
    pub rows: Vec<TableRow>,
    pub style: TableStyle,
    pub rules: Vec<ConditionalFormat>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableCell {
    pub content: Vec<Element>,
    pub align: CellAlign,
    pub colspan: usize,
    pub rowspan: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableStyle {
    Ascii,
    Unicode,
    Minimal,
    Compact,
}

#[derive(Debug, Clone)]
pub struct ConditionalFormat {
    pub condition: Condition,
    pub format: fn(Vec<Element>) -> Vec<Element>,
}

impl PartialEq for ConditionalFormat {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    GreaterThan(f64),
    LessThan(f64),
    Equals(String),
    Contains(String),
    Regex(String),
    Custom(String),
}

impl Default for TableCell {
    fn default() -> Self {
        Self {
            content: Vec::new(),
            align: CellAlign::Left,
            colspan: 1,
            rowspan: 1,
        }
    }
}

impl Element {
    pub fn text(s: impl Into<String>) -> Self {
        Element::Text(s.into())
    }

    pub fn bold(elements: Vec<Element>) -> Self {
        Element::Bold(elements)
    }

    pub fn italic(elements: Vec<Element>) -> Self {
        Element::Italic(elements)
    }

    pub fn code(s: impl Into<String>) -> Self {
        Element::Code(s.into())
    }

    pub fn pre(code: impl Into<String>, language: Option<String>) -> Self {
        Element::Pre(PreBlock {
            code: code.into(),
            language,
        })
    }

    pub fn link(text: Vec<Element>, url: impl Into<String>) -> Self {
        Element::Link {
            text,
            url: url.into(),
        }
    }

    pub fn text_link(text: impl Into<String>, url: impl Into<String>) -> Self {
        Element::TextLink {
            text: text.into(),
            url: url.into(),
        }
    }

    pub fn mention(username: impl Into<String>) -> Self {
        Element::Mention {
            username: username.into(),
        }
    }

    pub fn hashtag(tag: impl Into<String>) -> Self {
        Element::Hashtag(tag.into())
    }

    pub fn group(elements: Vec<Element>) -> Self {
        Element::Group(elements)
    }

    pub fn underline(elements: Vec<Element>) -> Self {
        Element::Underline(elements)
    }

    pub fn strikethrough(elements: Vec<Element>) -> Self {
        Element::Strikethrough(elements)
    }

    pub fn spoiler(elements: Vec<Element>) -> Self {
        Element::Spoiler(elements)
    }
}
