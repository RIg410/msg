#[derive(Debug, Clone, PartialEq)]
pub enum TgElement {
    Text(String),
    Bold(Vec<TgElement>),
    Italic(Vec<TgElement>),
    Code(String),
    Pre(PreBlock),
    Underline(Vec<TgElement>),
    Strikethrough(Vec<TgElement>),
    Spoiler(Vec<TgElement>),
    
    Link { text: Vec<TgElement>, url: String },
    TextLink { text: String, url: String },
    
    Mention { username: String },
    MentionId { user_id: u64, text: String },
    
    Hashtag(String),
    Command { name: String, args: Vec<String> },
    Emoji(String),
    CustomEmoji { emoji: String, id: u64 },
    
    List(ListNode),
    Table(TableNode),
    Quote(Vec<TgElement>),
    
    Custom { formatter: String, value: String },
    
    Group(Vec<TgElement>),
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
    pub content: Vec<TgElement>,
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
    pub content: Vec<TgElement>,
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
    pub format: fn(Vec<TgElement>) -> Vec<TgElement>,
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

impl TgElement {
    pub fn text(s: impl Into<String>) -> Self {
        TgElement::Text(s.into())
    }
    
    pub fn bold(elements: Vec<TgElement>) -> Self {
        TgElement::Bold(elements)
    }
    
    pub fn italic(elements: Vec<TgElement>) -> Self {
        TgElement::Italic(elements)
    }
    
    pub fn code(s: impl Into<String>) -> Self {
        TgElement::Code(s.into())
    }
    
    pub fn pre(code: impl Into<String>, language: Option<String>) -> Self {
        TgElement::Pre(PreBlock {
            code: code.into(),
            language,
        })
    }
    
    pub fn link(text: Vec<TgElement>, url: impl Into<String>) -> Self {
        TgElement::Link {
            text,
            url: url.into(),
        }
    }
    
    pub fn text_link(text: impl Into<String>, url: impl Into<String>) -> Self {
        TgElement::TextLink {
            text: text.into(),
            url: url.into(),
        }
    }
    
    pub fn mention(username: impl Into<String>) -> Self {
        TgElement::Mention {
            username: username.into(),
        }
    }
    
    pub fn hashtag(tag: impl Into<String>) -> Self {
        TgElement::Hashtag(tag.into())
    }
    
    pub fn group(elements: Vec<TgElement>) -> Self {
        TgElement::Group(elements)
    }
}