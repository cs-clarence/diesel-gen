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
  Generate,
}

#[derive(Args)]
pub struct CliArgs {
  #[clap(short = 'c', long, default_value = "./diesel-gen.yaml")]
  pub config: PathBuf,
}
