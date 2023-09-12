use std::{
  collections::{HashMap, HashSet},
  io::Write,
};

use inflector::Inflector;
use merge::Merge;

use crate::{
  config::{
    ColumnConfig, GraphqlFieldConfig, InheritConfig, OutputTypeConfig,
    TableConfig,
  },
  parse::{Column, Table},
  util::is_rust_keyword,
};

pub struct ModelImportsArgs<'a> {
  pub output_type_configs: &'a HashMap<String, OutputTypeConfig>,
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

  for (name, config) in args.output_type_configs.iter() {
    let name = name.to_snake_case().to_singular();

    let table = config.table.as_ref().unwrap_or(&name);

    let name = args.model_names.get(table).ok_or_else(|| {
      anyhow::anyhow!("model name for table {} not found", table)
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
      let ic = inherit.clone().into_config();
      let name = args.model_names.get(&ic.table).ok_or_else(|| {
        anyhow::anyhow!("model name for table {} not found", &ic.table)
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
  pub type_overrides: &'a HashMap<String, String>,
}

pub fn output_types<W: Write>(
  args: &OutputTypesArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  let wildcard_table_config =
    args.table_configs.get("*").cloned().unwrap_or_default();

  for (name, config) in args.output_type_configs {
    let default_table_name = name.to_snake_case().to_singular();

    let table_name = config.table.as_ref().unwrap_or(&default_table_name);

    let table = args
      .tables
      .iter()
      .find(|t| t.name == *table_name)
      .ok_or_else(|| anyhow::anyhow!("table {} not found", table_name))?;

    let mut table_config = args
      .table_configs
      .get(table_name)
      .cloned()
      .unwrap_or_default();

    table_config.merge(wildcard_table_config.clone());

    let inherits = &config
      .inherits
      .iter()
      .map(|e| e.clone().into_config())
      .collect::<Vec<InheritConfig>>();

    let inherit_names =
      inherits.iter().map(|i| &i.table).collect::<Vec<&String>>();

    let inherits = args
      .tables
      .iter()
      .filter(|t| inherit_names.contains(&&t.name))
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
        tables: args.tables,
        type_overrides: args.type_overrides,
        output_type_config: config,
      },
      &mut w,
    )?;
  }

  Ok(())
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
    writeln!(w, "({}, {})", args.inherits.join(", "), args.model_name)?;
  }

  Ok(())
}

struct GetFieldsArgs<'a> {
  pub table: &'a Table,
  pub inherits: &'a Vec<&'a Table>,
  pub table_configs: &'a HashMap<String, TableConfig>,
  pub tables: &'a Vec<Table>,
  pub output_type_config: &'a OutputTypeConfig,
}

#[derive(Clone, Debug)]
pub struct FieldInfo<'a> {
  pub field_name: String,
  pub model_field_name: String,
  pub column: &'a Column,
  pub field_config: Option<&'a GraphqlFieldConfig>,
  pub index: usize,
  pub column_config: Option<&'a ColumnConfig>,
}

fn get_fields<'a>(
  args: &'a GetFieldsArgs<'a>,
) -> anyhow::Result<Vec<FieldInfo<'a>>> {
  let mut fields = HashMap::<String, FieldInfo>::new();

  let wildcard_table_config = args.table_configs.get("*");
  let otc = args.output_type_config;

  let binding = [args.table];

  let tables = args.inherits.iter().chain(binding.iter());

  let max = tables.clone().count();

  for (index, t) in tables.enumerate() {
    for column in t.columns.iter() {
      if let Some(f) = fields.get(&column.name) {
        if f.column.r#type != column.r#type {
          anyhow::bail!(
          "column {} has different types: {} and {}, rename or omit one of them in the config file",
          column.name,
          f.column.r#type,
          column.r#type
        );
        }
      }

      let table_config =
        args.table_configs.get(&t.name).or(wildcard_table_config);

      let field_configs = if index == max - 1 {
        Some(&otc.fields)
      } else {
        otc.inherits.get(index).and_then(|i| i.fields())
      };

      let field_config = field_configs.and_then(|f| f.get(&column.name));

      let column_config =
        table_config.and_then(|t| t.columns.get(&column.name));

      if let (Some(f), Some(c)) = (field_config, column_config) {
        let model_skip = c.omit_in_model.unwrap_or(false);
        let gql_skip = f.omit.unwrap_or(false);

        if model_skip && !gql_skip {
          anyhow::bail!(
            "column {} is omitted in table {} but not in async_graphql config, omit it in async_graphql or remove the omit from the table config",
            &column.name,
            &t.name
          );
        }

        if gql_skip {
          continue;
        }
      }

      let field_name = field_config
        .and_then(|o| o.rename.as_ref())
        .unwrap_or(&column.name);

      let field_name = if is_rust_keyword(field_name) {
        format!("r#{}", field_name)
      } else {
        field_name.to_string()
      };

      let model_field_name = column_config
        .and_then(|c| c.rename.as_deref())
        .unwrap_or(&column.name);

      let model_field_name = if is_rust_keyword(model_field_name) {
        format!("r#{}", model_field_name)
      } else {
        model_field_name.to_string()
      };

      fields.insert(
        field_name.clone(),
        FieldInfo {
          field_name,
          model_field_name,
          column,
          index,
          field_config,
          column_config,
        },
      );
    }
  }

  let mut fields = fields.values().cloned().collect::<Vec<_>>();

  fields.sort_by_key(|f| f.field_name.clone());

  Ok(fields)
}

pub struct OutputTypeArgs<'a> {
  pub name: &'a str,
  pub table: &'a Table,
  pub tables: &'a Vec<Table>,
  pub model_names: &'a HashMap<String, String>,
  pub impl_from: bool,
  pub wildcard_table_config: &'a TableConfig,
  pub table_config: &'a TableConfig,
  pub attributes: &'a Vec<String>,
  pub derives: &'a Vec<String>,
  pub inherits: &'a Vec<&'a Table>,
  pub table_configs: &'a HashMap<String, TableConfig>,
  pub complex_object: bool,
  pub type_overrides: &'a HashMap<String, String>,
  pub output_type_config: &'a OutputTypeConfig,
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

  let get_field_args = GetFieldsArgs {
    table: args.table,
    inherits: args.inherits,
    table_configs: args.table_configs,
    tables: args.tables,
    output_type_config: args.output_type_config,
  };

  let fields = get_fields(&get_field_args)?;

  writeln!(w, "pub struct {output_type} {{", output_type = &args.name)?;

  for f in fields.iter() {
    let ty = crate::util::get_type(args.type_overrides, &f.column.r#type)
      .ok_or_else(|| {
        anyhow::anyhow!("type for field {} not found", f.column.name)
      })?;

    if let Some(f) = f.field_config {
      if let Some(ref a) = f.attributes {
        for a in a {
          writeln!(w, "#[{}]", a)?;
        }
      }

      if let Some(shareable) = f.shareable {
        if shareable {
          writeln!(w, "#[graphql(shareable)]")?;
        }
      }
    }

    let rust_keyword = is_rust_keyword(&f.field_name);

    if rust_keyword {
      writeln!(w, "#[graphql(rename = \"{}\")]", &f.field_name)?;
    }

    writeln!(
      w,
      "pub {}: {},",
      if rust_keyword {
        format!("r#{}", f.field_name)
      } else {
        f.field_name.clone()
      },
      ty
    )?;
  }

  writeln!(w, "}}")?;

  if args.impl_from {
    writeln!(w, "impl From<",)?;
    write_from(&wfa, &mut w)?;
    writeln!(w, "> for {output_type} {{", output_type = &args.name,)?;
    write!(w, "fn from(val: ")?;
    write_from(&wfa, &mut w)?;
    writeln!(w, ") -> Self {{")?;
    writeln!(w, "Self {{")?;
    for f in fields.iter() {
      if args.inherits.is_empty() {
        writeln!(w, "{}: val.{},", f.field_name, f.model_field_name)?;
      } else {
        writeln!(
          w,
          "{}: val.{}.{},",
          f.field_name, f.index, f.model_field_name
        )?;
      }
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}",)?;
  }

  Ok(())
}
