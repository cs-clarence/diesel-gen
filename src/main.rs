use std::{
  fs::{self, OpenOptions},
  io::Write,
  path::PathBuf,
  process::{Command, Stdio},
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
    cli::CliSubcommand::Models => {
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

      let parsed = parse::file(&mut ParseContext::new(&content));

      match parsed {
        Ok(parsed) => {
          let model = diesel_gen_config
            .models
            .as_ref()
            .cloned()
            .unwrap_or_default();

          let output_path = model
            .output
            .as_ref()
            .cloned()
            .unwrap_or(String::from_str("./models.rs")?);

          let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&output_path)?;

          let mut buff: Vec<u8> = Vec::new();

          generate::models(&parsed, &diesel_config, &model, &mut buff)?;

          let out = Command::new("which")
            .arg("rustfmt")
            .output()
            .expect("failed to execute process 'which'");

          if out.status.success() {
            let mut out = Command::new("rustfmt")
              .stdin(Stdio::piped())
              .stdout(file)
              .spawn()
              .expect("failed to execute process 'rustfmt'");

            let mut stdin = out.stdin.as_ref().unwrap();

            stdin.write_all(&buff)?;
            stdin.flush()?;
            out.wait()?;
          } else {
            file.write_all(&buff)?;
          }
          println!("Generated {} file successfully!", &output_path);
        }
        Err(err) => {
          eprintln!("Error: {}", err);
        }
      }
    }
  }

  Ok(())
}
