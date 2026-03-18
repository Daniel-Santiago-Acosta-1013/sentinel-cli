use crate::storage::install::InstallAction;

pub fn current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[allow(dead_code)]
pub fn current_release_tag() -> String {
    format!("v{}", current_version())
}

pub fn decide_action(
    installed_version: Option<&str>,
    install_broken: bool,
) -> InstallAction {
    match (installed_version, install_broken) {
        (_, true) => InstallAction::Reinstall,
        (None, false) => InstallAction::Install,
        (Some(version), false) if version != current_version() => InstallAction::Update,
        _ => InstallAction::None,
    }
}
