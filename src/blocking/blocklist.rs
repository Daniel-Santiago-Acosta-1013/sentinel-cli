use std::{collections::BTreeSet, time::SystemTime};

use chrono::{DateTime, Utc};
use miette::{Result, miette};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BlocklistBundle {
    pub bundle_id: String,
    pub version: String,
    pub domain_count: usize,
    pub source_label: String,
    pub loaded_at: DateTime<Utc>,
    pub integrity_state: bool,
    domains: BTreeSet<String>,
}

impl BlocklistBundle {
    pub fn load() -> Result<Self> {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/blocklists/default-domains.txt"
        ));
        let domains = raw
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|domain| domain.trim_end_matches('.').to_lowercase())
            .collect::<BTreeSet<_>>();
        if domains.is_empty() {
            return Err(miette!("the bundled ad-domain list is empty"));
        }

        let modified = SystemTime::UNIX_EPOCH;
        let loaded_at = DateTime::<Utc>::from(modified);
        Ok(Self {
            bundle_id: "sentinel-default-domains".to_owned(),
            version: format!("{}-{}", env!("CARGO_PKG_VERSION"), domains.len()),
            domain_count: domains.len(),
            source_label: "bundled-curated-domains".to_owned(),
            loaded_at,
            integrity_state: domains.len() >= 50,
            domains,
        })
    }

    pub fn matches(&self, domain: &str) -> bool {
        let normalized = domain.trim().trim_end_matches('.').to_lowercase();
        if self.domains.contains(&normalized) {
            return true;
        }
        normalized.split_once('.').is_some_and(|_| {
            self.domains.iter().any(|entry| {
                normalized == *entry || normalized.ends_with(&format!(".{entry}"))
            })
        })
    }
}
