use std::{collections::BTreeSet, fs, path::PathBuf};

use miette::{IntoDiagnostic, miette};

use crate::{
    app::{AppResult, normalize_domain},
    blocking::blocklist::BlocklistBundle,
};

#[derive(Debug, Clone)]
pub struct BlockedDomainsStore {
    path: PathBuf,
}

impl BlockedDomainsStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn ensure_seeded(&self) -> AppResult<()> {
        if self.path.exists() {
            return Ok(());
        }
        BlocklistBundle::sync_mirror(&self.path)
    }

    pub fn list(&self) -> AppResult<Vec<String>> {
        self.ensure_seeded()?;
        let contents = fs::read_to_string(&self.path).into_diagnostic()?;
        let domains = contents
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(normalize_domain)
            .collect::<AppResult<BTreeSet<_>>>()?;
        Ok(domains.into_iter().collect())
    }

    pub fn add(&self, domain: &str) -> AppResult<Vec<String>> {
        let normalized = normalize_domain(domain)?;
        let mut domains = self.list()?;
        if domains.iter().any(|existing| existing == &normalized) {
            return Err(miette!("ese dominio ya existe en el catalogo activo"));
        }
        domains.push(normalized);
        self.save(&domains)
    }

    pub fn update(&self, original: &str, replacement: &str) -> AppResult<Vec<String>> {
        let original = normalize_domain(original)?;
        let replacement = normalize_domain(replacement)?;
        let mut domains = self.list()?;
        let Some(position) = domains.iter().position(|domain| domain == &original) else {
            return Err(miette!("no se encontro el dominio seleccionado para editar"));
        };
        if replacement != original && domains.iter().any(|domain| domain == &replacement) {
            return Err(miette!("el dominio editado duplicaria una entrada existente"));
        }
        domains[position] = replacement;
        self.save(&domains)
    }

    pub fn remove(&self, domain: &str) -> AppResult<Vec<String>> {
        let domain = normalize_domain(domain)?;
        let mut removed = false;
        let domains = self
            .list()?
            .into_iter()
            .filter(|existing| {
                let keep = existing != &domain;
                if !keep {
                    removed = true;
                }
                keep
            })
            .collect::<Vec<_>>();
        if !removed {
            return Err(miette!("no se encontro el dominio seleccionado para eliminar"));
        }
        self.save(&domains)
    }

    pub fn save(&self, domains: &[String]) -> AppResult<Vec<String>> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).into_diagnostic()?;
        }
        let normalized = domains
            .iter()
            .map(|domain| normalize_domain(domain))
            .collect::<AppResult<BTreeSet<_>>>()?;
        let payload = normalized.into_iter().collect::<Vec<_>>();
        let body = if payload.is_empty() {
            String::new()
        } else {
            format!("{}\n", payload.join("\n"))
        };
        fs::write(&self.path, body).into_diagnostic()?;
        Ok(payload)
    }
}
