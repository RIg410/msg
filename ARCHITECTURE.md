# Telegram Message Builder - Архитектура

## Обзор проекта

Библиотека для генерации и парсинга сообщений Telegram с поддержкой MarkdownV2 и HTML форматов. Вдохновлена архитектурой `syn` и `quote` из экосистемы Rust.

## Основные компоненты

### 1. Токенизация

```rust
// src/token.rs
pub enum Token {
    // Текст и базовое форматирование
    Text(String),
    Bold,
    Italic,
    Code,
    Pre,
    Underline,
    Strikethrough,
    Spoiler,
    
    // Ссылки и упоминания
    Link(String),
    Mention(String),
    Hashtag(String),
    Command(String),
    
    // Специальные символы
    Emoji(String),
    CustomEmoji(String, u64), // emoji, id
    
    // Структурные элементы
    LineBreak,
    Escape(char),
    
    // Для парсинга
    Eof,
}
```

### 2. AST (Abstract Syntax Tree)

```rust
// src/ast.rs
#[derive(Debug, Clone, PartialEq)]
pub enum TgElement {
    // Базовые элементы
    Text(String),
    Bold(Vec<TgElement>),
    Italic(Vec<TgElement>),
    Code(String),
    Pre(PreBlock),
    Underline(Vec<TgElement>),
    Strikethrough(Vec<TgElement>),
    Spoiler(Vec<TgElement>),
    
    // Ссылки
    Link { text: Vec<TgElement>, url: String },
    TextLink { text: String, url: String },
    
    // Упоминания
    Mention { username: String },
    MentionId { user_id: u64, text: String },
    
    // Специальные
    Hashtag(String),
    Command { name: String, args: Vec<String> },
    Emoji(String),
    CustomEmoji { emoji: String, id: u64 },
    
    // Сложные структуры
    List(ListNode),
    Table(TableNode),
    Quote(Vec<TgElement>),
    
    // Кастомные форматеры
    Custom { formatter: String, value: String },
    
    // Группа элементов
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
```

### 3. Парсер

```rust
// src/parser.rs
pub trait Parse: Sized {
    fn parse(input: ParseStream) -> Result<Self>;
}

pub struct ParseStream<'a> {
    tokens: &'a [Token],
    cursor: usize,
}

impl<'a> ParseStream<'a> {
    pub fn peek(&self) -> Option<&Token>;
    pub fn advance(&mut self) -> Option<Token>;
    pub fn lookahead<F>(&self, f: F) -> bool;
    pub fn parse<T: Parse>(&mut self) -> Result<T>;
}
```

### 4. Генератор

```rust
// src/generator.rs
pub trait Generate {
    fn generate(&self, mode: ParseMode) -> Result<String>;
}

#[derive(Debug, Clone, Copy)]
pub enum ParseMode {
    MarkdownV2,
    Html,
}

pub struct Generator {
    mode: ParseMode,
    formatters: HashMap<String, Box<dyn CustomFormatter>>,
}
```

### 5. Кастомные форматеры

```rust
// src/formatter.rs
pub trait CustomFormatter: Send + Sync {
    fn name(&self) -> &str;
    fn format(&self, value: &str, mode: ParseMode) -> Result<String>;
    fn parse(&self, input: &str) -> Option<(String, usize)>;
}

// Встроенные форматеры
pub struct PhoneFormatter;
pub struct DateFormatter;
pub struct TimeFormatter;
pub struct CurrencyFormatter;
pub struct EmailFormatter;
pub struct PercentFormatter;
pub struct ProgressFormatter;
```

### 6. Макросы

```rust
// src/macros.rs
#[macro_export]
macro_rules! tg_message {
    // Основной макрос для создания сообщений
}

#[proc_macro]
pub fn tg_quote(input: TokenStream) -> TokenStream {
    // Процедурный макрос для квази-цитирования
}
```

### 7. Условное форматирование

```rust
// src/conditional.rs
pub enum Condition {
    GreaterThan(f64),
    LessThan(f64),
    Equals(String),
    Contains(String),
    Regex(regex::Regex),
    Custom(Box<dyn Fn(&str) -> bool>),
}

pub struct ConditionalFormat {
    pub condition: Condition,
    pub format: Box<dyn Fn(Vec<TgElement>) -> Vec<TgElement>>,
}
```

## План реализации

### Фаза 1: Базовая инфраструктура
1. **Создать структуру проекта**
   - [ ] Cargo.toml с зависимостями
   - [ ] Модульная структура (lib.rs, token.rs, ast.rs, etc.)
   - [ ] Настройка CI/CD

2. **Реализовать токенизацию**
   - [ ] Определить все типы токенов
   - [ ] Лексер для MarkdownV2
   - [ ] Лексер для HTML
   - [ ] Тесты для токенизации

3. **Реализовать AST**
   - [ ] Базовые структуры данных
   - [ ] Методы конструирования
   - [ ] Display trait для отладки
   - [ ] Тесты для AST

### Фаза 2: Парсинг
4. **Базовый парсер**
   - [ ] ParseStream и вспомогательные функции
   - [ ] Парсинг простого текста
   - [ ] Парсинг форматирования (bold, italic, etc.)
   - [ ] Тесты парсинга

5. **Парсинг сложных элементов**
   - [ ] Парсинг ссылок и упоминаний
   - [ ] Парсинг списков
   - [ ] Парсинг таблиц
   - [ ] Обработка ошибок
   - [ ] Тесты для сложных элементов

### Фаза 3: Генерация
6. **Базовый генератор**
   - [ ] Trait Generate
   - [ ] Генерация MarkdownV2
   - [ ] Генерация HTML
   - [ ] Экранирование специальных символов
   - [ ] Тесты генерации

7. **Генерация сложных структур**
   - [ ] Рендеринг списков
   - [ ] Рендеринг таблиц
   - [ ] Адаптивный рендеринг
   - [ ] Тесты для сложных структур

### Фаза 4: Расширенный функционал
8. **Кастомные форматеры**
   - [ ] Trait CustomFormatter
   - [ ] Встроенные форматеры (phone, date, etc.)
   - [ ] Регистрация форматеров
   - [ ] Тесты форматеров

9. **Условное форматирование**
   - [ ] Система условий
   - [ ] Применение правил
   - [ ] Интеграция с таблицами
   - [ ] Тесты условного форматирования

### Фаза 5: Макросы
10. **Декларативные макросы**
    - [ ] macro_rules! tg_message
    - [ ] Парсинг синтаксиса макроса
    - [ ] Генерация AST из макроса
    - [ ] Тесты макросов

11. **Процедурные макросы**
    - [ ] tg_quote! для квази-цитирования
    - [ ] derive макросы для кастомных типов
    - [ ] Тесты процедурных макросов

### Фаза 6: Интеграция и оптимизация
12. **Интеграция**
    - [ ] Примеры использования
    - [ ] Интеграция с telegram-bot API
    - [ ] Бенчмарки производительности
    - [ ] Документация

13. **Оптимизация**
    - [ ] Оптимизация парсера
    - [ ] Кеширование результатов
    - [ ] Уменьшение аллокаций
    - [ ] Профилирование

## Тестирование

### Модульные тесты
- Тесты для каждого модуля
- Тесты граничных случаев
- Тесты ошибок

### Интеграционные тесты
- Парсинг и генерация полных сообщений
- Работа с реальными примерами из Telegram
- Тесты производительности

### Property-based тесты
- Использование proptest/quickcheck
- Инварианты: parse(generate(ast)) == ast
- Fuzzing для поиска краевых случаев

### Примеры тестов
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_text() {
        let input = "Hello, world!";
        let ast = parse(input).unwrap();
        assert_eq!(ast, TgElement::Text("Hello, world!".to_string()));
    }

    #[test]
    fn test_generate_markdown() {
        let ast = TgElement::Bold(vec![TgElement::Text("Bold text".to_string())]);
        let output = ast.generate(ParseMode::MarkdownV2).unwrap();
        assert_eq!(output, "*Bold text*");
    }

    #[test]
    fn test_roundtrip() {
        let original = "Hello *bold* and _italic_ text";
        let ast = parse(original).unwrap();
        let generated = ast.generate(ParseMode::MarkdownV2).unwrap();
        let reparsed = parse(&generated).unwrap();
        assert_eq!(ast, reparsed);
    }
}
```

## Зависимости

```toml
[dependencies]
thiserror = "1.0"
regex = "1.10"
lazy_static = "1.4"
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }

[dev-dependencies]
proptest = "1.4"
criterion = "0.5"
pretty_assertions = "1.4"

[build-dependencies]
proc-macro2 = "1.0"
quote = "1.0"
syn = "2.0"
```

## Совместимость

- Rust 1.70+
- Поддержка no_std (опционально)
- WASM совместимость