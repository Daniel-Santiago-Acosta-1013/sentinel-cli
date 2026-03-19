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
        InputEvent, copy,
        menu_state::MenuSession,
        navigation::{
            ConfirmationAction, DomainEditorMode, LogScope, MenuActionId, ResultTone,
            Route, default_route,
        },
        parse_script, renderer,
        terminal::TerminalSession,
    },
    control::{activation::ActivationController, recovery::RecoveryController},
    install::version,
    storage::{
        blocked_domains::BlockedDomainsStore,
        config::ConfigStore,
        events::EventStore,
        install::InstallStore,
        state::{ProtectionMode, RiskLevel, StateStore},
    },
};

pub type AppResult<T> = Result<T>;

#[derive(Debug, Clone)]
pub struct AppPaths {
    pub state_dir: PathBuf,
    pub snapshots_dir: PathBuf,
    pub blocklist_file: PathBuf,
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
            blocklist_file: root_dir.join("blocklist.txt"),
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

pub fn normalize_domain(input: &str) -> AppResult<String> {
    let domain = input.trim().trim_matches('.').to_ascii_lowercase();
    if domain.is_empty() {
        return Err(miette!("el dominio no puede estar vacio"));
    }
    if !domain.is_ascii() {
        return Err(miette!(
            "el dominio debe usar solo caracteres ASCII o punycode"
        ));
    }
    if domain.len() > 253 {
        return Err(miette!("el dominio excede la longitud maxima permitida"));
    }
    if domain.contains("://") || domain.contains('/') {
        return Err(miette!("introduce solo el nombre de dominio, sin protocolo ni rutas"));
    }
    if domain.chars().any(char::is_whitespace) {
        return Err(miette!("el dominio no puede contener espacios"));
    }
    if !domain.contains('.') {
        return Err(miette!("el dominio debe incluir al menos un punto"));
    }

    for label in domain.split('.') {
        if label.is_empty() {
            return Err(miette!(
                "el dominio no puede contener segmentos vacios entre puntos"
            ));
        }
        if label.len() > 63 {
            return Err(miette!("cada segmento del dominio debe tener maximo 63 caracteres"));
        }
        if label.starts_with('-') || label.ends_with('-') {
            return Err(miette!(
                "cada segmento del dominio debe empezar y terminar con un caracter valido"
            ));
        }
        if !label
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-')
        {
            return Err(miette!(
                "el dominio solo admite letras, numeros, guiones y puntos"
            ));
        }
    }

    Ok(domain)
}

pub struct SentinelApp {
    paths: AppPaths,
    config_store: ConfigStore,
    state_store: StateStore,
    event_store: EventStore,
    install_store: InstallStore,
}

impl SentinelApp {
    pub fn new(paths: AppPaths) -> AppResult<Self> {
        let blocked_domains = BlockedDomainsStore::new(paths.blocklist_file.clone());
        blocked_domains.ensure_seeded()?;

        let blocklist = BlocklistBundle::load_from_path(&paths.blocklist_file)?;
        let state_store = StateStore::new(paths.clone());
        let mut state = state_store.load()?;
        state.refresh_bundle(&blocklist);
        state_store.save(&state)?;

        Ok(Self {
            config_store: ConfigStore::new(paths.clone()),
            state_store,
            event_store: EventStore::new(paths.clone()),
            install_store: InstallStore::new(paths.clone()),
            paths,
        })
    }

    fn blocked_domains_store(&self) -> BlockedDomainsStore {
        BlockedDomainsStore::new(self.paths.blocklist_file.clone())
    }

    fn load_blocklist(&self) -> AppResult<BlocklistBundle> {
        self.blocked_domains_store().ensure_seeded()?;
        BlocklistBundle::load_from_path(&self.paths.blocklist_file)
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
            if let Some(progress) = self.progress_label_for_input(&session, &event) {
                transcript
                    .push(renderer::render_progress_preview(&session, 100, &progress));
            }

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
        let capabilities = terminal.capabilities();
        let _color_enabled = capabilities.color;
        let _unicode_enabled = capabilities.unicode;
        let mut session = self.load_session(false)?;
        let mut last_drawn_route = None;

        loop {
            let frame = renderer::render(&session, terminal.width());
            draw_frame(&mut terminal, &frame, session.route, &mut last_drawn_route)?;
            let input = terminal.read_input()?;

            if let Some(progress) = self.progress_label_for_input(&session, &input) {
                let progress_frame = renderer::render_progress_preview(
                    &session,
                    terminal.width(),
                    &progress,
                );
                draw_frame(
                    &mut terminal,
                    &progress_frame,
                    Route::Progress,
                    &mut last_drawn_route,
                )?;
            }

            let should_exit = self.handle_input(&mut session, input).await?;
            if should_exit {
                let frame = renderer::render(&session, terminal.width());
                draw_frame(&mut terminal, &frame, session.route, &mut last_drawn_route)?;
                break;
            }
        }

        Ok(())
    }

    fn load_session(&self, transcript_mode: bool) -> AppResult<MenuSession> {
        let state = self.state_store.load()?;
        let install = self.install_store.inspect_current()?;
        let recent_events = self.event_store.read_recent(20)?;
        let blocked_domains = self.blocked_domains_store().list()?;
        let block_activity = self.event_store.block_activity_since_activation()?;
        let blocklist = self.load_blocklist()?;
        Ok(MenuSession::from_runtime_state(
            state,
            install,
            recent_events,
            blocked_domains,
            block_activity,
            &blocklist,
            transcript_mode,
        ))
    }

    async fn handle_input(
        &self,
        session: &mut MenuSession,
        input: InputEvent,
    ) -> AppResult<bool> {
        match input {
            InputEvent::Up => session.select_previous(),
            InputEvent::Down => session.select_next(),
            InputEvent::Back => self.handle_back(session),
            InputEvent::Exit => return self.exit_session(session),
            InputEvent::Backspace => {
                if matches!(session.route, Route::BlockedDomainEditor(_)) {
                    session.pop_domain_input();
                }
            }
            InputEvent::InsertChar(ch) => {
                if matches!(session.route, Route::BlockedDomainEditor(_)) {
                    session.append_domain_input(&ch.to_string());
                }
            }
            InputEvent::InsertText(text) => {
                if matches!(session.route, Route::BlockedDomainEditor(_)) {
                    session.replace_domain_input(text);
                }
            }
            InputEvent::Confirm => {
                if let Route::BlockedDomainEditor(mode) = session.route {
                    return self.persist_domain_editor(session, mode).await;
                }

                if let Some(action) = session.selected_action_id() {
                    return self.execute_action(session, action).await;
                }
            }
        }

        Ok(false)
    }

    fn handle_back(&self, session: &mut MenuSession) {
        match session.route {
            Route::Home => {}
            Route::Settings => {
                session.route = Route::Home;
                session.selected_index = 0;
                session.last_message =
                    "Volviste al inicio para revisar otra accion principal.".to_owned();
            }
            Route::BlockedDomains => {
                session.route = Route::Settings;
                session.selected_index = 0;
                session.last_message =
                    "Volviste a Ajustes sin perder el catalogo vigente.".to_owned();
            }
            Route::BlockedDomainEditor(_) => {
                session.route = Route::BlockedDomains;
                session.selected_index = 0;
                session.clear_domain_editor();
                session.last_message =
                    "La edicion fue cancelada antes de guardar cambios.".to_owned();
            }
            Route::Recovery => {
                session.route = Route::Home;
                session.selected_index = 0;
                session.last_message =
                    "Volviste al inicio para elegir otra accion con una pantalla limpia."
                        .to_owned();
            }
            Route::Confirm(_) => {
                session.route = back_route_for(session.runtime_state.mode);
                session.selected_index = 0;
                session.last_message =
                    "La accion sensible fue cancelada antes de cambiar la red."
                        .to_owned();
            }
            Route::Logs(scope) => {
                session.route = log_parent_route(scope);
                session.selected_index = 0;
                session.last_message =
                    "Volviste a la vista anterior sin perder el contexto.".to_owned();
            }
            Route::Result | Route::Safety | Route::Status => {
                session.route = Route::Home;
                session.selected_index = 0;
                session.last_result = None;
                session.last_message =
                    "Volviste al inicio para ejecutar otra accion.".to_owned();
            }
            Route::Progress | Route::Exit => {}
        }
    }

    fn exit_session(&self, session: &mut MenuSession) -> AppResult<bool> {
        session.route = Route::Exit;
        session.selected_index = 0;
        session.last_result = None;
        session.last_message.clear();
        Ok(true)
    }

    async fn execute_action(
        &self,
        session: &mut MenuSession,
        action: MenuActionId,
    ) -> AppResult<bool> {
        match action {
            MenuActionId::ToggleProtection => {
                session.route = Route::Confirm(session.toggle_confirmation_action());
                session.selected_index = 0;
                session.last_message =
                    "Confirma la accion solo si deseas aplicar el cambio de red."
                        .to_owned();
                Ok(false)
            }
            MenuActionId::ViewStatus => {
                self.refresh_status(session)?;
                Ok(false)
            }
            MenuActionId::OpenSettings => {
                self.refresh_blocking_views(session)?;
                session.route = Route::Settings;
                session.selected_index = 0;
                session.last_message =
                    "Ajustes centraliza la administracion del catalogo activo.".to_owned();
                Ok(false)
            }
            MenuActionId::ViewBlockedDomains => {
                self.refresh_blocking_views(session)?;
                session.route = Route::BlockedDomains;
                session.selected_index = 0;
                session.last_message = copy::blocked_domain_selection(
                    session.selected_blocked_domain(),
                );
                Ok(false)
            }
            MenuActionId::AddBlockedDomain => {
                session.start_domain_editor(DomainEditorMode::Add, None);
                session.last_message =
                    "Introduce un dominio valido y confirma para guardarlo.".to_owned();
                Ok(false)
            }
            MenuActionId::EditBlockedDomain => {
                let Some(domain) = session.selected_blocked_domain().map(str::to_owned)
                else {
                    session.last_message =
                        "No hay un dominio seleccionado para editar.".to_owned();
                    return Ok(false);
                };
                session.start_domain_editor(DomainEditorMode::Edit, Some(domain.clone()));
                session.last_message =
                    format!("Editando `{domain}`. Confirma cuando termines.");
                Ok(false)
            }
            MenuActionId::DeleteBlockedDomain => {
                let Some(domain) = session.selected_blocked_domain().map(str::to_owned)
                else {
                    session.last_message =
                        "No hay un dominio seleccionado para eliminar.".to_owned();
                    return Ok(false);
                };

                match self.blocked_domains_store().remove(&domain) {
                    Ok(updated) => {
                        session.sync_blocked_domains(updated);
                        self.persist_runtime_bundle(session)?;
                        session.route = Route::BlockedDomains;
                        session.selected_index = 0;
                        session.last_message = copy::blocked_domain_deleted(&domain);
                    }
                    Err(err) => session.last_message = err.to_string(),
                }
                Ok(false)
            }
            MenuActionId::SelectNextBlockedDomain => {
                session.select_next_domain();
                session.last_message = copy::blocked_domain_selection(
                    session.selected_blocked_domain(),
                );
                Ok(false)
            }
            MenuActionId::SelectPreviousBlockedDomain => {
                session.select_previous_domain();
                session.last_message = copy::blocked_domain_selection(
                    session.selected_blocked_domain(),
                );
                Ok(false)
            }
            MenuActionId::ViewLogs => {
                if let Some(scope) = session.log_scope() {
                    session.route = Route::Logs(scope);
                    session.selected_index = 0;
                    session.last_message =
                        "Revisa los logs y vuelve cuando termines.".to_owned();
                }
                Ok(false)
            }
            MenuActionId::RecoverNetwork => {
                session.route = Route::Confirm(ConfirmationAction::RecoverNetwork);
                session.selected_index = 0;
                session.last_message =
                    "Sentinel puede restaurar la red y validar el resultado antes de volver al inicio."
                        .to_owned();
                Ok(false)
            }
            MenuActionId::BackToPrevious => {
                if let Some(scope) = session.log_scope() {
                    session.route = log_parent_route(scope);
                    session.selected_index = 0;
                    session.last_message =
                        "Volviste a la vista anterior sin perder el contexto.".to_owned();
                }
                Ok(false)
            }
            MenuActionId::BackSettings => {
                session.route = Route::Settings;
                session.selected_index = 0;
                session.last_message =
                    "Volviste a Ajustes sin perder el catalogo actual.".to_owned();
                Ok(false)
            }
            MenuActionId::BackHome => {
                session.route = Route::Home;
                session.selected_index = 0;
                session.last_result = None;
                session.last_message =
                    "Volviste al inicio para revisar otra vista o accion.".to_owned();
                Ok(false)
            }
            MenuActionId::Exit => self.exit_session(session),
            MenuActionId::Confirm => {
                let pending = match session.route {
                    Route::Confirm(action) => action,
                    _ => return Ok(false),
                };
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
            MenuActionId::Cancel => {
                session.route = back_route_for(session.runtime_state.mode);
                session.selected_index = 0;
                session.last_message =
                    "La accion sensible fue cancelada antes de cambiar la red."
                        .to_owned();
                Ok(false)
            }
        }
    }

    fn progress_label_for_input(
        &self,
        session: &MenuSession,
        input: &InputEvent,
    ) -> Option<String> {
        if *input != InputEvent::Confirm {
            return None;
        }

        match session.route {
            Route::Home => None,
            Route::Confirm(ConfirmationAction::EnableProtection) => {
                Some("Activando Sentinel y preparando snapshot recuperable...".to_owned())
            }
            Route::Confirm(ConfirmationAction::DisableProtection) => {
                Some("Desactivando Sentinel y restaurando la red...".to_owned())
            }
            Route::Confirm(ConfirmationAction::RecoverNetwork) => {
                Some("Recuperando la red y verificando el resultado...".to_owned())
            }
            _ => None,
        }
    }

    async fn enable_protection(&self, session: &mut MenuSession) -> AppResult<()> {
        let blocklist = self.load_blocklist()?;
        let controller = ActivationController::new(
            &self.paths,
            &self.config_store,
            &self.state_store,
            &self.event_store,
            &blocklist,
        );
        let next_state = controller.enable().await?;
        session.sync_runtime_state(next_state);
        self.refresh_recent_events(session)?;
        session.sync_block_activity(self.event_store.block_activity_since_activation()?);

        if session.runtime_state.mode == ProtectionMode::Active {
            session.show_result(
                "Sentinel activado",
                "Sentinel quedo activo con un resultado visible y una ruta clara para continuar.",
                "Pulsa Enter para volver al inicio o baja a Salir.",
                ResultTone::Success,
            );
        } else {
            session.show_result(
                "Activacion con advertencias",
                session
                    .runtime_state
                    .last_message
                    .clone()
                    .unwrap_or_else(|| session.status_summary.clone()),
                "Revisa el estado o ejecuta recuperacion desde el inicio antes de seguir.",
                ResultTone::Warning,
            );
        }
        Ok(())
    }

    async fn disable_protection(&self, session: &mut MenuSession) -> AppResult<()> {
        let blocklist = self.load_blocklist()?;
        let controller = ActivationController::new(
            &self.paths,
            &self.config_store,
            &self.state_store,
            &self.event_store,
            &blocklist,
        );
        let next_state = controller.disable().await?;
        session.sync_runtime_state(next_state);
        self.refresh_recent_events(session)?;
        session.sync_block_activity(self.event_store.block_activity_since_activation()?);

        let tone = if matches!(
            session.runtime_state.mode,
            ProtectionMode::Degraded | ProtectionMode::Recovering
        ) {
            ResultTone::Warning
        } else {
            ResultTone::Success
        };
        let title = if tone == ResultTone::Success {
            "Sentinel desactivado"
        } else {
            "Desactivacion con advertencias"
        };
        session.show_result(
            title,
            session
                .runtime_state
                .last_message
                .clone()
                .unwrap_or_else(|| session.status_summary.clone()),
            "Vuelve al inicio para revisar el estado o salir con una pantalla limpia.",
            tone,
        );
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
        self.refresh_recent_events(session)?;
        session.sync_block_activity(self.event_store.block_activity_since_activation()?);

        let (title, tone) = match session.runtime_state.last_verification_result.as_ref()
        {
            Some(verification) if verification.matches_snapshot => {
                ("Recuperacion completada", ResultTone::Success)
            }
            Some(_) => ("Recuperacion con advertencias", ResultTone::Warning),
            None if session.runtime_state.risk_level == RiskLevel::Critical => {
                ("Recuperacion con advertencias", ResultTone::Warning)
            }
            None => ("Recuperacion completada", ResultTone::Success),
        };

        session.show_result(
            title,
            session.runtime_state.last_message.clone().unwrap_or_else(|| {
                "La recuperacion termino y Sentinel verifico el estado de red.".to_owned()
            }),
            "Pulsa Enter para volver al inicio o revisa el estado antes de salir.",
            tone,
        );
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

        let blocklist = self.load_blocklist()?;
        state.refresh_bundle(&blocklist);
        self.state_store.save(&state)?;

        let install = self.install_store.inspect_current()?;
        session.sync_runtime_state(state);
        session.install_state = install;
        self.refresh_recent_events(session)?;
        self.refresh_blocking_views(session)?;
        session.route = Route::Status;
        session.selected_index = 0;
        session.last_result = None;
        session.last_message =
            "Estado actualizado desde el runtime persistido, seguridad e instalacion."
                .to_owned();
        Ok(())
    }

    fn refresh_recent_events(&self, session: &mut MenuSession) -> AppResult<()> {
        session.sync_recent_events(self.event_store.read_recent(20)?);
        Ok(())
    }

    fn refresh_blocking_views(&self, session: &mut MenuSession) -> AppResult<()> {
        session.sync_blocked_domains(self.blocked_domains_store().list()?);
        session.sync_block_activity(self.event_store.block_activity_since_activation()?);
        self.persist_runtime_bundle(session)?;
        Ok(())
    }

    fn persist_runtime_bundle(&self, session: &mut MenuSession) -> AppResult<()> {
        let blocklist = self.load_blocklist()?;
        session.runtime_state.refresh_bundle(&blocklist);
        self.state_store.save(&session.runtime_state)?;
        Ok(())
    }

    async fn persist_domain_editor(
        &self,
        session: &mut MenuSession,
        mode: DomainEditorMode,
    ) -> AppResult<bool> {
        let result = match mode {
            DomainEditorMode::Add => self
                .blocked_domains_store()
                .add(&session.domain_input)
                .map(|domains| (domains, false, session.domain_input.clone())),
            DomainEditorMode::Edit => {
                let Some(original) = session.domain_original.as_deref() else {
                    session.last_message =
                        "No hay un dominio base para aplicar la edicion.".to_owned();
                    return Ok(false);
                };
                self.blocked_domains_store()
                    .update(original, &session.domain_input)
                    .map(|domains| (domains, true, session.domain_input.clone()))
            }
        };

        match result {
            Ok((domains, edited, domain)) => {
                let normalized = normalize_domain(&domain)?;
                session.sync_blocked_domains(domains);
                if let Some(index) = session
                    .blocked_domains
                    .iter()
                    .position(|item| item == &normalized)
                {
                    session.selected_domain_index = index;
                }
                self.persist_runtime_bundle(session)?;
                session.route = Route::BlockedDomains;
                session.selected_index = 0;
                session.clear_domain_editor();
                session.last_message = copy::blocked_domain_saved(&normalized, edited);
            }
            Err(err) => session.last_message = err.to_string(),
        }

        Ok(false)
    }
}

fn draw_frame(
    terminal: &mut TerminalSession,
    frame: &str,
    route: Route,
    last_drawn_route: &mut Option<Route>,
) -> AppResult<()> {
    if last_drawn_route.is_some_and(|previous| previous == route) {
        terminal.redraw(frame)?;
    } else {
        terminal.draw(frame)?;
    }
    *last_drawn_route = Some(route);
    Ok(())
}

fn back_route_for(mode: ProtectionMode) -> Route {
    if matches!(mode, ProtectionMode::Degraded | ProtectionMode::Recovering) {
        Route::Recovery
    } else {
        default_route(mode)
    }
}

fn log_parent_route(scope: LogScope) -> Route {
    match scope {
        LogScope::Safety => Route::Safety,
        LogScope::Status => Route::Status,
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
