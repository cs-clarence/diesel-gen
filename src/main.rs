use std::{
  collections::HashMap,
  fs::{self, OpenOptions},
  io::{self, Write},
  process::{Command, Stdio},
  str::FromStr,
};

use clap::{CommandFactory, Parser};
use diesel_gen::{
  config::Config,
  parse::File,
  write::async_graphql::{ModelImportsArgs, OutputTypesArgs},
};
use diesel_gen::{parse::ParseContext, write::TypeUsesArgs};

use diesel_gen::write::ModelsArgs;
use diesel_gen::*;
use merge::Merge;

fn generate_models(config: &Config, parsed_file: &File) -> anyhow::Result<()> {
  let model = config.models.as_ref();

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

    let ref_type_overrides =
      util::remove_spaces_from_keys(&config.ref_type_overrides);

    let type_overrides = util::remove_spaces_from_keys(&config.type_overrides);

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
      config.table_imports_root.clone().unwrap_or_else(|| {
        let path = util::to_rust_path(config.schema.clone().to_str().unwrap());

        if let Some(module) = &parsed_file.module {
          format!("{}::{}", path, module)
        } else {
          path
        }
      });

    write::table_uses(
      &table_imports_root,
      &parsed_file.tables,
      &config.tables,
      &mut buff,
    )?;

    write::type_uses(
      &TypeUsesArgs {
        tables: &parsed_file.tables,
        type_overrides: &type_overrides,
        types_uses: &config.type_uses,
      },
      &mut buff,
    )?;

    write::models(
      &ModelsArgs {
        file: parsed_file,
        backend: &model.backend,
        ref_type_overrides: &ref_type_overrides,
        type_overrides: &type_overrides,
        table_configs: &config.tables,
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
  config: &Config,
  parsed_file: &File,
) -> anyhow::Result<()> {
  let content = fs::read_to_string(config.schema.clone())?;

  let parsed = parse::file(&mut ParseContext::new(&content))?;

  let async_graphql = config.async_graphql.as_ref();

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

    let type_overrides = util::remove_spaces_from_keys(&config.type_overrides);

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
        types_uses: &config.type_uses,
      },
      &mut buff,
    )?;

    let tables = &config.tables;

    let wildcard_table_config = tables.get("*");

    let model_names = parsed_file
      .tables
      .iter()
      .map(|v| {
        let t = tables
          .get(&v.name)
          .or(wildcard_table_config)
          .cloned()
          .unwrap_or_default();

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
        util::to_rust_path(
          &config
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
        output_type_configs: &async_graphql.output_types,
        rename_prefix: Some("Model"),
      },
      &mut buff,
    )?;

    let model_names = model_names
      .iter()
      .map(|(k, v)| (k.clone(), format!("Model{}", v)))
      .collect::<HashMap<String, String>>();

    let type_overrides = util::remove_spaces_from_keys(&config.type_overrides);

    write::async_graphql::output_types(
      &OutputTypesArgs {
        model_names: &model_names,
        output_type_configs: &async_graphql.output_types,
        table_configs: &config.tables,
        tables: &parsed.tables,
        type_overrides: &type_overrides,
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

  match args.command {
    cli::CliSubcommands::Generate => {
      let diesel_gen_config_content = fs::read_to_string(args.args.config)?;

      let mut diesel_gen_config =
        serde_yaml::from_str::<Config>(&diesel_gen_config_content)?;

      if let Some(t) = diesel_gen_config.tables.get("*").cloned() {
        for (k, v) in diesel_gen_config.tables.iter_mut() {
          if k != "*" {
            v.merge(t.clone());
          }
        }
      }

      if let Some(ag) = &mut diesel_gen_config.async_graphql {
        if let Some(t) = ag.output_types.get("*").cloned() {
          for (k, v) in ag.output_types.iter_mut() {
            if k != "*" {
              v.merge(t.clone());
            }
          }
        }
      }

      let content = fs::read_to_string(&diesel_gen_config.schema)?;

      let parsed = parse::file(&mut ParseContext::new(&content))?;
      generate_models(&diesel_gen_config, &parsed)?;
      generate_async_graphql(&diesel_gen_config, &parsed)?;
    }
    cli::CliSubcommands::Config { subcommand } => match subcommand {
      cli::ConfigSubcommands::Schema => {
        let schema = schemars::schema_for!(Config);
        serde_json::to_writer_pretty(io::stdout(), &schema)?;
      }
    },
    cli::CliSubcommands::Completion { shell } => {
      let mut cmd = cli::Cli::command_for_update();
      clap_complete::generate(
        shell,
        &mut cmd,
        build::PROJECT_NAME,
        &mut io::stdout(),
      );
    }
  }

  Ok(())
}
