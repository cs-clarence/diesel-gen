use std::{
  collections::HashMap,
  fs::{self, OpenOptions},
  io::Write,
  path::PathBuf,
  process::{Command, Stdio},
  str::FromStr,
};

use clap::Parser;
use diesel_gen::{
  config::{DieselConfig, DieselGenConfig},
  parse::File,
  write::async_graphql::{ModelImportsArgs, OutputTypesArgs},
};
use diesel_gen::{parse::ParseContext, write::TypeUsesArgs};

use diesel_gen::write::ModelsArgs;
use diesel_gen::*;
use merge::Merge;

fn generate_models(
  diesel_config: &DieselConfig,
  diesel_gen_config: &DieselGenConfig,
  parsed_file: &File,
) -> anyhow::Result<()> {
  let model = diesel_gen_config.models.as_ref();

  if let Some(model) = model {
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
      &diesel_gen_config
        .ref_type_overrides
        .clone()
        .unwrap_or_default(),
    );

    let type_overrides = util::remove_spaces_from_keys(
      &diesel_gen_config.type_overrides.clone().unwrap_or_default(),
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

    let table_imports_root = diesel_gen_config
      .table_imports_root
      .clone()
      .unwrap_or_else(|| {
        util::import_root_from_path(
          parsed_file,
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
      &parsed_file.tables,
      &diesel_gen_config.tables.clone().unwrap_or_default(),
      &mut buff,
    )?;

    write::type_uses(
      &TypeUsesArgs {
        tables: &parsed_file.tables,
        type_overrides: &type_overrides,
        types_uses: &diesel_gen_config.type_uses.clone().unwrap_or_default(),
      },
      &mut buff,
    )?;

    write::models(
      &ModelsArgs {
        file: parsed_file,
        backend: &model.backend,
        ref_type_overrides: &ref_type_overrides,
        type_overrides: &type_overrides,
        table_configs: &diesel_gen_config.tables.clone().unwrap_or_default(),
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
    println!("Generated models in {} successfully!", &output_path);
  }

  Ok(())
}

fn generate_async_graphql(
  diesel_config: &DieselConfig,
  diesel_gen_config: &DieselGenConfig,
  parsed_file: &File,
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

  let async_graphql = diesel_gen_config.async_graphql.as_ref();

  if let Some(async_graphql) = async_graphql {
    let output_path = async_graphql
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

    let type_overrides = util::remove_spaces_from_keys(
      &diesel_gen_config.type_overrides.clone().unwrap_or_default(),
    );

    let binding = async_graphql.mods.clone().unwrap_or_default();
    let mods = binding.iter().map(|e| e.as_str()).collect::<Vec<&str>>();

    for m in mods.iter() {
      writeln!(buff, "mod {};", m)?;
    }

    let binding = async_graphql.pub_mods.clone().unwrap_or_default();
    let pub_mods = binding.iter().map(|e| e.as_str()).collect::<Vec<&str>>();

    for m in pub_mods.iter() {
      writeln!(buff, "pub mod {};", m)?;
    }

    let binding = async_graphql.uses.clone().unwrap_or_default();
    let uses = binding.iter().map(|e| e.as_str()).collect::<Vec<&str>>();
    for u in uses.iter() {
      writeln!(buff, "use {};", u)?;
    }

    let binding = async_graphql.pub_uses.clone().unwrap_or_default();
    let pub_uses = binding.iter().map(|e| e.as_str()).collect::<Vec<&str>>();
    for u in pub_uses.iter() {
      writeln!(buff, "pub use {};", u)?;
    }

    write::type_uses(
      &TypeUsesArgs {
        tables: &parsed.tables,
        type_overrides: &type_overrides,
        types_uses: &diesel_gen_config.type_uses.clone().unwrap_or_default(),
      },
      &mut buff,
    )?;

    let tables = diesel_gen_config.tables.clone().unwrap_or_default();

    let wildcard_table_config = tables.get("*").cloned().unwrap_or_default();

    let model_names = parsed_file
      .tables
      .iter()
      .map(|v| {
        let mut t = tables.get(&v.name).cloned().unwrap_or_default();
        t.merge(wildcard_table_config.clone());

        (
          v.name.clone(),
          util::final_name(
            t.model_struct_name_prefix.as_deref(),
            &util::model_name(&v.name),
            t.model_struct_name_suffix.as_deref(),
          ),
        )
      })
      .collect::<HashMap<String, String>>();

    let model_imports_root =
      async_graphql.model_imports_root.clone().unwrap_or_else(|| {
        util::import_root_from_path(
          parsed_file,
          &diesel_gen_config
            .models
            .clone()
            .unwrap_or_default()
            .output
            .unwrap_or_default(),
        )
      });

    write::async_graphql::model_imports(
      &ModelImportsArgs {
        model_imports_root: &model_imports_root,
        model_names: &model_names,
        output_type_configs: &async_graphql.output_types.values().collect(),
        rename_prefix: Some("Model"),
      },
      &mut buff,
    )?;

    let model_names = model_names
      .iter()
      .map(|(k, v)| (k.clone(), format!("Model{}", v)))
      .collect::<HashMap<String, String>>();

    write::async_graphql::output_types(
      &OutputTypesArgs {
        model_names: &model_names,
        output_type_configs: &async_graphql.output_types,
        table_configs: &diesel_gen_config.tables.clone().unwrap_or_default(),
        tables: &parsed.tables,
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
    println!(
      "Generated async-graphql output in {} successfully!",
      &output_path
    );
  }

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

  match args.command {
    cli::CliSubcommand::Generate => {
      generate_models(&diesel_config, &diesel_gen_config, &parsed)?;
      generate_async_graphql(&diesel_config, &diesel_gen_config, &parsed)?;
    }
  }

  Ok(())
}
