use std::{
  fs::{self, OpenOptions},
  path::PathBuf,
  str::FromStr,
};

use clap::Parser;
use config::{DieselConfig, DieselGenConfig};
use parse::ParseContext;

mod cli;
mod config;
mod generate;
mod parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = cli::Cli::parse();

  let diesel_config_content = fs::read_to_string(args.args.diesel_config)?;
  let diesel_gen_config_content =
    fs::read_to_string(args.args.diesel_gen_config)?;

  let diesel_config = toml::from_str::<DieselConfig>(&diesel_config_content)?;
  let diesel_gen_config =
    toml::from_str::<DieselGenConfig>(&diesel_gen_config_content)?;

  match args.command {
    cli::CliSubcommand::Model => {
      let content = fs::read_to_string(
        diesel_config
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
          let model = diesel_gen_config
            .model
            .as_ref()
            .cloned()
            .unwrap_or_default();

          let output_path = model
            .output
            .as_ref()
            .cloned()
            .unwrap_or(String::from_str("./models.rs")?);

          let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_path)?;

          generate::generate_models(&parsed, &diesel_config, &model, &file)?;
        }
        Err(err) => {
          println!("Error: {}", err);
        }
      }
    }
  }

  Ok(())
}
