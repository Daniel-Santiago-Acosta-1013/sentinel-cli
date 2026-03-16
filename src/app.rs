use std::{
    env,
    io::{self},
    net::SocketAddr,
    path::{Path, PathBuf},
    process::Command,
};

use directories::ProjectDirs;
use miette::{Context, IntoDiagnostic, Result, miette};
use tracing_subscriber::EnvFilter;

use crate::{
    blocking::{blocklist::BlocklistBundle, runtime},
    cli::{
        InputEvent,
        menu_state::{ActionId, ConfirmationAction, MenuSession, ViewId},
        parse_script, renderer,
        terminal::TerminalSession,
    },
    control::{
        activation::ActivationController, recovery::RecoveryController,
        safety::SafetyController,
    },
    install::version,
    storage::{
        config::ConfigStore,
        events::EventStore,
        install::InstallStore,
        state::{ProtectionMode, StateStore},
    },
};

pub type AppResult<T> = Result<T>;

#[derive(Debug, Clone)]
pub struct AppPaths {
    pub state_dir: PathBuf,
    pub snapshots_dir: PathBuf,
    pub config_file: PathBuf,
    pub state_file: PathBuf,
    pub events_file: PathBuf,
    pub install_file: PathBuf,
}

impl AppPaths {
    pub fn discover() -> AppResult<Self> {
        let root_dir = if let Ok(custom_root) = env::var("SENTINEL_HOME") {
            PathBuf::from(custom_root)
        } else {
            let project_dirs = ProjectDirs::from("com", "sentinel", "sentinel")
                .ok_or_else(|| {
                    miette!(
                        "no se pudo determinar el directorio de soporte de la aplicacion"
                    )
                })?;
            project_dirs.data_dir().to_path_buf()
        };

        let config_dir = root_dir.join("config");
        let data_dir = root_dir.join("data");
        let state_dir = root_dir.join("state");
        let snapshots_dir = state_dir.join("snapshots");
        std::fs::create_dir_all(&config_dir).into_diagnostic()?;
        std::fs::create_dir_all(&data_dir).into_diagnostic()?;
        std::fs::create_dir_all(&state_dir).into_diagnostic()?;
        std::fs::create_dir_all(&snapshots_dir).into_diagnostic()?;

        Ok(Self {
            state_dir: state_dir.clone(),
            snapshots_dir,
            config_file: config_dir.join("config.toml"),
            state_file: state_dir.join("state.json"),
            events_file: state_dir.join("events.jsonl"),
            install_file: data_dir.join("install.json"),
        })
    }

    pub fn runtime_addr(&self) -> AppResult<SocketAddr> {
        let port = env::var("SENTINEL_DNS_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(53);
        let bind_ip =
            env::var("SENTINEL_DNS_BIND").unwrap_or_else(|_| "127.0.0.1".to_owned());
        format!("{bind_ip}:{port}").parse().into_diagnostic()
    }
}

pub async fn run() -> AppResult<()> {
    init_tracing()?;
    let paths = AppPaths::discover()?;

    match env::var("SENTINEL_INTERNAL_MODE").ok().as_deref() {
        Some("runtime") => runtime::run_runtime(paths).await,
        Some("print-version") => {
            println!("{}", version::current_version());
            Ok(())
        }
        _ => SentinelApp::new(paths)?.run().await,
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

pub fn read_file_if_exists(path: &Path) -> AppResult<Option<String>> {
    if !path.exists() {
        return Ok(None);
    }
    std::fs::read_to_string(path).into_diagnostic().map(Some)
}

pub struct SentinelApp {
    paths: AppPaths,
    blocklist: BlocklistBundle,
    config_store: ConfigStore,
    state_store: StateStore,
    event_store: EventStore,
    install_store: InstallStore,
}

impl SentinelApp {
    pub fn new(paths: AppPaths) -> AppResult<Self> {
        let blocklist = BlocklistBundle::load()?;
        Ok(Self {
            config_store: ConfigStore::new(paths.clone()),
            state_store: StateStore::new(paths.clone()),
            event_store: EventStore::new(paths.clone()),
            install_store: InstallStore::new(paths.clone()),
            paths,
            blocklist,
        })
    }

    async fn run(self) -> AppResult<()> {
        if let Ok(script) = env::var("SENTINEL_SCRIPT") {
            return self.run_scripted(script).await;
        }

        if !std::io::IsTerminal::is_terminal(&io::stdout()) {
            return Err(miette!(
                "Sentinel requiere una terminal interactiva. Vuelve a ejecutarlo en un TTY."
            ));
        }

        self.run_terminal().await
    }

    async fn run_scripted(self, script: String) -> AppResult<()> {
        let mut session = self.load_session(true)?;
        let mut transcript = vec![renderer::render_snapshot(&session)];
        for event in parse_script(&script)? {
            let should_exit = self.handle_input(&mut session, event).await?;
            transcript.push(renderer::render_snapshot(&session));
            if should_exit {
                break;
            }
        }

        println!("{}", transcript.join("\n\n---\n\n"));
        Ok(())
    }

    async fn run_terminal(self) -> AppResult<()> {
        let mut terminal = TerminalSession::start()?;
        let mut session = self.load_session(false)?;

        loop {
            let frame = renderer::render(&session, terminal.width());
            terminal.draw(&frame)?;
            let input = terminal.read_input()?;
            let should_exit = self.handle_input(&mut session, input).await?;
            if should_exit {
                let frame = renderer::render(&session, terminal.width());
                terminal.draw(&frame)?;
                break;
            }
        }

        Ok(())
    }

    fn load_session(&self, transcript_mode: bool) -> AppResult<MenuSession> {
        let state = self.state_store.load()?;
        let install = self.install_store.inspect_current()?;
        Ok(MenuSession::from_runtime_state(
            state,
            install,
            &self.blocklist,
            transcript_mode,
        ))
    }

    async fn handle_input(
        &self,
        session: &mut MenuSession,
        input: InputEvent,
    ) -> AppResult<bool> {
        if let Some(pending) = session.pending_confirmation {
            return self.handle_confirmation(session, pending, input).await;
        }

        match input {
            InputEvent::Confirm => {
                let action = session.selected_action_id();
                match action {
                    ActionId::RunSafetyChecks => self.run_safety_checks(session)?,
                    ActionId::ToggleProtection => {
                        session.pending_confirmation =
                            Some(session.toggle_confirmation_action());
                        session.view = ViewId::Confirmacion;
                    }
                    ActionId::ViewStatus => self.refresh_status(session)?,
                    ActionId::ViewInstallState => self.show_install_state(session)?,
                    ActionId::RecoverNetwork => {
                        session.pending_confirmation =
                            Some(ConfirmationAction::RecoverNetwork);
                        session.view = ViewId::Confirmacion;
                    }
                    ActionId::Exit => {
                        session.view = ViewId::Salida;
                        session.last_message =
                            "Sesion cerrada sin acumular texto ni dejar cambios pendientes."
                                .to_owned();
                        return Ok(true);
                    }
                }
            }
            InputEvent::Up => session.select_previous(),
            InputEvent::Down => session.select_next(),
            InputEvent::Back => {
                session.view = if matches!(
                    session.runtime_state.mode,
                    ProtectionMode::Degraded | ProtectionMode::Recovering
                ) {
                    ViewId::Recuperacion
                } else {
                    ViewId::Inicio
                };
            }
            InputEvent::Exit => {
                session.view = ViewId::Salida;
                session.last_message =
                    "Sesion cerrada sin acumular texto ni dejar cambios pendientes."
                        .to_owned();
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn handle_confirmation(
        &self,
        session: &mut MenuSession,
        pending: ConfirmationAction,
        input: InputEvent,
    ) -> AppResult<bool> {
        match input {
            InputEvent::Back | InputEvent::Exit => {
                session.pending_confirmation = None;
                session.view = if matches!(
                    session.runtime_state.mode,
                    ProtectionMode::Degraded | ProtectionMode::Recovering
                ) {
                    ViewId::Recuperacion
                } else {
                    ViewId::Inicio
                };
                session.last_message =
                    "La accion sensible fue cancelada antes de cambiar la red."
                        .to_owned();
                Ok(matches!(input, InputEvent::Exit))
            }
            InputEvent::Confirm => {
                session.pending_confirmation = None;
                match pending {
                    ConfirmationAction::EnableProtection => {
                        self.enable_protection(session).await?
                    }
                    ConfirmationAction::DisableProtection => {
                        self.disable_protection(session).await?
                    }
                    ConfirmationAction::RecoverNetwork => {
                        self.recover_network(session).await?
                    }
                }
                Ok(false)
            }
            InputEvent::Up | InputEvent::Down => Ok(false),
        }
    }

    fn run_safety_checks(&self, session: &mut MenuSession) -> AppResult<()> {
        let state = self.state_store.load()?;
        let summary =
            SafetyController::new(&self.paths, &self.blocklist).run_checks(&state)?;
        let mut next_state = state;
        next_state.last_safety_check = Some(summary.clone());
        next_state.risk_level = summary.risk_level();
        next_state.status_summary = summary.recommended_action.clone();
        next_state.last_message = Some(if summary.issues.is_empty() {
            "Los chequeos terminaron sin alertas bloqueantes.".to_owned()
        } else {
            summary.issues.join(" | ")
        });
        next_state.refresh_bundle(&self.blocklist);
        self.state_store.save(&next_state)?;
        self.event_store.record_safety(&summary)?;
        session.sync_runtime_state(next_state);
        session.view = ViewId::Seguridad;
        session.last_message = session
            .runtime_state
            .last_message
            .clone()
            .unwrap_or_else(|| session.status_summary.clone());
        Ok(())
    }

    async fn enable_protection(&self, session: &mut MenuSession) -> AppResult<()> {
        let controller = ActivationController::new(
            &self.paths,
            &self.config_store,
            &self.state_store,
            &self.event_store,
            &self.blocklist,
        );
        let next_state = controller.enable().await?;
        session.sync_runtime_state(next_state);
        if session.runtime_state.mode == ProtectionMode::Active {
            session.view = ViewId::Estado;
            session.last_message =
                "Proteccion activada con snapshot recuperable y runtime DNS local."
                    .to_owned();
        } else {
            session.view = ViewId::Seguridad;
            session.last_message = session
                .runtime_state
                .last_message
                .clone()
                .unwrap_or_else(|| session.status_summary.clone());
        }
        Ok(())
    }

    async fn disable_protection(&self, session: &mut MenuSession) -> AppResult<()> {
        let controller = ActivationController::new(
            &self.paths,
            &self.config_store,
            &self.state_store,
            &self.event_store,
            &self.blocklist,
        );
        let next_state = controller.disable().await?;
        session.sync_runtime_state(next_state);
        session.view = if matches!(
            session.runtime_state.mode,
            ProtectionMode::Degraded | ProtectionMode::Recovering
        ) {
            ViewId::Recuperacion
        } else {
            ViewId::Estado
        };
        session.last_message = session
            .runtime_state
            .last_message
            .clone()
            .unwrap_or_else(|| session.status_summary.clone());
        Ok(())
    }

    async fn recover_network(&self, session: &mut MenuSession) -> AppResult<()> {
        let controller = RecoveryController::new(
            &self.paths,
            &self.config_store,
            &self.state_store,
            &self.event_store,
        );
        let next_state = controller.recover().await?;
        session.sync_runtime_state(next_state);
        session.view = ViewId::Recuperacion;
        session.last_message =
            session.runtime_state.last_message.clone().unwrap_or_else(|| {
                "La recuperacion termino y Sentinel verifico el estado de red.".to_owned()
            });
        Ok(())
    }

    fn refresh_status(&self, session: &mut MenuSession) -> AppResult<()> {
        let mut state = self.state_store.load()?;
        if let Some(pid) = state.runtime_pid
            && state.mode == ProtectionMode::Active
            && !runtime::process_alive(pid)
        {
            state.mode = ProtectionMode::Degraded;
            state.risk_level = crate::storage::state::RiskLevel::Critical;
            state.status_summary =
                "El runtime local ya no esta sano. Se recomienda recuperar la red."
                    .to_owned();
            state.last_message =
                Some(
                    "Sentinel detecto que el runtime en segundo plano desaparecio. Se recomienda recuperar la red."
                        .to_owned(),
                );
        }
        state.refresh_bundle(&self.blocklist);
        self.state_store.save(&state)?;

        let install = self.install_store.inspect_current()?;
        session.sync_runtime_state(state);
        session.install_state = install;
        session.view = ViewId::Estado;
        session.last_message =
            "Estado actualizado desde el runtime persistido, seguridad e instalacion."
                .to_owned();
        Ok(())
    }

    fn show_install_state(&self, session: &mut MenuSession) -> AppResult<()> {
        let install = self.install_store.inspect_current()?;
        session.install_state = install;
        session.view = ViewId::Instalacion;
        session.last_message =
            "Estado de instalacion cargado. Usa el script oficial para instalar o actualizar."
                .to_owned();
        Ok(())
    }
}

pub fn require_privileges() -> AppResult<()> {
    if env::var("SENTINEL_FAKE_PLATFORM").ok().as_deref() == Some("1") {
        return Ok(());
    }

    let output = Command::new("id")
        .arg("-u")
        .output()
        .into_diagnostic()
        .context("no se pudieron determinar los privilegios actuales")?;
    if String::from_utf8_lossy(&output.stdout).trim() == "0" {
        Ok(())
    } else {
        Err(miette!(
            "Sentinel necesita privilegios elevados para cambiar el DNS del sistema"
        ))
    }
}
