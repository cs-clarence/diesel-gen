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

use crate::write::ModelsArgs;

mod cli;
mod config;
mod parse;
mod util;
mod write;

fn generate_models(
  diesel_config: &DieselConfig,
  diesel_gen_config: &DieselGenConfig,
) -> anyhow::Result<()> {
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

  let parsed = parse::file(&mut ParseContext::new(&content))?;

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
  write::rust_file_headers(&mut buff)?;

  let ref_type_overrides = util::remove_spaces_from_keys(
    &model.ref_type_override.clone().unwrap_or_default(),
  );

  let type_overrides = util::remove_spaces_from_keys(
    &model.type_override.clone().unwrap_or_default(),
  );

  let binding = model.mods.clone().unwrap_or_default();
  let mods = binding.iter().map(|e| e.as_str()).collect::<Vec<&str>>();

  for m in mods.iter() {
    writeln!(buff, "mod {};", m)?;
  }

  let binding = model.pub_mods.clone().unwrap_or_default();
  let pub_mods = binding.iter().map(|e| e.as_str()).collect::<Vec<&str>>();

  for m in pub_mods.iter() {
    writeln!(buff, "pub mod {};", m)?;
  }

  let binding = model.uses.clone().unwrap_or_default();
  let uses = binding.iter().map(|e| e.as_str()).collect::<Vec<&str>>();
  for u in uses.iter() {
    writeln!(buff, "use {};", u)?;
  }

  let binding = model.pub_uses.clone().unwrap_or_default();
  let pub_uses = binding.iter().map(|e| e.as_str()).collect::<Vec<&str>>();
  for u in pub_uses.iter() {
    writeln!(buff, "pub use {};", u)?;
  }

  let table_imports_root =
    model.table_imports_root.clone().unwrap_or_else(|| {
      util::import_root_from_path(
        &parsed,
        diesel_config
          .print_schema
          .clone()
          .unwrap_or_default()
          .file
          .clone()
          .unwrap_or_default()
          .as_os_str()
          .to_str()
          .unwrap(),
      )
    });

  write::table_uses(
    &table_imports_root,
    &parsed.tables,
    &model.tables.clone().unwrap_or_default(),
    &mut buff,
  )?;

  write::tables_type_uses(
    &parsed.tables,
    &model.type_use.clone().unwrap_or_default(),
    &type_overrides,
    &mut buff,
  )?;

  write::models(
    &ModelsArgs {
      file: &parsed,
      backend: &model.backend,
      ref_type_overrides: &ref_type_overrides,
      type_overrides: &type_overrides,
      table_configs: &model.tables.clone().unwrap_or_default(),
    },
    &mut buff,
  )?;

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

  Ok(())
}

fn main() -> anyhow::Result<()> {
  let args = cli::Cli::parse();

  let diesel_config_content = fs::read_to_string(args.args.diesel_config)?;
  let diesel_gen_config_content =
    fs::read_to_string(args.args.diesel_gen_config)?;

  let diesel_config = toml::from_str::<DieselConfig>(&diesel_config_content)?;
  let diesel_gen_config =
    toml::from_str::<DieselGenConfig>(&diesel_gen_config_content)?;

  match args.command {
    cli::CliSubcommand::Models => {
      generate_models(&diesel_config, &diesel_gen_config)?;
    }
  }

  Ok(())
}
