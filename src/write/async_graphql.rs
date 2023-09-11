use std::{
  collections::{HashMap, HashSet},
  io::Write,
};

use merge::Merge;

use crate::{
  config::{OutputTypeConfig, TableConfig},
  parse::Table,
};

pub struct ModelImportsArgs<'a> {
  pub output_type_configs: &'a Vec<&'a OutputTypeConfig>,
  pub rename_prefix: Option<&'a str>,
  pub model_names: &'a HashMap<String, String>,
  pub model_imports_root: &'a str,
}

pub fn model_imports<W: Write>(
  args: &ModelImportsArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  let prefix = args.rename_prefix.unwrap_or_default();
  let mut uses = HashSet::<String>::new();

  for config in args.output_type_configs.iter() {
    let name = args.model_names.get(&config.table).ok_or_else(|| {
      anyhow::anyhow!("model name for table {} not found", config.table)
    })?;

    if prefix.is_empty() {
      uses.insert(name.clone());
    } else {
      uses.insert(format!(
        "{name} as {prefix}{name}",
        prefix = &prefix,
        name = &name
      ));
    }

    for inherit in config.inherits.iter() {
      let name = args.model_names.get(inherit).ok_or_else(|| {
        anyhow::anyhow!("model name for table {} not found", inherit)
      })?;

      if prefix.is_empty() {
        uses.insert(name.clone());
      } else {
        uses.insert(format!(
          "{name} as {prefix}{name}",
          prefix = &prefix,
          name = &name
        ));
      }
    }
  }

  write!(w, "use {}::{{", args.model_imports_root)?;
  write!(
    w,
    "{}",
    uses
      .iter()
      .map(|e| e.as_str())
      .collect::<Vec<&str>>()
      .join(", ")
  )?;
  write!(w, "}};",)?;

  Ok(())
}

pub struct OutputTypesArgs<'a> {
  pub tables: &'a Vec<Table>,
  pub table_configs: &'a HashMap<String, TableConfig>,
  pub output_type_configs: &'a HashMap<String, OutputTypeConfig>,
  pub model_names: &'a HashMap<String, String>,
}

pub fn output_types<W: Write>(
  args: &OutputTypesArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  let wildcard_table_config =
    args.table_configs.get("*").cloned().unwrap_or_default();

  let wildcard_output_type_config = args
    .output_type_configs
    .get("*")
    .cloned()
    .unwrap_or_default();

  for (name, config) in args.output_type_configs {
    let mut config = config.clone();
    config.merge(wildcard_output_type_config.clone());

    let table = args
      .tables
      .iter()
      .find(|t| t.name == config.table)
      .ok_or_else(|| anyhow::anyhow!("table {} not found", config.table))?;

    let mut table_config = args
      .table_configs
      .get(&config.table)
      .cloned()
      .unwrap_or_default();

    table_config.merge(wildcard_table_config.clone());

    let inherits = args
      .tables
      .iter()
      .filter(|t| config.inherits.contains(&t.name))
      .collect::<Vec<&Table>>();

    output_type(
      &OutputTypeArgs {
        wildcard_table_config: &wildcard_table_config,
        name,
        table,
        table_config: &table_config,
        inherits: &inherits,
        table_configs: args.table_configs,
        impl_from: config.impl_from.unwrap_or(true),
        complex_object: config.complex_object.unwrap_or(false),
        attributes: config.attributes.clone().unwrap_or_default().vec(),
        derives: config.derives.clone().unwrap_or_default().vec(),
        model_names: args.model_names,
      },
      &mut w,
    )?;
  }

  Ok(())
}

pub struct OutputTypeArgs<'a> {
  pub name: &'a str,
  pub table: &'a Table,
  pub model_names: &'a HashMap<String, String>,
  pub impl_from: bool,
  pub wildcard_table_config: &'a TableConfig,
  pub table_config: &'a TableConfig,
  pub attributes: &'a Vec<String>,
  pub derives: &'a Vec<String>,
  pub inherits: &'a Vec<&'a Table>,
  pub table_configs: &'a HashMap<String, TableConfig>,
  pub complex_object: bool,
}

struct WriteFromArgs<'a> {
  pub model_name: &'a str,
  pub inherits: &'a Vec<String>,
}

fn write_from<W: Write>(
  args: &WriteFromArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  if args.inherits.is_empty() {
    writeln!(w, "{}", args.model_name)?;
  } else {
    writeln!(w, "({}, {})", args.model_name, args.inherits.join(", "))?;
  }

  Ok(())
}

pub fn output_type<W: Write>(
  args: &OutputTypeArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  let mod_name = args.model_names.get(&args.table.name).ok_or_else(|| {
    anyhow::anyhow!("model name for table {} not found", args.table.name)
  })?;

  let wfa = WriteFromArgs {
    inherits: &args
      .inherits
      .iter()
      .map(|e| {
        args
          .model_names
          .get(&e.name)
          .expect("model name not found")
          .clone()
      })
      .collect::<Vec<String>>(),
    model_name: mod_name,
  };

  writeln!(w, "#[derive(async_graphql::SimpleObject)]",)?;
  if args.complex_object {
    writeln!(w, "#[graphql(complex)]",)?;
  }

  if !args.derives.is_empty() {
    writeln!(w, "#[derive({})]", args.derives.join(", "))?;
  }

  if !args.attributes.is_empty() {
    for a in args.attributes {
      writeln!(w, "#[{}]", a)?;
    }
  }

  writeln!(w, "pub struct {output_type} {{", output_type = &args.name)?;

  writeln!(w, "}}")?;

  if args.impl_from {
    writeln!(w, "impl From<",)?;
    write_from(&wfa, &mut w)?;
    writeln!(w, "> for {output_type} {{", output_type = &args.name,)?;
    write!(w, "fn from(val: ")?;
    write_from(&wfa, &mut w)?;
    writeln!(w, ") -> Self {{")?;
    writeln!(w, "todo!()")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}",)?;
  }

  Ok(())
}
