#[path = "support/mod.rs"]
mod support;

#[path = "integration/end_to_end_cli.rs"]
mod end_to_end_cli;
#[path = "integration/install_existing.rs"]
mod install_existing;
#[path = "integration/install_fresh.rs"]
mod install_fresh;
#[path = "integration/interactive_activation.rs"]
mod interactive_activation;
#[path = "integration/recovery_flow.rs"]
mod recovery_flow;
#[path = "integration/safety_failures.rs"]
mod safety_failures;
