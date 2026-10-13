//! Bounded declarative trigger conditions. Execution hosts obtain matches from pinned source reads.
use crate::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowConditions")]
pub struct Conditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revset: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub authors: BTreeSet<String>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub excluded_authors: BTreeSet<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<TextCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excluded_emoji: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowTextCondition")]
pub struct TextCondition {
    pub operator: TextOperator,
    pub value: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "WorkflowTextOperator")]
pub enum TextOperator {
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    Equals,
    NotEquals,
    IsEmpty,
    IsNotEmpty,
}
impl TextCondition {
    pub fn matches(&self, text: &str) -> bool {
        match self.operator {
            TextOperator::Contains => text.contains(&self.value),
            TextOperator::NotContains => !text.contains(&self.value),
            TextOperator::StartsWith => text.starts_with(&self.value),
            TextOperator::EndsWith => text.ends_with(&self.value),
            TextOperator::Equals => text == self.value,
            TextOperator::NotEquals => text != self.value,
            TextOperator::IsEmpty => text.is_empty(),
            TextOperator::IsNotEmpty => !text.is_empty(),
        }
    }
}
impl Conditions {
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
    pub fn id(&self) -> Result<FilterId> {
        use sha2::{Digest, Sha256};
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Invalid("condition encoding"))?;
        Ok(FilterId::new(Sha256::digest(bytes).into()))
    }
    pub fn validate(&self) -> Result<()> {
        if self.authors.len() + self.excluded_authors.len() > 16 {
            return Err(Error::Limit);
        }
        for author in self.authors.union(&self.excluded_authors) {
            text(author, 512)?;
            if author.trim().is_empty() {
                return Err(Error::Invalid("author condition"));
            }
        }
        if let Some(revset) = &self.revset {
            text(revset, 4096)?;
            if revset.trim().is_empty() {
                return Err(Error::Invalid("revset condition"));
            }
        }
        if let Some(condition) = &self.text {
            text(&condition.value, 4096)?;
        }
        for id in [&self.thread, &self.message].into_iter().flatten() {
            if id.len() != 32
                || !id
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            {
                return Err(Error::Invalid("conversation condition identity"));
            }
        }
        for emoji in [&self.emoji, &self.excluded_emoji].into_iter().flatten() {
            text(emoji, 128)?;
            if emoji.is_empty() {
                return Err(Error::Invalid("reaction condition"));
            }
        }
        Ok(())
    }
    pub fn matches_content(
        &self,
        author: &str,
        content: &str,
        thread: Option<&str>,
        message: Option<&str>,
        emoji: Option<&str>,
    ) -> bool {
        (self.authors.is_empty() || self.authors.contains(author))
            && !self.excluded_authors.contains(author)
            && self
                .excluded_emoji
                .as_deref()
                .is_none_or(|v| Some(v) != emoji)
            && self.text.as_ref().is_none_or(|c| c.matches(content))
            && self.thread.as_deref().is_none_or(|v| Some(v) == thread)
            && self.message.as_deref().is_none_or(|v| Some(v) == message)
            && self.emoji.as_deref().is_none_or(|v| Some(v) == emoji)
    }
}
/// Match literal bookmark names, `*` and `?`, without an executable expression language.
pub fn bookmark_matches(pattern: &str, value: &str) -> bool {
    let value: Vec<_> = value.chars().collect();
    let mut row = vec![false; value.len() + 1];
    row[0] = true;
    for ch in pattern.chars() {
        let mut next = vec![false; value.len() + 1];
        if ch == '*' {
            next[0] = row[0];
        }
        for index in 1..=value.len() {
            next[index] = if ch == '*' {
                row[index] || next[index - 1]
            } else {
                row[index - 1] && (ch == '?' || ch == value[index - 1])
            };
        }
        row = next;
    }
    row[value.len()]
}
