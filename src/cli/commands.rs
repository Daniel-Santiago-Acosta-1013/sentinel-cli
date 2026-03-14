use clap::{Parser, Subcommand};

use crate::cli::GlobalOptions;

#[derive(Debug, Parser)]
#[command(name = "sentinel", version, about = "Protect system DNS from ad domains")]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOptions,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Enable,
    Disable,
    Status,
    Allow {
        #[command(subcommand)]
        command: AllowCommand,
    },
    Rules {
        #[command(subcommand)]
        command: RulesCommand,
    },
    Recover,
    Events {
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    #[command(name = "__serve", hide = true)]
    Serve,
}

#[derive(Debug, Subcommand)]
pub enum AllowCommand {
    Add { domain: String },
    Remove { domain: String },
}

#[derive(Debug, Subcommand)]
pub enum RulesCommand {
    List,
}
