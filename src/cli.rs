use std::path::PathBuf;

use crate::build;
use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(version = build::PKG_VERSION)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: CliSubcommands,

  #[command(flatten)]
  pub args: CliArgs,
}

#[derive(Subcommand)]
pub enum CliSubcommands {
  Generate,
  Config {
    #[command(subcommand)]
    subcommand: ConfigSubcommands,
  },
  Completion {
    #[arg(required = true, short = 's', long)]
    shell: Shell,
  },
}

#[derive(Subcommand)]
pub enum ConfigSubcommands {
  Schema,
}

#[derive(Args)]
pub struct CliArgs {
  #[arg(short = 'c', long, default_value = "./diesel-gen.yaml")]
  pub config: PathBuf,
}
