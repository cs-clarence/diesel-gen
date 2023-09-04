use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
  #[command(subcommand)]
  pub command: CliSubcommand,

  #[command(flatten)]
  pub args: CliArgs,
}

#[derive(Subcommand)]
pub enum CliSubcommand {
  Model,
}

#[derive(Args)]
pub struct CliArgs {
  #[clap(short = 'c', long = "config", default_value = "./diesel.toml")]
  pub config: PathBuf,
}
