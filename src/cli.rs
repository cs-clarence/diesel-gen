use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
  #[clap(subcommand)]
  pub command: CliSubcommand,

  #[clap(flatten)]
  pub args: CliArgs,
}

#[derive(Subcommand)]
pub enum CliSubcommand {
  Model,
}

#[derive(Args)]
pub struct CliArgs {
  #[clap(short = 'c', long = "config", default_value = "diesel.toml")]
  config: PathBuf,
}
