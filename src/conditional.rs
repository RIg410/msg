use crate::ast::{Condition, ConditionalFormat, TgElement};
use regex::Regex;

impl Condition {
    pub fn evaluate(&self, value: &str) -> bool {
        match self {
            Condition::GreaterThan(threshold) => {
                value.parse::<f64>().map_or(false, |v| v > *threshold)
            }
            Condition::LessThan(threshold) => {
                value.parse::<f64>().map_or(false, |v| v < *threshold)
            }
            Condition::Equals(expected) => value == expected,
            Condition::Contains(substring) => value.contains(substring),
            Condition::Regex(pattern) => {
                Regex::new(pattern).map_or(false, |re| re.is_match(value))
            }
            Condition::Custom(_) => false,
        }
    }
}

pub fn apply_conditional_format(
    element: TgElement,
    rules: &[ConditionalFormat],
) -> TgElement {
    if let TgElement::Text(ref text) = element {
        for rule in rules {
            if rule.condition.evaluate(text) {
                return (rule.format)(vec![element.clone()]).into_iter().next().unwrap_or(element);
            }
        }
    }
    element
}