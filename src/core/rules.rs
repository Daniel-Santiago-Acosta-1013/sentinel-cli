use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::app::{AppResult, normalize_domain};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuleKind {
    Block,
    Allow,
}

impl RuleKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Block => "block",
            Self::Allow => "allow",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MatchType {
    Domain,
    Suffix,
}

impl MatchType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Domain => "domain",
            Self::Suffix => "suffix",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuleSource {
    BuiltIn,
    User,
}

impl RuleSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BuiltIn => "built_in",
            Self::User => "user",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuleEntry {
    pub id: String,
    pub kind: RuleKind,
    pub match_type: MatchType,
    pub value: String,
    pub source: RuleSource,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RuleEntry {
    pub fn allow(domain: &str) -> AppResult<Self> {
        let normalized = normalize_domain(domain)?;
        Ok(Self {
            id: format!("allow-{normalized}"),
            kind: RuleKind::Allow,
            match_type: MatchType::Suffix,
            value: normalized,
            source: RuleSource::User,
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn blocks(&self, domain: &str) -> bool {
        match self.match_type {
            MatchType::Domain => self.value == domain,
            MatchType::Suffix => {
                domain == self.value || domain.ends_with(&format!(".{}", self.value))
            }
        }
    }
}

pub fn built_in_rules() -> Vec<RuleEntry> {
    let now = Utc::now();
    [
        "doubleclick.net",
        "googlesyndication.com",
        "googleadservices.com",
        "ads-twitter.com",
        "adservice.google.com",
        "taboola.com",
        "outbrain.com",
    ]
    .into_iter()
    .map(|domain| RuleEntry {
        id: format!("block-{domain}"),
        kind: RuleKind::Block,
        match_type: MatchType::Suffix,
        value: domain.to_owned(),
        source: RuleSource::BuiltIn,
        enabled: true,
        created_at: now,
        updated_at: now,
    })
    .collect()
}

pub fn should_block(
    domain: &str,
    built_in: &[RuleEntry],
    allow_rules: &[RuleEntry],
) -> bool {
    if allow_rules.iter().any(|rule| rule.enabled && rule.blocks(domain)) {
        return false;
    }

    built_in.iter().any(|rule| rule.enabled && rule.blocks(domain))
}
