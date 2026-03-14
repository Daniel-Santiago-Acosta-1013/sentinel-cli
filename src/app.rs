use std::{
    env,
    net::SocketAddr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    str::FromStr,
    time::Duration,
};

use directories::ProjectDirs;
use miette::{Context, IntoDiagnostic, Result, miette};
use serde_json::{Value, json};
use thiserror::Error;
use tokio::time::sleep;
use tracing_subscriber::EnvFilter;

use crate::{
    cli::{
        GlobalOptions, Parser,
        commands::{AllowCommand, Cli, Commands, RulesCommand},
        output::{render_events, render_rules, render_status},
    },
    control::{coordinator::Coordinator, recovery::RecoveryManager},
    core::{
        events::{OperationEvent, OperationEventKind, Severity},
        rules::built_in_rules,
        state::ProtectionMode,
    },
    engine::runtime::run_runtime,
    storage::{config::ConfigStore, events::EventStore, state::StateStore},
};

pub type AppResult<T> = Result<T>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("command requires elevated privileges")]
    PermissionDenied,
    #[error("background runtime failed to start")]
    RuntimeStartFailed,
    #[error("invalid domain value: {0}")]
    InvalidDomain(String),
}

/// Runtime paths and files used by the CLI and background runtime.
#[derive(Debug, Clone)]
pub struct AppPaths {
    pub state_dir: PathBuf,
    pub config_file: PathBuf,
    pub state_file: PathBuf,
    pub events_file: PathBuf,
    pub snapshots_dir: PathBuf,
}

impl AppPaths {
    pub fn discover() -> AppResult<Self> {
        let root = if let Ok(custom_root) = env::var("SENTINEL_HOME") {
            PathBuf::from(custom_root)
        } else {
            let project_dirs = ProjectDirs::from("com", "Sentinel", "sentinel-cli")
                .ok_or_else(|| {
                    miette!("unable to determine application support directory")
                })?;
            project_dirs.data_dir().to_path_buf()
        };

        let config_dir = root.join("config");
        let data_dir = root.join("data");
        let state_dir = root.join("state");
        let snapshots_dir = state_dir.join("snapshots");

        std::fs::create_dir_all(&config_dir).into_diagnostic()?;
        std::fs::create_dir_all(&data_dir).into_diagnostic()?;
        std::fs::create_dir_all(&state_dir).into_diagnostic()?;
        std::fs::create_dir_all(&snapshots_dir).into_diagnostic()?;

        Ok(Self {
            state_dir: state_dir.clone(),
            config_file: config_dir.join("config.toml"),
            state_file: state_dir.join("state.json"),
            events_file: state_dir.join("events.jsonl"),
            snapshots_dir,
        })
    }

    pub fn runtime_addr(&self) -> AppResult<SocketAddr> {
        let port = env::var("SENTINEL_DNS_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(53);
        SocketAddr::from_str(&format!("127.0.0.1:{port}")).into_diagnostic()
    }
}

pub async fn run() -> AppResult<()> {
    init_tracing()?;

    let cli = Cli::parse();
    let paths = AppPaths::discover()?;
    let app = Application::new(paths, cli.global)?;

    match cli.command {
        Commands::Enable => app.enable().await,
        Commands::Disable => app.disable().await,
        Commands::Status => app.status().await,
        Commands::Allow { command } => match command {
            AllowCommand::Add { domain } => app.allow_add(&domain).await,
            AllowCommand::Remove { domain } => app.allow_remove(&domain).await,
        },
        Commands::Rules { command } => match command {
            RulesCommand::List => app.rules_list().await,
        },
        Commands::Recover => app.recover().await,
        Commands::Events { limit } => app.events(limit).await,
        Commands::Serve => run_runtime(app.paths.clone()).await,
    }
}

fn init_tracing() -> AppResult<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .try_init()
        .map_err(|err| miette!(err.to_string()))
        .or_else(|err| {
            if err
                .to_string()
                .contains("global default trace dispatcher has already been set")
            {
                Ok(())
            } else {
                Err(err)
            }
        })
}

pub fn require_privileges() -> AppResult<()> {
    if env::var("SENTINEL_FAKE_PLATFORM").ok().as_deref() == Some("1") {
        return Ok(());
    }

    let output = Command::new("id")
        .arg("-u")
        .output()
        .into_diagnostic()
        .context("failed to determine current privileges")?;
    let uid = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if uid == "0" { Ok(()) } else { Err(miette!(AppError::PermissionDenied)) }
}

pub fn process_alive(pid: u32) -> bool {
    Command::new("kill")
        .arg("-0")
        .arg(pid.to_string())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn stop_process(pid: u32) -> AppResult<()> {
    if !process_alive(pid) {
        return Ok(());
    }

    Command::new("kill")
        .arg("-TERM")
        .arg(pid.to_string())
        .status()
        .into_diagnostic()
        .context("failed to stop runtime process")?;
    Ok(())
}

pub fn normalize_domain(domain: &str) -> AppResult<String> {
    let trimmed = domain.trim().trim_end_matches('.').to_lowercase();
    if trimmed.is_empty()
        || trimmed.contains(' ')
        || !trimmed.contains('.')
        || trimmed.starts_with('.')
    {
        return Err(miette!(AppError::InvalidDomain(domain.to_owned())));
    }
    Ok(trimmed)
}

pub struct Application {
    paths: AppPaths,
    global: GlobalOptions,
    config_store: ConfigStore,
    state_store: StateStore,
    event_store: EventStore,
}

impl Application {
    pub fn new(paths: AppPaths, global: GlobalOptions) -> AppResult<Self> {
        Ok(Self {
            config_store: ConfigStore::new(paths.clone()),
            state_store: StateStore::new(paths.clone()),
            event_store: EventStore::new(paths.clone()),
            paths,
            global,
        })
    }

    async fn enable(&self) -> AppResult<()> {
        require_privileges()?;

        let config = self.config_store.load()?;
        let mut state = self.state_store.load()?;

        if state.mode == ProtectionMode::Active && state.runtime_pid.is_some() {
            if process_alive(state.runtime_pid.unwrap_or_default()) {
                return self.print_response(
                    "enable",
                    "Protection is already active",
                    Some(state.mode.as_str()),
                    json!({"already_active": true}),
                );
            }
            state.mode = ProtectionMode::Degraded;
            self.state_store.save(&state)?;
        }

        let coordinator = Coordinator::new(self.paths.clone());
        let snapshot = coordinator.capture_snapshot()?;

        let child = Command::new(std::env::current_exe().into_diagnostic()?)
            .arg("__serve")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .into_diagnostic()
            .context("failed to launch sentinel background runtime")?;

        sleep(Duration::from_millis(400)).await;
        if !process_alive(child.id()) {
            return Err(miette!(AppError::RuntimeStartFailed));
        }

        if let Err(err) = coordinator.apply_local_dns(&snapshot, "127.0.0.1") {
            let _ = stop_process(child.id());
            let _ = coordinator.restore_snapshot(&snapshot);
            return Err(err);
        }

        state.activate(
            snapshot.id.clone(),
            child.id(),
            self.paths.runtime_addr()?,
            &config,
        );
        self.state_store.save(&state)?;
        self.event_store.append(OperationEvent::new(
            OperationEventKind::Activate,
            Severity::Info,
            "Protection enabled",
        ))?;

        self.print_response(
            "enable",
            "Protection is active",
            Some(state.mode.as_str()),
            json!({
                "snapshot_id": state.snapshot_id,
                "runtime_pid": state.runtime_pid,
                "rule_count": state.active_rule_count,
                "allow_count": state.active_exclusion_count
            }),
        )
    }

    async fn disable(&self) -> AppResult<()> {
        require_privileges()?;

        let mut state = self.state_store.load()?;
        if let Some(pid) = state.runtime_pid {
            stop_process(pid)?;
        }

        let recovery = RecoveryManager::new(self.paths.clone());
        let restored = recovery.restore_if_available(state.snapshot_id.clone())?;
        let config = self.config_store.load()?;
        state.deactivate(&config);
        self.state_store.save(&state)?;
        self.event_store.append(OperationEvent::new(
            OperationEventKind::Disable,
            Severity::Info,
            "Protection disabled",
        ))?;

        self.print_response(
            "disable",
            "Protection is inactive",
            Some(state.mode.as_str()),
            json!({"restored": restored}),
        )
    }

    async fn status(&self) -> AppResult<()> {
        let config = self.config_store.load()?;
        let mut state = self.state_store.load()?;

        if let Some(pid) = state.runtime_pid {
            if state.mode == ProtectionMode::Active && !process_alive(pid) {
                state.mode = ProtectionMode::Degraded;
                self.state_store.save(&state)?;
            }
        }

        let body = render_status(&state, &config, &self.global)?;
        self.emit("status", "Status loaded", Some(state.mode.as_str()), body)
    }

    async fn allow_add(&self, domain: &str) -> AppResult<()> {
        let normalized = normalize_domain(domain)?;
        let mut config = self.config_store.load()?;
        let changed = config.add_allow_rule(normalized.clone());
        self.config_store.save(&config)?;

        self.event_store.append(OperationEvent::new(
            OperationEventKind::Allow,
            Severity::Info,
            format!("Allow rule added for {normalized}"),
        ))?;

        self.print_response(
            "allow-add",
            if changed { "Allow rule applied" } else { "Allow rule already present" },
            None,
            json!({"domain": normalized, "changed": changed}),
        )
    }

    async fn allow_remove(&self, domain: &str) -> AppResult<()> {
        let normalized = normalize_domain(domain)?;
        let mut config = self.config_store.load()?;
        let changed = config.remove_allow_rule(&normalized);
        self.config_store.save(&config)?;

        self.event_store.append(OperationEvent::new(
            OperationEventKind::RemoveAllow,
            Severity::Info,
            format!("Allow rule removed for {normalized}"),
        ))?;

        self.print_response(
            "allow-remove",
            if changed { "Allow rule removed" } else { "Allow rule was not present" },
            None,
            json!({"domain": normalized, "changed": changed}),
        )
    }

    async fn rules_list(&self) -> AppResult<()> {
        let config = self.config_store.load()?;
        let body = render_rules(&built_in_rules(), config.user_rules(), &self.global)?;
        self.emit("rules-list", "Rules listed", None, body)
    }

    async fn recover(&self) -> AppResult<()> {
        require_privileges()?;

        let mut state = self.state_store.load()?;
        if let Some(pid) = state.runtime_pid {
            let _ = stop_process(pid);
        }

        let recovery = RecoveryManager::new(self.paths.clone());
        recovery.restore_latest_or_active(state.snapshot_id.clone())?;
        let config = self.config_store.load()?;
        state.deactivate(&config);
        self.state_store.save(&state)?;
        self.event_store.append(OperationEvent::new(
            OperationEventKind::Recover,
            Severity::Warning,
            "Recovery completed",
        ))?;

        self.print_response(
            "recover",
            "Recovery completed",
            Some(state.mode.as_str()),
            json!({"restored": true}),
        )
    }

    async fn events(&self, limit: usize) -> AppResult<()> {
        let events = self.event_store.read_recent(limit)?;
        let body = render_events(&events, &self.global)?;
        self.emit("events", "Recent events loaded", None, body)
    }

    fn print_response(
        &self,
        command: &str,
        message: &str,
        state: Option<&str>,
        details: Value,
    ) -> AppResult<()> {
        self.emit(
            command,
            message,
            state,
            if self.global.json { details } else { json!({"details": details}) },
        )
    }

    fn emit(
        &self,
        command: &str,
        message: &str,
        state: Option<&str>,
        details: Value,
    ) -> AppResult<()> {
        if self.global.json {
            let mut payload = json!({
                "ok": true,
                "command": command,
                "message": message,
            });
            if let Some(state) = state {
                payload["state"] = json!(state);
            }
            if let Some(map) = payload.as_object_mut() {
                if let Some(detail_map) = details.as_object() {
                    for (key, value) in detail_map {
                        map.insert(key.clone(), value.clone());
                    }
                }
            }
            println!("{}", serde_json::to_string_pretty(&payload).into_diagnostic()?);
            return Ok(());
        }

        if let Some(output) = details.get("rendered").and_then(Value::as_str) {
            println!("{output}");
        } else {
            println!("{message}");
            if let Some(state) = state {
                println!("State: {state}");
            }
        }
        Ok(())
    }
}

pub fn read_file_if_exists(path: &Path) -> AppResult<Option<String>> {
    if !path.exists() {
        return Ok(None);
    }
    std::fs::read_to_string(path).into_diagnostic().map(Some)
}
