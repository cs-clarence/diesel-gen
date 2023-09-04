use std::{
  collections::HashMap, io::Write, path::PathBuf, str::FromStr, sync::OnceLock,
};

use inflector::Inflector;

use crate::{
  config::{DieselConfig, ModelConfig},
  parse::{type_name, File, Type, TypeName},
};

fn print_rust_file_headers<W: Write>(mut writer: W) -> std::io::Result<()> {
  writeln!(writer, "// @generated automatically by diesel-gen\n")?;

  writeln!(writer, "#![allow(unused)]")?;
  writeln!(writer, "#![allow(clippy::all)]\n")?;

  Ok(())
}

fn to_singular_pascal_case(text: &str) -> String {
  text.to_pascal_case().to_singular()
}

fn get_field_name(
  model: &ModelConfig,
  table_name: &str,
  column_name: &str,
) -> String {
  if model.tables.is_none() {
    return column_name.to_string();
  }

  let tables = model.tables.as_ref().unwrap();

  let table = tables.get(table_name);
  let wildcard = tables.get("*");

  if let Some(table) = table {
    if let Some(columns) = table.columns.as_ref() {
      if let Some(column) = columns.get(column_name) {
        return column.rename.as_deref().unwrap_or(column_name).to_string();
      }
    }
  }

  if let Some(table) = wildcard {
    if let Some(columns) = table.columns.as_ref() {
      if let Some(column) = columns.get(column_name) {
        return column.rename.as_deref().unwrap_or(column_name).to_string();
      }
    }
  }

  column_name.to_string()
}

macro_rules! hash_map {
  {$($key:expr => $value:expr),* $(,)?} => {{
    let mut map = HashMap::new();
    $(map.insert($key, $value);)*
    map
  }};
}

static RUST_TYPE_MAP: OnceLock<HashMap<&'static str, &'static str>> =
  OnceLock::new();

fn init_type_map() {
  use type_name::*;
  _ = RUST_TYPE_MAP.set(hash_map! {
   ARRAY => "Vec",
   INT2 => "i16",
   SMALLINT=> "i16",
   INT4 => "i32",
   INTEGER => "i32",
   UNSIGNED => "Unsigned",
   INT8 => "i64",
   BIGINT => "i64",
   NUMERIC => "bigdecimal::BigDecimal",
   DECIMAL => "bigdecimal::BigDecimal",
   TEXT => "String",
   DATE => "time::Date",
   TIME => "time::Time",
   DATETIME => "time::OffsetDateTime",
   TIMESTAMP => "time::OffsetDateTime",
   TIMESTAMPTZ => "time::OffsetDateTime",
   FLOAT4 => "f32",
   FLOAT8 => "f64",
   FLOAT => "f32",
   BOOL => "bool",
   JSON => "serde_json::Value",
   JSONB => "serde_json::Value",
   UUID => "uuid::Uuid",
   CHAR => "char",
   VARCHAR =>"String",
   DOUBLE => "f64",
   TINYINT => "i8",
   NULLABLE => "Option",

  // Unsupported types
   BYTEA => "Bytea",
   BINARY =>"Binary",
   VARBINARY => "Varbinary",
   BLOB => "Blob",
   TINYBLOB => "Tinyblob",
   MEDIUMBLOB => "Mediumblob",
   LONGBLOB => "Longblob",
   BIT => "Bit",
   INET => "Inet",
   TINYTEXT => "Tinytext",
   MEDIUMTEXT => "Mediumtext",
   LONGTEXT => "Longtext",
  });
}

fn get_type(
  type_overrides: &HashMap<String, String>,
  ty: &Type,
) -> Option<String> {
  init_type_map();
  let ts = ty.to_string().replace(' ', "");

  if let Some(ty) = type_overrides.get(&ts) {
    return Some(ty.clone());
  }

  let tp_map = RUST_TYPE_MAP.get().expect("RUST_TYPE_MAP not initialized");

  if let Some(ty) = tp_map.get(ts.as_str()) {
    return Some(ty.to_string());
  }

  if let Some(t) = tp_map.get(&ty.name.to_string().as_str()) {
    let params = ty
      .params
      .iter()
      .map(|i| get_type(type_overrides, i))
      .collect::<Vec<_>>();

    if params.iter().any(|i| i.is_none()) {
      return None;
    }

    let params = params.into_iter().map(|i| i.unwrap()).collect::<Vec<_>>();

    return Some(format!("{}<{}>", t, params.join(", ")));
  }

  None
}

fn get_ref_type(
  type_overrides: &HashMap<String, String>,
  ty: &Type,
  lifetime: &str,
) -> Option<String> {
  init_type_map();
  let ts = ty.to_string().replace(' ', "");

  if let Some(ty) = type_overrides.get(&ts) {
    return Some(ty.clone());
  }

  if ty.name.is_string_type() {
    return Some(format!("&{} str", lifetime));
  }

  if ty.name == TypeName::Nullable {
    let params = ty
      .params
      .iter()
      .map(|i| get_ref_type(type_overrides, i, lifetime))
      .collect::<Vec<_>>();

    if params.iter().any(|i| i.is_none()) {
      return None;
    }

    let params = params.into_iter().map(|i| i.unwrap()).collect::<Vec<_>>();

    return Some(format!("Option<{}>", params.join(", ")));
  }

  let tp_map = RUST_TYPE_MAP.get().expect("RUST_TYPE_MAP not initialized");

  if let Some(ty) = tp_map.get(ts.as_str()) {
    return Some(format!("&{} {}", lifetime, ty));
  }

  if let Some(t) = tp_map.get(&ty.name.to_string().as_str()) {
    let params = ty
      .params
      .iter()
      .map(|i| get_type(type_overrides, i))
      .collect::<Vec<_>>();

    if params.iter().any(|i| i.is_none()) {
      return None;
    }

    let params = params.into_iter().map(|i| i.unwrap()).collect::<Vec<_>>();

    return Some(format!("&{} {}<{}>", lifetime, t, params.join(", ")));
  }

  None
}

pub fn generate_models<W: Write>(
  file: &File,
  config: &DieselConfig,
  model: &ModelConfig,
  mut w: W,
) -> anyhow::Result<()> {
  print_rust_file_headers(&mut w)?;

  let import_root = model.table_import_root.as_ref().cloned().unwrap_or(
    config
      .print_schema
      .as_ref()
      .cloned()
      .unwrap_or_default()
      .file
      .unwrap_or(PathBuf::from_str("./schema.rs").unwrap())
      .to_str()
      .unwrap()
      .trim_start_matches("./")
      .split('/')
      .filter_map(|mut e| {
        if e == "mod.rs" {
          return None;
        }

        if e == "src" {
          return Some("crate");
        }

        if e.ends_with(".rs") {
          e = e.trim_end_matches(".rs");

          return Some(e);
        }

        Some(e)
      })
      .collect::<Vec<&str>>()
      .join("::"),
  );

  if let Some(module) = file.module.as_deref() {
    writeln!(w, "use {}::{}::{{", import_root, module)?;
  } else {
    writeln!(w, "use {}::{{", import_root)?;
  }

  for t in &file.tables {
    writeln!(w, "  {},", &t.name)?;
  }

  writeln!(w, "}};\n")?;

  if let Some(ref imports) = model.imports {
    for i in imports {
      writeln!(w, "use {};", i)?;
    }
  }

  const DIESEL_DEFAULT_DERIVE: &str =
    "#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]";

  const DIESEL_INSERTER_DERIVE: &str = "#[derive(diesel::Insertable)]";

  const DIESEL_UPDATER_DERIVE: &str = "#[derive(diesel::AsChangeset)]";

  let struct_prefix = model
    .struct_names_prefix
    .as_ref()
    .cloned()
    .unwrap_or("".to_string());

  let struct_suffix = model
    .struct_names_suffix
    .as_ref()
    .cloned()
    .unwrap_or("".to_string());

  let inserter_prefix = model
    .inserter_struct_names_prefix
    .as_ref()
    .cloned()
    .unwrap_or("New".to_string());

  let inserter_suffix = model
    .inserter_struct_name_suffix
    .as_ref()
    .cloned()
    .unwrap_or("".to_string());

  let updater_prefix = model
    .updater_structs_name_prefix
    .as_ref()
    .cloned()
    .unwrap_or("".to_string());

  let updater_suffix = model
    .updater_structs_name_suffix
    .as_ref()
    .cloned()
    .unwrap_or("Update".to_string());

  let table_configs = model.tables.as_ref().cloned().unwrap_or_default();

  let wildcard_table_config =
    table_configs.get("*").cloned().unwrap_or_default();

  let wildcard_derives = wildcard_table_config.derives.unwrap_or_default();

  let wildcard_inserter_derives =
    wildcard_table_config.inserter_derives.unwrap_or_default();

  let wildcard_updater_derives =
    wildcard_table_config.updater_derives.unwrap_or_default();

  let wildcard_attributes =
    wildcard_table_config.attributes.unwrap_or_default();

  let wildcard_inserter_attributes = wildcard_table_config
    .inserter_attributes
    .unwrap_or_default();

  let wildcard_updater_attributes =
    wildcard_table_config.updater_attributes.unwrap_or_default();

  // remove spaces from type overrides to make it easier to match
  let type_overrides = model
    .type_overrides
    .as_ref()
    .cloned()
    .unwrap_or_default()
    .iter()
    .map(|(x, y)| {
      (
        x.replace(' ', "").to_string(),
        y.replace(' ', "").to_string(),
      )
    })
    .collect::<HashMap<String, String>>();

  // remove spaces from type overrides to make it easier to match
  let ref_type_overrides = model
    .ref_type_overrides
    .as_ref()
    .cloned()
    .unwrap_or_default()
    .iter()
    .map(|(x, y)| {
      (
        x.replace(' ', "").to_string(),
        y.replace(' ', "").to_string(),
      )
    })
    .collect::<HashMap<String, String>>();

  let optinal_updater_fields = model.updater_fields_optional.unwrap_or(true);

  for t in &file.tables {
    let table_config = table_configs.get(&t.name).cloned().unwrap_or_default();

    let mut d = wildcard_derives.clone();
    d.append(&mut table_config.derives.clone().unwrap_or_default());
    d.dedup();
    if !d.is_empty() {
      writeln!(w, "#[derive({})]", d.join(", "))?;
    }

    writeln!(w, "{}", DIESEL_DEFAULT_DERIVE)?;

    writeln!(w, "#[diesel(table_name = {})]", t.name)?;
    writeln!(w, "#[diesel(primary_key({}))]", t.primary_key.join(", "))?;

    let struct_name = to_singular_pascal_case(&t.name);

    let mut a = wildcard_attributes.clone();
    a.append(&mut table_config.attributes.clone().unwrap_or_default());
    a.dedup();

    if !a.is_empty() {
      for a in a {
        writeln!(w, "#[{}]", a)?;
      }
    }

    writeln!(
      w,
      "pub struct {}{}{} {{",
      &struct_prefix, &struct_name, &struct_suffix
    )?;

    for c in &t.columns {
      let field_name = get_field_name(model, &t.name, &c.name);

      if field_name != c.name {
        writeln!(w, "  #[diesel(column_name = \"{}\")]", c.name)?;
      }
      writeln!(
        w,
        "  pub {}: {},",
        field_name,
        get_type(&type_overrides, &c.r#type).ok_or_else(|| {
          anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
        })?
      )?;
    }

    writeln!(w, "}}\n")?;

    if model.inserter_structs.unwrap_or(true) {
      let mut d = wildcard_inserter_derives.clone();
      d.append(&mut table_config.inserter_derives.clone().unwrap_or_default());
      d.dedup();
      if !d.is_empty() {
        writeln!(w, "#[derive({})]", d.join(", "))?;
      }

      writeln!(w, "{}", DIESEL_INSERTER_DERIVE)?;
      writeln!(w, "#[diesel(table_name = {})]", t.name)?;

      let mut a = wildcard_inserter_attributes.clone();

      a.append(
        &mut table_config.inserter_attributes.clone().unwrap_or_default(),
      );
      a.dedup();

      if !a.is_empty() {
        for a in a {
          writeln!(w, "#[{}]", a)?;
        }
      }

      let lifetime = "'a";
      writeln!(
        w,
        "pub struct {}{}{}<{}>{{",
        &inserter_prefix, &struct_name, &inserter_suffix, lifetime
      )?;
      for c in &t.columns {
        let field_name = get_field_name(model, &t.name, &c.name);
        if field_name != c.name {
          writeln!(w, "  #[diesel(column_name = \"{}\")]", c.name)?;
        }
        writeln!(
          w,
          "  pub {}: {},",
          field_name,
          get_ref_type(&ref_type_overrides, &c.r#type, lifetime).ok_or_else(
            || { anyhow::anyhow!("Unknown type: {}", c.r#type.to_string()) },
          )?
        )?;
      }
      writeln!(w, "}}\n")?;
    }

    let non_primary_key_columns = t.non_primary_key_columns();

    if model.updater_structs.unwrap_or(true)
      && !non_primary_key_columns.is_empty()
    {
      let mut d = wildcard_updater_derives.clone();
      d.append(&mut table_config.updater_derives.clone().unwrap_or_default());
      d.dedup();
      if !d.is_empty() {
        writeln!(w, "#[derive({})]", d.join(", "))?;
      }
      writeln!(w, "{}", DIESEL_UPDATER_DERIVE)?;
      writeln!(w, "#[diesel(table_name = {})]", t.name)?;

      let mut a = wildcard_updater_attributes.clone();

      a.append(
        &mut table_config.updater_attributes.clone().unwrap_or_default(),
      );
      a.dedup();

      if !a.is_empty() {
        for a in a {
          writeln!(w, "#[{}]", a)?;
        }
      }

      let lifetime = "'a";
      writeln!(
        w,
        "pub struct {}{}{}<{}>{{",
        &updater_prefix, &struct_name, &updater_suffix, lifetime
      )?;
      for c in non_primary_key_columns {
        let field_name = get_field_name(model, &t.name, &c.name);

        if field_name != c.name {
          writeln!(w, "  #[diesel(column_name = \"{}\")]", c.name)?;
        }
        let ty = get_ref_type(&ref_type_overrides, &c.r#type, lifetime)
          .ok_or_else(|| {
            anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
          })?;
        writeln!(
          w,
          "  pub {}: {},",
          field_name,
          if optinal_updater_fields {
            format!("Option<{}>", ty)
          } else {
            ty
          }
        )?;
      }
      writeln!(w, "}}\n")?;
    }
  }

  Ok(())
}
