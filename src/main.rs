use std::{
  fs::{self, OpenOptions},
  path::PathBuf,
  str::FromStr,
};

use clap::Parser;
use config::Config;
use parse::ParseContext;

mod cli;
mod config;
mod generate;
mod parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = cli::Cli::parse();

  let content = fs::read_to_string(args.args.config)?;

  let config = toml::from_str::<Config>(&content)?;

  match args.command {
    cli::CliSubcommand::Model => {
      let content = fs::read_to_string(
        config
          .print_schema
          .as_ref()
          .cloned()
          .unwrap_or_default()
          .file
          .clone()
          .unwrap_or(PathBuf::from_str("./schema.rs")?),
      )?;

      let parsed = parse::parse_file(&mut ParseContext::new(&content));

      match parsed {
        Ok(parsed) => {
          let output_path = config
            .generate
            .as_ref()
            .cloned()
            .unwrap_or_default()
            .model
            .as_ref()
            .cloned()
            .unwrap_or_default()
            .output
            .unwrap_or(String::from_str("./models.rs")?);

          let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_path)?;

          generate::generate_models(&parsed, &config, &file)?;
        }
        Err(err) => {
          println!("Error: {}", err);
        }
      }
    }
  }

  Ok(())
}
