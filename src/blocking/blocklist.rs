use std::{
    collections::BTreeSet,
    fs,
    path::Path,
    time::SystemTime,
};

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
    pub fn bundled_source() -> &'static str {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/blocklists/default-domains.txt"
        ))
    }

    pub fn sync_mirror(path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| miette!("failed to create blocklist directory: {err}"))?;
        }

        fs::write(path, Self::bundled_source())
            .map_err(|err| miette!("failed to sync blocklist mirror: {err}"))?;
        Ok(())
    }

    pub fn load_from_path(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path)
            .map_err(|err| miette!("failed to read blocklist mirror {}: {err}", path.display()))?;
        let modified = fs::metadata(path)
            .and_then(|meta| meta.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);
        Self::parse(
            &raw,
            modified,
            "mirrored-local-blocklist".to_owned(),
        )
    }

    #[allow(dead_code)]
    pub fn load() -> Result<Self> {
        Self::parse(
            Self::bundled_source(),
            SystemTime::UNIX_EPOCH,
            "bundled-curated-domains".to_owned(),
        )
    }

    fn parse(
        raw: &str,
        modified: SystemTime,
        source_label: String,
    ) -> Result<Self> {
        let domains = raw
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|domain| domain.trim_end_matches('.').to_lowercase())
            .collect::<BTreeSet<_>>();
        if domains.is_empty() {
            return Err(miette!("the bundled ad-domain list is empty"));
        }

        let loaded_at = DateTime::<Utc>::from(modified);
        Ok(Self {
            bundle_id: "sentinel-default-domains".to_owned(),
            version: format!("{}-{}", env!("CARGO_PKG_VERSION"), domains.len()),
            domain_count: domains.len(),
            source_label,
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

#[cfg(test)]
mod tests {
    use super::BlocklistBundle;

    #[test]
    fn sync_mirror_writes_the_full_bundled_blocklist() {
        let temp = tempfile::tempdir().expect("tempdir");
        let blocklist_path = temp.path().join("blocklist.txt");

        BlocklistBundle::sync_mirror(&blocklist_path).expect("sync mirror");

        let mirrored = std::fs::read_to_string(&blocklist_path).expect("read mirror");
        assert_eq!(mirrored, BlocklistBundle::bundled_source());
    }

    #[test]
    fn mirrored_blocklist_loads_with_the_expected_domain_count() {
        let temp = tempfile::tempdir().expect("tempdir");
        let blocklist_path = temp.path().join("blocklist.txt");

        BlocklistBundle::sync_mirror(&blocklist_path).expect("sync mirror");
        let bundle = BlocklistBundle::load_from_path(&blocklist_path).expect("load mirror");

        assert_eq!(bundle.domain_count, 121);
        assert_eq!(bundle.version, "0.1.1-121");
        assert_eq!(bundle.source_label, "mirrored-local-blocklist");
        assert!(bundle.matches("pixel.rubiconproject.com"));
        assert!(bundle.matches("subdomain.zemanta.com"));
    }
}
