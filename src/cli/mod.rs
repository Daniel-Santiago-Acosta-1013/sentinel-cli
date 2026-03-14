pub mod commands;
pub mod output;
pub mod styles;

pub use clap::Parser;

#[derive(Clone, Debug, Default, clap::Args)]
pub struct GlobalOptions {
    #[arg(long, global = true)]
    pub json: bool,
    #[arg(long, global = true)]
    pub verbose: bool,
    #[arg(long, global = true)]
    pub no_color: bool,
}
